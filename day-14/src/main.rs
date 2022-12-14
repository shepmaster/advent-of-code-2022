use itertools::Itertools;
use snafu::prelude::*;
use std::{collections::BTreeMap, ops::ControlFlow};

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = units_of_sand_come_to_rest(INPUT)?;
    println!("{part1}");
    assert_eq!(1406, part1);

    let part2 = units_of_sand_come_to_rest_infinite_floor(INPUT)?;
    println!("{part2}");
    assert_eq!(20870, part2);

    Ok(())
}

fn units_of_sand_come_to_rest(s: &str) -> Result<usize> {
    let (mut map, max_y) = parse_map_and_max_y(s)?;

    while let ControlFlow::Continue(()) = drop_one_sand(&mut map, |_| false, |sand| sand.1 > max_y)
    {
    }

    Ok(n_sand(&map))
}

fn units_of_sand_come_to_rest_infinite_floor(s: &str) -> Result<usize> {
    let (mut map, max_y) = parse_map_and_max_y(s)?;

    while let ControlFlow::Continue(()) = drop_one_sand(
        &mut map,
        |coord| coord.1 >= max_y + 2,
        |sand| sand == ORIGIN_POINT,
    ) {}

    Ok(n_sand(&map))
}

const ORIGIN_POINT: (u32, u32) = (500, 0);

type Dim = u32;
type Coord = (Dim, Dim);
type Map = BTreeMap<Coord, State>;

#[derive(Debug, Copy, Clone)]
enum State {
    Wall,
    Sand,
}

fn drop_one_sand(
    map: &mut Map,
    mut wall_predicate: impl FnMut(Coord) -> bool,
    mut complete_predicate: impl FnMut(Coord) -> bool,
) -> ControlFlow<()> {
    let mut sand = ORIGIN_POINT;

    loop {
        let d = (sand.0, sand.1 + 1);
        let dl = (d.0 - 1, d.1);
        let dr = (d.0 + 1, d.1);

        let candidates = [d, dl, dr];
        match candidates
            .into_iter()
            .find(|c| !(map.contains_key(c) || wall_predicate(*c)))
        {
            Some(next) => sand = next,
            None => {
                let old_state = map.insert(sand, State::Sand);

                return if old_state.is_some() {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                };
            }
        }

        if complete_predicate(sand) {
            return ControlFlow::Break(());
        }
    }
}

fn parse_map_and_max_y(s: &str) -> Result<(Map, Dim)> {
    let map = parse_map(s).context(MapInvalidSnafu)?;
    let max_y = map
        .keys()
        .map(|&(_, y)| y)
        .max()
        .context(MapNoMaxValueSnafu)?;
    Ok((map, max_y))
}

fn parse_map(s: &str) -> ParseMapResult<Map> {
    use parse_map_error::*;

    let mut map = BTreeMap::new();

    for l in s.lines() {
        let windows = l
            .split(" -> ")
            .map(|c| {
                let (x, y) = c.split_once(',').context(CoordinateMalformedSnafu)?;
                let x = x.parse().context(XInvalidSnafu)?;
                let y = y.parse().context(YInvalidSnafu)?;
                let coord: Coord = (x, y);
                Ok(coord)
            })
            .tuple_windows();

        for window in windows {
            let (start, end) = window;
            let start = start?;
            let end = end?;
            let [x0, x1] = sort_oneshot([start.0, end.0]);
            let [y0, y1] = sort_oneshot([start.1, end.1]);
            let x = x0..=x1;
            let y = y0..=y1;
            let line = x.cartesian_product(y);

            map.extend(line.map(|c| (c, State::Wall)));
        }
    }

    Ok(map)
}

fn sort_oneshot<T, const N: usize>(mut a: [T; N]) -> [T; N]
where
    T: Ord,
{
    a.sort();
    a
}

fn n_sand(map: &Map) -> usize {
    map.values().filter(|s| matches!(s, State::Sand)).count()
}

#[derive(Debug, Clone, Snafu)]
#[snafu(module)]
enum ParseMapError {
    CoordinateMalformed,
    XInvalid { source: std::num::ParseIntError },
    YInvalid { source: std::num::ParseIntError },
}

type ParseMapResult<T, E = ParseMapError> = std::result::Result<T, E>;

#[derive(Debug, Clone, Snafu)]
enum Error {
    MapInvalid { source: ParseMapError },

    MapNoMaxValue,
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(24, units_of_sand_come_to_rest(INPUT)?);
        Ok(())
    }

    #[test]
    #[snafu::report]
    fn example_part2() -> Result<()> {
        assert_eq!(93, units_of_sand_come_to_rest_infinite_floor(INPUT)?);
        Ok(())
    }
}
