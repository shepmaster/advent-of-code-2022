use itertools::Itertools;
use snafu::prelude::*;

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = location_of_first_start_of_packet(INPUT)?;
    println!("{part1}");
    assert_eq!(1282, part1);

    let part2 = location_of_first_start_of_message(INPUT)?;
    println!("{part2}");
    assert_eq!(3513, part2);

    Ok(())
}

fn location_of_first_start_of_packet(s: &str) -> Result<usize> {
    common(s, 4)
}

fn location_of_first_start_of_message(s: &str) -> Result<usize> {
    common(s, 14)
}

fn common(s: &str, width: usize) -> Result<usize> {
    s.trim()
        .as_bytes()
        .windows(width)
        .enumerate()
        .find_map(|(i, w)| w.iter().all_unique().then_some(i + width))
        .context(NoStartFoundSnafu)
}

#[derive(Debug, Snafu)]
enum Error {
    NoStartFound,
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &[(&str, usize, usize)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        for (i, &(input, location, _)) in INPUTS.into_iter().enumerate() {
            assert_eq!(
                location,
                location_of_first_start_of_packet(input)?,
                "Test input {i} ({input:?}) failed"
            );
        }
        Ok(())
    }

    #[test]
    #[snafu::report]
    fn example_part2() -> Result<()> {
        for (i, &(input, _, location)) in INPUTS.into_iter().enumerate() {
            assert_eq!(
                location,
                location_of_first_start_of_message(input)?,
                "Test input {i} ({input:?}) failed"
            );
        }
        Ok(())
    }
}
