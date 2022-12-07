use snafu::prelude::*;
use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = sum_of_directories_less_than_100000(INPUT)?;
    println!("{part1}");
    assert_eq!(1491614, part1);

    Ok(())
}

fn sum_of_directories_less_than_100000(s: &str) -> Result<u64> {
    let root = build_directory_hierarchy(s)?;
    let total = root
        .directories()
        .map(|d| d.total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();

    Ok(total)
}

fn build_directory_hierarchy(s: &str) -> Result<Directory<'_>> {
    let mut cursor = Directory::new("/");

    for l in s.lines() {
        let l = Line::try_from(l)?;

        match l {
            Line::ChangeDirectory("/") => cursor = cursor.into_root(),
            Line::ChangeDirectory("..") => cursor = cursor.into_parent(),
            Line::ChangeDirectory(name) => cursor = cursor.into_child(name),
            Line::List => {}
            Line::DirEntry(name) => cursor.add_directory(name),
            Line::FileEntry(name, size) => cursor.add_file(name, size),
        }
    }

    Ok(cursor.into_root())
}

#[derive(Debug, Default)]
struct Directory<'a> {
    name: &'a str,
    parent: Option<Box<Self>>,
    directories: BTreeMap<&'a str, Self>,
    files: BTreeMap<&'a str, u64>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    fn into_root(self) -> Directory<'a> {
        let mut current = self;
        while let Some(mut parent) = current.parent.take() {
            parent.directories.insert(current.name, current);
            current = *parent
        }
        current
    }

    fn into_parent(self) -> Directory<'a> {
        let mut current = self;
        if let Some(mut parent) = current.parent.take() {
            parent.directories.insert(current.name, current);
            current = *parent
        }
        current
    }

    fn into_child(mut self, name: &'a str) -> Directory<'a> {
        let mut child = self
            .directories
            .remove(name)
            .unwrap_or_else(|| Self::new(name));
        child.parent = Some(Box::new(self));
        child
    }

    fn add_directory(&mut self, name: &'a str) {
        self.directories
            .entry(name)
            .or_insert_with(|| Self::new(name));
    }

    fn add_file(&mut self, name: &'a str, size: u64) {
        self.files.insert(name, size);
    }

    fn directories(&self) -> impl Iterator<Item = &'_ Directory<'a>> + '_ {
        let mut state = vec![self];

        std::iter::from_fn(move || {
            let top = state.pop()?;
            state.extend(top.directories.values());
            Some(top)
        })
    }

    fn total_size(&self) -> u64 {
        let files = self.files.values().sum::<u64>();
        let children = self.directories.values().map(Self::total_size).sum::<u64>();

        files + children
    }
}

#[derive(Debug)]
enum Line<'a> {
    ChangeDirectory(&'a str),
    List,
    DirEntry(&'a str),
    FileEntry(&'a str, u64),
}

impl<'a> TryFrom<&'a str> for Line<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        Ok(match parts.next() {
            Some("$") => match parts.next() {
                Some("cd") => {
                    let name = parts.next().context(MissingChangeDirectoryNameSnafu)?;
                    Line::ChangeDirectory(name)
                }
                Some("ls") => Line::List,
                Some(cmd) => return UnknownCommandSnafu { cmd }.fail(),
                None => return MissingCommandSnafu.fail(),
            },
            Some("dir") => {
                let name = parts.next().context(MissingDirectoryEntryNameSnafu)?;
                Line::DirEntry(name)
            }
            Some(size) => {
                let size = size.parse().context(InvalidFileEntrySizeSnafu)?;
                let name = parts.next().context(MissingFileEntryNameSnafu)?;
                Line::FileEntry(name, size)
            }
            None => return MissingOutputSnafu.fail(),
        })
    }
}

#[derive(Debug, Snafu)]
enum Error {
    MissingChangeDirectoryName,

    UnknownCommand { cmd: String },

    MissingCommand,

    MissingDirectoryEntryName,

    InvalidFileEntrySize { source: std::num::ParseIntError },

    MissingFileEntryName,

    MissingOutput,
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(95437, sum_of_directories_less_than_100000(INPUT)?);
        Ok(())
    }
}
