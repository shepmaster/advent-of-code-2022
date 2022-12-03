use snafu::prelude::*;
use std::collections::BTreeSet;

const INPUT: &str = include_str!("../input");

type Priority = u32;

#[snafu::report]
fn main() -> Result<()> {
    let part1 = sum_of_duplicated_priorities(INPUT)?;
    println!("{part1}");
    assert_eq!(7878, part1);

    Ok(())
}

fn sum_of_duplicated_priorities(s: &str) -> Result<Priority> {
    s.lines()
        .map(|l| {
            let l = l.trim();
            // String contains only ASCII
            let l = l.as_bytes();

            ensure!(l.len() % 2 == 0, NonEvenLengthSnafu);
            let half_len = l.len() / 2;
            let (front, back) = l.split_at(half_len);
            let [front, back] = [front, back].map(Contents::try_from);
            let front = front.context(FrontInvalidContentSnafu)?;
            let back = back.context(BackInvalidContentSnafu)?;
            let common = Contents::intersect(&front, &back)?;
            Ok(u32::from(common))
        })
        .sum()
}

struct Contents(BTreeSet<u8>);

impl Contents {
    fn intersect(&self, other: &Self) -> Result<u8> {
        self.0
            .intersection(&other.0)
            .next()
            .copied()
            .context(NoIntersectionSnafu)
    }
}

#[derive(Debug, Snafu)]
struct InvalidContentError {
    v: char,
}

impl TryFrom<&[u8]> for Contents {
    type Error = InvalidContentError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|&v| {
                Ok(match v {
                    b'a'..=b'z' => v - b'a' + 1,
                    b'A'..=b'Z' => v - b'A' + 1 + 26,
                    _ => return InvalidContentSnafu { v }.fail(),
                })
            })
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

#[derive(Debug, Snafu)]
enum Error {
    NonEvenLength,

    FrontInvalidContent { source: InvalidContentError },

    BackInvalidContent { source: InvalidContentError },

    NoIntersection,
}

type Result<T, E = Error> = ::std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    fn exercise() -> Result<()> {
        assert_eq!(157, sum_of_duplicated_priorities(INPUT)?);
        Ok(())
    }
}
