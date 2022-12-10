use itertools::Itertools;
use snafu::prelude::*;
use std::{iter, str::FromStr};

const ADD_X_DURATION: u8 = 2;

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = sum_of_six_signal_strengths(INPUT)?;
    println!("{part1}");
    assert_eq!(15140, part1);

    Ok(())
}

fn sum_of_six_signal_strengths(s: &str) -> Result<i32> {
    let mut instructions = s.lines().map(Instruction::from_str).fuse();

    let mut cycle = 1u16;
    let mut x = 1;
    let mut cached_add_x = None;

    let x_values = iter::from_fn(|| {
        let orig_cycle = cycle;
        let orig_x = x;

        if let Some((mut time, value)) = cached_add_x.take() {
            time -= 1;

            if time == 0 {
                x += value;
            } else {
                cached_add_x = Some((time, value));
            }
        } else {
            let instruction = match instructions.next() {
                Some(Ok(i)) => i,
                Some(Err(e)) => return Some(Err(e)),
                None => Default::default(),
            };

            match instruction {
                Instruction::Noop => {}
                Instruction::AddX(v) => {
                    // Minus 1 because we started the operation and it's been running
                    cached_add_x = Some((ADD_X_DURATION - 1, v));
                }
            }
        }

        cycle += 1;

        Some(Ok((orig_cycle, orig_x)))
    });

    x_values
        .skip(20 - 1) // One based cycle indexing
        .step_by(40)
        .take(6)
        .map_ok(|(i, x)| i32::from(i) * x)
        .sum()
}

#[derive(Debug, Copy, Clone, Default)]
enum Instruction {
    #[default]
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;

        let mut parts = s.split_whitespace();
        Ok(match parts.next().context(InstructionMissingSnafu)? {
            "noop" => Noop,
            "addx" => {
                let value = parts
                    .next()
                    .context(AddXValueMissingSnafu)?
                    .parse()
                    .context(InvalidAddXValueSnafu)?;
                AddX(value)
            }
            instruction => return UnknownInstructionSnafu { instruction }.fail(),
        })
    }
}

#[derive(Debug, Snafu)]
enum Error {
    InstructionMissing,

    UnknownInstruction { instruction: String },

    AddXValueMissing,

    InvalidAddXValue { source: std::num::ParseIntError },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    // const INPUT_TINY: &str = include_str!("../input-tiny.test");

    // #[test]
    // #[snafu::report]
    // fn example_tiny() -> Result<()> {
    //     assert_eq!(13140, sum_of_six_signal_strengths(INPUT_TINY)?);
    //     Ok(())
    // }

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(13140, sum_of_six_signal_strengths(INPUT)?);
        Ok(())
    }
}
