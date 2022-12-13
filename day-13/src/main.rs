use snafu::prelude::*;
use std::{cmp::Ordering, slice, str::FromStr};

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = sum_of_indices_of_pairs_in_right_order(INPUT)?;
    println!("{part1}");
    assert_eq!(5843, part1);

    let part2 = decoder_key(INPUT)?;
    println!("{part2}");
    assert_eq!(26289, part2);

    Ok(())
}

fn sum_of_indices_of_pairs_in_right_order(s: &str) -> Result<usize> {
    Pair::parse_many(s)
        .enumerate()
        .filter_map(|(i, pair)| {
            let i = i + 1; // One-based indexing
            let pair = match pair.context(InvalidPairSnafu) {
                Ok(p) => p,
                Err(e) => return Some(Err(e)),
            };

            pair.is_in_right_order().then_some(Ok(i))
        })
        .sum()
}

fn decoder_key(s: &str) -> Result<usize> {
    let mut packets = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(Packet::from_str)
        .collect::<PacketParseResult<Vec<_>>>()
        .context(InvalidPacketSnafu)?;

    let key_a = Packet::make_key(2);
    let key_b = Packet::make_key(6);

    packets.push(key_a.clone());
    packets.push(key_b.clone());

    packets.sort();

    let a = packets
        .iter()
        .position(|p| p == &key_a)
        .expect("Manually inserted key 'a' went missing");
    let b = packets
        .iter()
        .position(|p| p == &key_b)
        .expect("Manually inserted key 'b' went missing");

    // One-based indexing
    let a = a + 1;
    let b = b + 1;

    Ok(a * b)
}

#[derive(Debug)]
struct Pair(Packet, Packet);

impl Pair {
    fn parse_many(s: &str) -> impl Iterator<Item = PairParseResult<Self>> + '_ {
        let mut lines = s.lines().fuse().peekable();
        std::iter::from_fn(move || {
            lines.peek()?;
            Some(Self::parse_one(&mut lines))
        })
    }

    fn parse_one<'a>(mut i: impl Iterator<Item = &'a str>) -> PairParseResult<Self> {
        use pair_parse_error::*;

        let left = i.next().context(LeftMissingSnafu)?;
        let right = i.next().context(RightMissingSnafu)?;
        i.next(); // Burn the empty line

        let left = left.parse().context(LeftInvalidSnafu)?;
        let right = right.parse().context(RightInvalidSnafu)?;

        Ok(Self(left, right))
    }

    fn is_in_right_order(&self) -> bool {
        self.0.cmp(&self.1) == Ordering::Less
    }
}

#[derive(Debug, Snafu)]
#[snafu(module)]
enum PairParseError {
    LeftMissing,
    RightMissing,
    LeftInvalid { source: PacketParseError },
    RightInvalid { source: PacketParseError },
}

type PairParseResult<T, E = PairParseError> = std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Value(u32),
    List(Vec<Self>),
}

impl FromStr for Packet {
    type Err = PacketParseError;

    fn from_str(s: &str) -> PacketParseResult<Self> {
        use packet_parse_error::*;

        fn walking_parse(s: &str) -> PacketParseResult<(Packet, &str)> {
            use packet_parse_error::*;
            use Packet::*;

            match s.strip_prefix('[') {
                Some(mut s) => {
                    let mut children = vec![];
                    loop {
                        if s.is_empty() || s.starts_with(']') {
                            break;
                        }

                        let (child, tail) = walking_parse(s)?;
                        children.push(child);

                        if let Some(tail) = tail.strip_prefix(',') {
                            s = tail;
                        } else {
                            s = tail;
                            break;
                        }
                    }
                    let s = s.strip_prefix(']').context(ListUnclosedSnafu)?;
                    Ok((List(children), s))
                }
                None => {
                    let idx = s
                        .char_indices()
                        .find_map(|(i, c)| if c.is_ascii_digit() { None } else { Some(i) })
                        .unwrap_or(s.len());
                    let (head, tail) = s.split_at(idx);
                    let value = head.parse().context(ValueInvalidSnafu { s: head })?;
                    Ok((Value(value), tail))
                }
            }
        }

        let (this, tail) = walking_parse(s)?;
        ensure!(tail.is_empty(), TrailingDataSnafu { tail });
        Ok(this)
    }
}

impl Packet {
    fn make_key(v: u32) -> Self {
        use Packet::*;

        List(vec![List(vec![Value(v)])])
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Less == right order
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;

        let left = self;
        let right = other;

        fn list_order(left: &[Packet], right: &[Packet]) -> Ordering {
            let mut left = left.iter();
            let mut right = right.iter();

            loop {
                match (left.next(), right.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (Some(l), Some(r)) => match l.cmp(r) {
                        Ordering::Equal => continue,
                        other => return other,
                    },
                };
            }
        }

        match (left, right) {
            (Value(l), Value(r)) => l.cmp(r),
            (List(l), List(r)) => list_order(l, r),
            (l, List(r)) => {
                let l = slice::from_ref(l);
                list_order(l, r)
            }
            (List(l), r) => {
                let r = slice::from_ref(r);
                list_order(l, r)
            }
        }
    }
}

#[derive(Debug, Snafu)]
#[snafu(module)]
enum PacketParseError {
    ValueInvalid {
        source: std::num::ParseIntError,
        s: String,
    },

    ListUnclosed,

    TrailingData {
        tail: String,
    },
}

type PacketParseResult<T, E = PacketParseError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
enum Error {
    InvalidPair { source: PairParseError },

    InvalidPacket { source: PacketParseError },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(13, sum_of_indices_of_pairs_in_right_order(INPUT)?);
        Ok(())
    }

    #[test]
    #[snafu::report]
    fn example_part2() -> Result<()> {
        assert_eq!(140, decoder_key(INPUT)?);
        Ok(())
    }
}
