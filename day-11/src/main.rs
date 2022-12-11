#![feature(cell_update)]

use itertools::Itertools;
use snafu::prelude::*;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    mem,
    str::FromStr,
};

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = monkey_business(INPUT)?;
    println!("{part1}");
    assert_eq!(88208, part1);

    Ok(())
}

fn monkey_business(s: &str) -> Result<usize> {
    let monkeys = Monkey::parse_barrel(s)
        .map_ok(|m| (RefCell::new(m), Cell::new(0)))
        .collect::<MonkeyParseResult<Vec<_>>>()
        .context(MonkeyInvalidSnafu)?;

    for _round in 0..20 {
        for (monkey, items_inspected) in &monkeys {
            let mut monkey = monkey.borrow_mut();

            let items = mem::take(&mut monkey.starting_items);
            items_inspected.update(|v| v + items.len());

            for item in items {
                let mut worry_level = monkey.apply_operation(item);

                // Always bored?
                worry_level /= 3;

                let target = if monkey.apply_test(worry_level) {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                let mut target = monkeys[target.0].0.borrow_mut();
                target.starting_items.push_back(worry_level);
            }
        }
    }

    Ok(monkeys
        .iter()
        .map(|(_, n_items)| n_items.get())
        .sorted()
        .rev()
        .take(2)
        .product())
}

type MonkeyId = usize;
type WorryLevel = u64;

#[derive(Debug)]
struct Monkey {
    starting_items: VecDeque<WorryLevel>,
    operation: Operation,
    test: DivisibleBy,
    if_true: ThrowTo,
    if_false: ThrowTo,
}

impl Monkey {
    fn parse_barrel(s: &str) -> impl Iterator<Item = MonkeyParseResult<Self>> + '_ {
        let mut lines = s.lines().fuse().peekable();

        std::iter::from_fn(move || {
            lines.peek()?;
            let monkey = match Self::from_lines(&mut lines) {
                Ok(m) => m,
                Err(e) => return Some(Err(e)),
            };
            lines.next(); // Skip the blank line
            Some(Ok(monkey))
        })
    }

    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    fn from_lines<'a>(mut i: impl Iterator<Item = &'a str>) -> MonkeyParseResult<Self> {
        use monkey_parse_error::*;

        let _id = i.next().context(IdMissingSnafu)?;
        let starting_items = i.next().context(StartingItemsMissingSnafu)?;
        let operation = i.next().context(OperationMissingSnafu)?;
        let test = i.next().context(TestMissingSnafu)?;
        let if_true = i.next().context(IfTrueMissingSnafu)?;
        let if_false = i.next().context(IfFalseMissingSnafu)?;

        let starting_items = starting_items
            .trim()
            .strip_prefix("Starting items: ")
            .context(StartingItemsMalformedSnafu)?;
        let starting_items = starting_items
            .split(',')
            .map(|item| {
                let item = item.trim();
                item.parse().context(StartingItemInvalidSnafu { item })
            })
            .collect::<MonkeyParseResult<_>>()?;

        let operation = operation
            .trim()
            .strip_prefix("Operation: ")
            .context(OperationMalformedSnafu)?;
        let operation = operation.parse().context(OperationInvalidSnafu)?;

        let test = test
            .trim()
            .strip_prefix("Test: ")
            .context(TestMalformedSnafu)?;
        let test = test.parse().context(TestInvalidSnafu)?;

        let if_true = if_true
            .trim()
            .strip_prefix("If true: ")
            .context(IfTrueMalformedSnafu)?;
        let if_true = if_true.parse().context(IfTrueInvalidSnafu)?;

        let if_false = if_false
            .trim()
            .strip_prefix("If false: ")
            .context(IfFalseMalformedSnafu)?;
        let if_false = if_false.parse().context(IfFalseInvalidSnafu)?;

        Ok(Self {
            starting_items,
            operation,
            test,
            if_true,
            if_false,
        })
    }

    fn apply_operation(&self, item: WorryLevel) -> WorryLevel {
        use {Op::*, Rhs::*};

        let rhs = match self.operation.rhs {
            Literal(v) => v,
            Old => item,
        };

        match self.operation.op {
            Add => item + rhs,
            Multiply => item * rhs,
        }
    }

    fn apply_test(&self, item: WorryLevel) -> bool {
        item % self.test.0 == 0
    }
}

