use snafu::prelude::*;
use std::{collections::BTreeSet, str::FromStr};

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = unique_tail_positions(INPUT)?;
    println!("{part1}");
    assert_eq!(6384, part1);

    Ok(())
}

fn unique_tail_positions(s: &str) -> Result<usize> {
    let mut state = State::default();

    for line in s.lines() {
        let command = line.parse::<Command<u8>>().context(InvalidCommandSnafu)?;
        command.try_repeat(|c| state.move_once(c))?;
    }

    Ok(state.tail_visited())
}

type Coord = (i32, i32);

#[derive(Debug)]
struct State {
    head: Coord,
    tail: Coord,
    tail_visited: BTreeSet<Coord>,
}

impl Default for State {
    fn default() -> Self {
        let mut me = Self {
            head: Default::default(),
            tail: Default::default(),
            tail_visited: Default::default(),
        };
        me.tail_visited.insert(me.tail);
        me
    }
}

impl State {
    fn tail_visited(&self) -> usize {
        self.tail_visited.len()
    }

    fn move_once(&mut self, command: Command<()>) -> Result<()> {
        use Command::*;

        match command {
            Up(()) => self.head.1 += 1,
            Down(()) => self.head.1 -= 1,
            Left(()) => self.head.0 -= 1,
            Right(()) => self.head.0 += 1,
        }

        match self.delta().context(LeftTheBoardSnafu)? {
            #[rustfmt::skip]
            (-1,  1) | ( 0,  1) | ( 1,  1) |
            (-1,  0) | ( 0,  0) | ( 1,  0) |
            (-1, -1) | ( 0, -1) | ( 1, -1) => { /* Close enough */ }

            (dx, dy) => {
                assert!((-2..=2).contains(&dx));
                assert!((-2..=2).contains(&dy));

                self.tail.0 += dx.signum();
                self.tail.1 += dy.signum();
            }
        }

        self.tail_visited.insert(self.tail);
        Ok(())
    }

    fn delta(&self) -> Option<Coord> {
        let Self { head, tail, .. } = *self;
        let x = head.0.checked_sub(tail.0)?;
        let y = head.1.checked_sub(tail.1)?;
        Some((x, y))
    }
}

#[derive(Debug, Copy, Clone)]
enum Command<T> {
    Up(T),
    Down(T),
    Left(T),
    Right(T),
}

impl FromStr for Command<u8> {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let d = parts.next().context(MissingDirectionSnafu)?;
        let c = parts
            .next()
            .context(MissingCountSnafu)?
            .parse()
            .context(InvalidCountSnafu)?;
        Ok(match d {
            "U" => Self::Up(c),
            "D" => Self::Down(c),
            "L" => Self::Left(c),
            "R" => Self::Right(c),
            _ => return UnknownDirectionSnafu { d }.fail(),
        })
    }
}

impl Command<u8> {
    fn try_repeat<E>(self, mut f: impl FnMut(Command<()>) -> Result<(), E>) -> Result<(), E> {
        use Command::*;

        match self {
            Up(c) => (0..c).try_for_each(|_| f(Up(()))),
            Down(c) => (0..c).try_for_each(|_| f(Down(()))),
            Left(c) => (0..c).try_for_each(|_| f(Left(()))),
            Right(c) => (0..c).try_for_each(|_| f(Right(()))),
        }
    }
}

#[derive(Debug, Snafu)]
enum ParseCommandError {
    MissingDirection,

    MissingCount,

    UnknownDirection { d: String },

    InvalidCount { source: std::num::ParseIntError },
}

#[derive(Debug, Snafu)]
enum Error {
    InvalidCommand { source: ParseCommandError },

    LeftTheBoard,
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(13, unique_tail_positions(INPUT)?);
        Ok(())
    }
}
