use snafu::prelude::*;
use std::{
    cell::Cell,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = fewest_steps_to_goal(INPUT)?;
    println!("{part1}");
    assert!(part1 < 875); // Was using a max-heap, not min-heap
    assert!(part1 > 387); // Was not testing that we could step to the end coordinate
    assert_eq!(391, part1);

    let part2 = fewest_steps_from_scenic_start_to_goal(INPUT)?;
    println!("{part2}");
    assert_eq!(386, part2);

    Ok(())
}

fn fewest_steps_to_goal(s: &str) -> Result<usize> {
    let (height_map, start, end) = parse_height_map(s)?;
    let path = find_path(&height_map, start, end).context(NoPathFoundSnafu)?;

    Ok(path.steps())
}

fn fewest_steps_from_scenic_start_to_goal(s: &str) -> Result<usize> {
    let (height_map, _start, end) = parse_height_map(s)?;

    let starts = height_map
        .iter()
        .flat_map(|(&coord, &height)| (height == 0).then_some(coord));

    starts
        .flat_map(|start| find_path(&height_map, start, end))
        .map(|p| p.steps())
        .min()
        .context(NoMinimalPathFoundSnafu)
}

type Coord = (usize, usize);
type HeightMap = BTreeMap<Coord, u8>;

fn parse_height_map(s: &str) -> Result<(HeightMap, Coord, Coord)> {
    let start = Cell::new(None);
    let end = Cell::new(None);

    let height_map = s
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().map({
                let start = &start;
                let end = &end;

                move |(x, c)| {
                    let coord = (x, y);

                    let c = match c {
                        b'S' => {
                            start.set(Some(coord));
                            b'a'
                        }
                        b'E' => {
                            end.set(Some(coord));
                            b'z'
                        }
                        b'a'..=b'z' => c,
                        c => return InvalidDigitSnafu { c }.fail(),
                    };
                    let height = c - b'a';

                    Ok((coord, height))
                }
            })
        })
        .collect::<Result<HeightMap>>()?;

    let start = start.get().context(StartMissingSnafu)?;
    let end = end.get().context(EndMissingSnafu)?;

    Ok((height_map, start, end))
}

#[allow(dead_code)]
fn h(b: u8) -> char {
    (b + b'a') as char
}

fn find_path(height_map: &HeightMap, start: Coord, end: Coord) -> Option<Path> {
    let mut visited = BTreeSet::new();
    let mut paths = BinaryHeap::from([Path::new(start)]);

    while let Some(path) = paths.pop() {
        let current_pos = path.end();
        let current_height = height_map[&current_pos];

        let newly_visited = visited.insert(current_pos);
        if !newly_visited {
            continue;
        }

        for candidate in path.directions_from_end() {
            if visited.contains(&candidate) {
                continue;
            }

            if let Some(&candidate_height) = height_map.get(&candidate) {
                if candidate_height <= (current_height + 1) {
                    let next_path = path.append(candidate);

                    if candidate == end {
                        return Some(next_path);
                    } else {
                        paths.push(next_path);
                    }
                }
            }
        }
    }

    None
}

#[derive(Debug, PartialEq, Eq)]
struct Path(Vec<Coord>);

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.len().cmp(&other.0.len()).reverse()
    }
}

impl Path {
    fn new(start: Coord) -> Self {
        Self(vec![start])
    }

    fn steps(&self) -> usize {
        self.0.len() - 1 // We track each location, not the steps between
    }

    fn end(&self) -> Coord {
        self.0.last().copied().expect("Path was empty")
    }

    fn directions_from_end(&self) -> impl Iterator<Item = Coord> + '_ {
        let current_pos = self.end();
        let (x, y) = current_pos;

        let l = (|| Some((x.checked_sub(1)?, y)))();
        let r = (|| Some((x.checked_add(1)?, y)))();
        let u = (|| Some((x, y.checked_sub(1)?)))();
        let d = (|| Some((x, y.checked_add(1)?)))();

        [l, r, u, d].into_iter().flatten()
    }

    fn append(&self, end: Coord) -> Self {
        let mut path = self.0.clone();
        path.push(end);
        Self(path)
    }
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("{c:?} is not a valid character"))]
    InvalidDigit { c: char },

    #[snafu(display("There was no start point"))]
    StartMissing,

    #[snafu(display("There was no end point"))]
    EndMissing,

    #[snafu(display("No path was found between the start and end points"))]
    NoPathFound,

    #[snafu(display("No path was found between any start point and the end point"))]
    NoMinimalPathFound,
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(31, fewest_steps_to_goal(INPUT)?);
        Ok(())
    }

    #[test]
    #[snafu::report]
    fn example_part2() -> Result<()> {
        assert_eq!(29, fewest_steps_from_scenic_start_to_goal(INPUT)?);
        Ok(())
    }
}
