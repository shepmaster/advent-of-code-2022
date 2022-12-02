use itertools::Itertools;
use snafu::prelude::*;
use std::num::ParseIntError;

const INPUT: &str = include_str!("../input");
const TOP_N: usize = 3;

#[snafu::report]
fn main() -> Result<()> {
    let part1 = max_calories(INPUT)?;
    println!("{part1}");
    assert_eq!(part1, 70374);

    let part2 = max_n_calories(INPUT, TOP_N)?;
    println!("{part2}");
    assert_eq!(part2, 204610);

    Ok(())
}

fn max_calories(s: &str) -> Result<u32> {
    summed_calories_by_elf(s)?
        .into_iter()
        .max()
        .context(NoNumbersSnafu)
}

fn max_n_calories(s: &str, n: usize) -> Result<u32> {
    Ok(summed_calories_by_elf(s)?
        .into_iter()
        .sorted_unstable_by(|l, r| l.cmp(r).reverse())
        .take(n) // What if there wasn't N?
        .sum())
}

// SAD: Returning a `Vec`
fn summed_calories_by_elf(s: &str) -> Result<Vec<u32>> {
    let grouped_by_elf = s.lines().group_by(|l| l.is_empty());

    grouped_by_elf
        .into_iter()
        .flat_map(|(is_empty, group)| (!is_empty).then_some(group))
        .map(|group| {
            group
                .map(|l| l.trim().parse::<u32>().context(BadNumberSnafu))
                .sum::<Result<u32>>()
        })
        .collect()
}

#[derive(Debug, Snafu)]
enum Error {
    BadNumber { source: ParseIntError },
    NoNumbers,
    NotEnoughNumbers,
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

    #[test]
    #[snafu::report]
    fn example_part2() -> Result<()> {
        assert_eq!(max_n_calories(INPUT, TOP_N)?, 45000);
        Ok(())
    }
}
