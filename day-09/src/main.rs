#![feature(get_many_mut)]

use image::{codecs::gif::GifEncoder, Delay, Frame, RgbaImage};
use snafu::prelude::*;
use std::{collections::BTreeSet, fs::File, io::BufWriter, str::FromStr, time::Duration};

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = unique_tail_positions::<2>(INPUT, |_| {})?;
    println!("{part1}");
    assert_eq!(6384, part1);

    let f = File::create("/tmp/rope.gif").unwrap();
    let f = BufWriter::new(f);
    let mut f = GifEncoder::new(f);

    // -119 -> 185
    const W: u32 = 400;
    // -280 ->  67
    const H: u32 = 600;

    // let mut x_min = 0;
    // let mut x_max = 0;
    // let mut y_min = 0;
    // let mut y_max = 0;

    let part2 = unique_tail_positions::<10>(INPUT, |knots| {
        let mut image = RgbaImage::new(W, H);
        for (x, y, px) in image.enumerate_pixels_mut() {
            let x = i32::try_from(x).unwrap() - i32::try_from(W / 2).unwrap();
            let y = i32::try_from(y).unwrap() - i32::try_from(H / 2).unwrap();

            let light_up = knots.iter().any(|&c| c == (x, y));
            px.0 = if light_up {
                [0, 255, 0, 255]
            } else {
                [127, 0, 127, 255]
            };
        }

        // for &(x, y) in knots {
        //     use std::cmp;

        //     x_min = cmp::min(x_min, x);
        //     x_max = cmp::max(x_max, x);
        //     y_min = cmp::min(y_min, y);
        //     y_max = cmp::max(y_max, y);
        // }

        let frame = Frame::from_parts(
            image,
            0,
            0,
            Delay::from_saturating_duration(Duration::from_millis(50)),
        );
        f.encode_frame(frame).unwrap();
    })?;
    println!("{part2}");
    assert_eq!(2734, part2);

    // dbg!(x_min, x_max, y_min, y_max);

    Ok(())
}

fn unique_tail_positions<const N: usize>(s: &str, mut f: impl FnMut(&[Coord; N])) -> Result<usize> {
    let mut state = State::<N>::default();

    for line in s.lines() {
        let command = line.parse::<Command<u8>>().context(InvalidCommandSnafu)?;
        command.try_repeat(|c| state.move_once(c))?;
        f(&state.knots);
    }

    Ok(state.tail_visited())
}

type Coord = (i32, i32);

#[derive(Debug)]
struct State<const N: usize> {
    knots: [Coord; N],
    tail_visited: BTreeSet<Coord>,
}

impl<const N: usize> Default for State<N> {
    fn default() -> Self {
        let mut me = Self {
            knots: [(0, 0); N],
            tail_visited: Default::default(),
        };
        me.tail_visited.extend(me.knots.last().copied());
        me
    }
}

impl<const N: usize> State<N> {
    fn tail_visited(&self) -> usize {
        self.tail_visited.len()
    }

    fn move_once(&mut self, command: Command<()>) -> Result<()> {
        use Command::*;

        let indices = 0..N;
        let head_indices = indices.clone();
        let tail_indices = indices.skip(1);
        let head_tail_indices = head_indices.zip(tail_indices);

        if let Some(head) = self.knots.first_mut() {
            match command {
                Up(()) => head.1 += 1,
                Down(()) => head.1 -= 1,
                Left(()) => head.0 -= 1,
                Right(()) => head.0 += 1,
            }
        }

        for (head_index, tail_index) in head_tail_indices {
            let [head, tail] = self
                .knots
                .get_many_mut([head_index, tail_index])
                .expect("Created overlapping indices");

            match delta(*head, *tail).context(LeftTheBoardSnafu)? {
                #[rustfmt::skip]
                (-1,  1) | ( 0,  1) | ( 1,  1) |
                (-1,  0) | ( 0,  0) | ( 1,  0) |
                (-1, -1) | ( 0, -1) | ( 1, -1) => { /* Close enough */ }

                (dx, dy) => {
                    assert!((-2..=2).contains(&dx));
                    assert!((-2..=2).contains(&dy));

                    tail.0 += dx.signum();
                    tail.1 += dy.signum();
                }
            }
        }

        self.tail_visited.extend(self.knots.last().copied());
        Ok(())
    }
}

fn delta(head: Coord, tail: Coord) -> Option<Coord> {
    let x = head.0.checked_sub(tail.0)?;
    let y = head.1.checked_sub(tail.1)?;
    Some((x, y))
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
    const INPUT2: &str = include_str!("../input2.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(13, unique_tail_positions::<2>(INPUT, |_| {})?);
        Ok(())
    }

    #[test]
    #[snafu::report]
    fn example_part2() -> Result<()> {
        assert_eq!(1, unique_tail_positions::<10>(INPUT, |_| {})?);
        Ok(())
    }

    #[test]
    #[snafu::report]
    fn example_part2_input2() -> Result<()> {
        assert_eq!(36, unique_tail_positions::<10>(INPUT2, |_| {})?);
        Ok(())
    }
}
