#![feature(get_many_mut)]

use core::{slice::GetManyMutError, str::FromStr};
use itertools::Either;
use snafu::prelude::*;

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = top_crates_9000(INPUT)?;
    println!("{}", String::from_utf8_lossy(&part1));
    assert_eq!(b"QMBMJDFTD"[..], part1);

    let part2 = top_crates_9001(INPUT)?;
    println!("{}", String::from_utf8_lossy(&part2));
    assert_eq!(b"NBTVTJNFJ"[..], part2);

    Ok(())
}

fn top_crates_9000(s: &str) -> Result<Vec<u8>> {
    top_crates_common(s, true)
}

fn top_crates_9001(s: &str) -> Result<Vec<u8>> {
    top_crates_common(s, false)
}

fn top_crates_common(s: &str, reverse: bool) -> Result<Vec<u8>> {
    let mut lines = s.lines();

    let column_lines = lines.by_ref().take_while(|l| !l.is_empty());
    let mut columns = vec![];

    for l in column_lines {
        let row = l.as_bytes().iter().enumerate().filter_map(|(i, &b)| {
            b.is_ascii_uppercase().then(|| {
                // Note: We zero-index the columns, but the input is one-indexed
                let column = (i - 1) / 4;
                (column, b)
            })
        });

        for (column, b) in row {
            let column_count = column + 1;
            if columns.len() < column_count {
                columns.resize_with(column_count, Vec::new);
            }
            columns[column].push(b);
        }
    }

    // Note: We've pushed into the columns from the top-down, so we
    // need to flip once all input is read.
    for c in &mut columns {
        c.reverse();
    }

    for line in lines {
        let Command { count, from, to } = line.parse::<Command>()?;

        let [from, to] = columns
            .get_many_mut([from, to])
            .context(MovingColumnsToSelfSnafu)?;

        let start = from.len() - count;
        let removed = from.drain(start..);

        let removed = if reverse {
            Either::Left(removed.rev())
        } else {
            Either::Right(removed)
        };

        to.extend(removed)
    }

    Ok(columns.iter().flat_map(|c| c.last()).copied().collect())
}

#[derive(Debug)]
struct Command {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().fuse();

        let count = parts
            .nth(1)
            .context(MissingCountSnafu)?
            .parse()
            .context(InvalidCountSnafu)?;
        let mut from = parts
            .nth(1)
            .context(MissingFromSnafu)?
            .parse()
            .context(InvalidFromSnafu)?;
        let mut to = parts
            .nth(1)
            .context(MissingToSnafu)?
            .parse()
            .context(InvalidToSnafu)?;

        // Note: We zero-index the columns, but the input is one-indexed
        from -= 1;
        to -= 1;

        Ok(Command { count, from, to })
    }
}

#[derive(Debug, Snafu)]
enum Error {
    MissingCount,
    InvalidCount { source: std::num::ParseIntError },

    MissingFrom,
    InvalidFrom { source: std::num::ParseIntError },

    MissingTo,
    InvalidTo { source: std::num::ParseIntError },

    MovingColumnsToSelf { source: GetManyMutError<2> },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(&b"CMZ"[..], top_crates_9000(INPUT)?);
        Ok(())
    }

    #[test]
    #[snafu::report]
    fn example_part2() -> Result<()> {
        assert_eq!(&b"MCD"[..], top_crates_9001(INPUT)?);
        Ok(())
    }
}
