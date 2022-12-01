use itertools::Itertools;
use snafu::prelude::*;
use std::num::ParseIntError;

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = max_calories(INPUT)?;
    println!("{part1}");
    assert_eq!(part1, 70374);
    Ok(())
}

fn max_calories(s: &str) -> Result<u32> {
    let grouped_by_elf = s.lines().group_by(|l| l.is_empty());

    let summed_calories_by_elf = grouped_by_elf
        .into_iter()
        .flat_map(|(is_empty, group)| (!is_empty).then_some(group))
        .map(|group| {
            group
                .map(|l| l.trim().parse::<u32>().context(BadNumberSnafu))
                .sum::<Result<u32>>()
        });

    itertools::process_results(summed_calories_by_elf, |i| i.max().context(NoNumbersSnafu))?
}

#[derive(Debug, Snafu)]
enum Error {
    BadNumber { source: ParseIntError },
    NoNumbers,
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(max_calories(INPUT)?, 24000);
        Ok(())
    }
}