#[derive(Debug, Snafu)]
#[snafu(module)]
enum MonkeyParseError {
    IdMissing,
    StartingItemsMissing,
    OperationMissing,
    TestMissing,
    IfTrueMissing,
    IfFalseMissing,

    StartingItemsMalformed,
    OperationMalformed,
    TestMalformed,
    IfTrueMalformed,
    IfFalseMalformed,

    StartingItemInvalid {
        source: std::num::ParseIntError,
        item: String,
    },
    OperationInvalid {
        source: OperationParseError,
    },
    TestInvalid {
        source: DivisibleByParseError,
    },
    IfTrueInvalid {
        source: ThrowToParseError,
    },
    IfFalseInvalid {
        source: ThrowToParseError,
    },
}

type MonkeyParseResult<T, E = MonkeyParseError> = std::result::Result<T, E>;

#[derive(Debug, Copy, Clone)]
struct Operation {
    op: Op,
    rhs: Rhs,
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Multiply,
}

#[derive(Debug, Copy, Clone)]
enum Rhs {
    Literal(WorryLevel),
    Old,
}

impl FromStr for Operation {
    type Err = OperationParseError;

    // new = old * 19
    fn from_str(s: &str) -> OperationParseResult<Self> {
        use operation_parse_error::*;

        let (_, operation) = s.split_once('=').context(MalformedSnafu)?;

        let mut operation = operation.split_whitespace();
        let lhs = operation.next().context(LeftHandSideMissingSnafu)?;
        let op = operation.next().context(OperationMissingSnafu)?;
        let rhs = operation.next().context(RightHandSideMissingSnafu)?;

        ensure!(lhs == "old", LeftHandSideUnknownSnafu);
        let op = match op {
            "+" => Op::Add,
            "*" => Op::Multiply,
            _ => return OpUnknownSnafu { op }.fail(),
        };
        let rhs = match rhs {
            "old" => Rhs::Old,
            rhs => rhs
                .parse()
                .ok()
                .map(Rhs::Literal)
                .context(RightHandSideUnknownSnafu { rhs })?,
        };

        Ok(Self { op, rhs })
    }
}

#[derive(Debug, Snafu)]
#[snafu(module)]
enum OperationParseError {
    Malformed,

    LeftHandSideMissing,
    OperationMissing,
    RightHandSideMissing,

    LeftHandSideUnknown,
    OpUnknown { op: String },
    RightHandSideUnknown { rhs: String },
}
type OperationParseResult<T, E = OperationParseError> = std::result::Result<T, E>;

#[derive(Debug, Copy, Clone)]
struct DivisibleBy(WorryLevel);

impl FromStr for DivisibleBy {
    type Err = DivisibleByParseError;

    // divisible by 23
    fn from_str(s: &str) -> DivisibleByParseResult<Self> {
        use divisible_by_parse_error::*;

        let v = s
            .trim()
            .strip_prefix("divisible by ")
            .context(MalformedSnafu)?;
        let v = v.parse().context(ValueInvalidSnafu { v })?;

        Ok(Self(v))
    }
}

#[derive(Debug, Snafu)]
#[snafu(module)]
enum DivisibleByParseError {
    Malformed,

    ValueInvalid {
        source: std::num::ParseIntError,
        v: String,
    },
}

type DivisibleByParseResult<T, E = DivisibleByParseError> = std::result::Result<T, E>;

#[derive(Debug, Copy, Clone)]
struct ThrowTo(MonkeyId);

impl FromStr for ThrowTo {
    type Err = ThrowToParseError;

    // throw to monkey 2
    fn from_str(s: &str) -> ThrowToParseResult<Self> {
        use throw_to_parse_error::*;

        let v = s
            .trim()
            .strip_prefix("throw to monkey ")
            .context(MalformedSnafu)?;
        let v = v.parse().context(ValueInvalidSnafu { v })?;

        Ok(Self(v))
    }
}

#[derive(Debug, Snafu)]
#[snafu(module)]
enum ThrowToParseError {
    Malformed,

    ValueInvalid {
        source: std::num::ParseIntError,
        v: String,
    },
}

type ThrowToParseResult<T, E = ThrowToParseError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
enum Error {
    MonkeyInvalid { source: MonkeyParseError },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(10605, monkey_business(INPUT)?);
        Ok(())
    }
}
