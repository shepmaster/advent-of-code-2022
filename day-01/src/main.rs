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
    itertools::process_results(summed_calories_by_elf(s), |i| {
        i.max().context(NoNumbersSnafu)
    })?
}

fn max_n_calories(s: &str, n: usize) -> Result<u32> {
    itertools::process_results(summed_calories_by_elf(s), |i| {
        i.sorted_unstable_by(|l, r| l.cmp(r).reverse())
            .take(n) // What if there wasn't N?
            .sum()
    })
}

fn summed_calories_by_elf(s: &str) -> impl Iterator<Item = Result<u32>> + '_ {
    let mut lines = s.lines();

    std::iter::from_fn(move || {
        let mut sum = None;

        while let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }

            let v = match l.parse::<u32>().context(BadNumberSnafu) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };

            *sum.get_or_insert(0) += v;
        }

        sum.map(Ok)
    })
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

    #[test]
    #[snafu::report]
    fn example_part2() -> Result<()> {
        assert_eq!(max_n_calories(INPUT, TOP_N)?, 45000);
        Ok(())
    }
}
