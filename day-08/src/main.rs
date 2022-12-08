use itertools::Itertools;
use snafu::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = visible_trees(INPUT)?;
    println!("{part1}");
    assert_eq!(1792, part1);

    Ok(())
}

fn visible_trees(s: &str) -> Result<usize> {
    let forest = parse_forest(s)?;
    let ((min_x, min_y), (max_x, max_y)) = forest_bounds(&forest).context(ForestIsEmptySnafu)?;

    let all_coords = (min_x..=max_x).cartesian_product(min_y..=max_y);

    let visible: BTreeSet<_> = all_coords
        .filter(|&(x, y)| {
            let my_height = forest[&(x, y)];

            let visible_from_top = (min_y..y).rev().all(|y| forest[&(x, y)] < my_height);
            let visible_from_bottom = (y + 1..=max_y).all(|y| forest[&(x, y)] < my_height);
            let visible_from_left = (min_x..x).rev().all(|x| forest[&(x, y)] < my_height);
            let visible_from_right = (x + 1..=max_x).all(|x| forest[&(x, y)] < my_height);

            visible_from_top || visible_from_bottom || visible_from_left || visible_from_right
        })
        .collect();

    Ok(visible.len())
}

type Coord = (usize, usize);
type Height = u32;
type Forest = BTreeMap<Coord, Height>;

fn parse_forest(s: &str) -> Result<Forest> {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let height = c.to_digit(10).context(InvalidHeightSnafu { x, y, c })?;
                Ok(((x, y), height))
            })
        })
        .collect()
}

fn forest_bounds(forest: &Forest) -> Option<(Coord, Coord)> {
    let min = forest.keys().next().copied()?;
    let max = forest.keys().next_back().copied()?;
    Some((min, max))
}

#[derive(Debug, Snafu)]
enum Error {
    InvalidHeight { x: usize, y: usize, c: char },

    ForestIsEmpty,
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(21, visible_trees(INPUT)?);
        Ok(())
    }
}
