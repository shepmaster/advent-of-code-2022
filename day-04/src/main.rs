#![feature(array_try_map)]

use itertools::Itertools;
use snafu::prelude::*;
use std::{num::ParseIntError, ops::RangeInclusive};

const INPUT: &str = include_str!("../input");

type Id = u32;

#[snafu::report]
fn main() -> Result<()> {
    let part1 = count_of_fully_contained_pairs(INPUT)?;
    println!("{part1}");
    assert_eq!(444, part1);

    Ok(())
}

fn count_of_fully_contained_pairs(s: &str) -> Result<usize> {
    let i = s
        .lines()
        .map(|l| {
            let (e1, e2) = l.split_once(',').context(MissingPairSnafu)?;
            let [e1, e2] =
                [e1, e2].try_map(|e| e.split_once('-').context(MissingRangePartSnafu))?;
            [e1, e2].try_map(|(s, e)| {
                let [s, e] =
                    [s, e].try_map(|id| id.parse::<Id>().context(InvalidIdSnafu { id }))?;
                Ok(s..=e)
            })
        })
        .filter_ok(|[e1, e2]| either_fully_contains(e1, e2));

    itertools::process_results(i, |i| i.count())
}

fn either_fully_contains<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: Ord,
{
    fully_contains(a, b) || fully_contains(b, a)
}

fn fully_contains<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: Ord,
{
    a.start() >= b.start() && a.end() <= b.end()
}

#[derive(Debug, Snafu)]
enum Error {
    MissingPair,

    MissingRangePart,

    InvalidId { source: ParseIntError, id: String },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(2, count_of_fully_contained_pairs(INPUT)?);
        Ok(())
    }
}
