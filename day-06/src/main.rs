use itertools::Itertools;
use snafu::prelude::*;

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = location_of_first_start_of_packet(INPUT)?;
    println!("{part1}");
    assert_eq!(1282, part1);

    Ok(())
}

fn location_of_first_start_of_packet(s: &str) -> Result<usize> {
    s.trim()
        .as_bytes()
        .windows(4)
        .enumerate()
        .find_map(|(i, w)| w.iter().all_unique().then_some(i + 4))
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

    const INPUTS: &[(&str, usize)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        for (i, &(input, location)) in INPUTS.into_iter().enumerate() {
            assert_eq!(
                location,
                location_of_first_start_of_packet(input)?,
                "Test input {i} ({input:?}) failed"
            );
        }
        Ok(())
    }
}
