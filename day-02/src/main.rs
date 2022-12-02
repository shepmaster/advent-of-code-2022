use snafu::prelude::*;
use std::str::FromStr;

const INPUT: &str = include_str!("../input");

type Score = u32;

#[snafu::report]
fn main() -> Result<()> {
    let part1 = total_score(INPUT)?;
    println!("{part1}");
    assert_eq!(9241, part1);

    Ok(())
}

fn total_score(s: &str) -> Result<Score> {
    let hands = s.lines().map(|l| {
        let mut l = l.splitn(2, ' ').fuse();
        let them = l.next().context(ThemMissingSnafu)?.parse::<Them>()?;
        let us = l.next().context(UsMissingSnafu)?.parse::<Us>()?;
        <Result<_>>::Ok((them, us))
    });

    itertools::process_results(hands, |hands| {
        hands
            .map(|(them, us)| play(them, us) + us.score())
            .sum::<Score>()
    })
}

const WIN: Score = 6;
const DRAW: Score = 3;
const LOSE: Score = 0;

fn play(them: Them, us: Us) -> Score {
    use {Them as T, Us as U};
    match (them, us) {
        (T::Rock, U::Paper) | (T::Paper, U::Scissors) | (T::Scissors, U::Rock) => WIN,

        (T::Rock, U::Rock) | (T::Paper, U::Paper) | (T::Scissors, U::Scissors) => DRAW,

        (T::Rock, U::Scissors) | (T::Paper, U::Rock) | (T::Scissors, U::Paper) => LOSE,
    }
}

#[derive(Debug, Copy, Clone)]
enum Them {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Snafu)]
struct UnknownThemError {
    s: String,
}

impl FromStr for Them {
    type Err = UnknownThemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Them::*;

        Ok(match s {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => return UnknownThemSnafu { s }.fail(),
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Us {
    Rock,
    Paper,
    Scissors,
}

impl Us {
    fn score(&self) -> Score {
        use Us::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Debug, Snafu)]
struct UnknownUsError {
    s: String,
}

impl FromStr for Us {
    type Err = UnknownUsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Us::*;

        Ok(match s {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => return UnknownUsSnafu { s }.fail(),
        })
    }
}

#[derive(Debug, Snafu)]
enum Error {
    ThemMissing,

    UsMissing,

    #[snafu(context(false))]
    BadThem {
        source: UnknownThemError,
    },

    #[snafu(context(false))]
    BadUs {
        source: UnknownUsError,
    },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(15, total_score(INPUT)?);
        Ok(())
    }
}
