use itertools::Itertools;
use snafu::prelude::*;
use std::{collections::BTreeMap, ops::ControlFlow};

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = units_of_sand_come_to_rest(INPUT)?;
    println!("{part1}");
    assert_eq!(1406, part1);

    Ok(())
}

fn units_of_sand_come_to_rest(s: &str) -> Result<usize> {
    let mut map = parse_map(s).context(MapInvalidSnafu)?;
    let max_y = map
        .keys()
        .map(|&(_, y)| y)
        .max()
        .context(MapNoMaxValueSnafu)?;

    while let ControlFlow::Continue(()) = drop_one_sand(&mut map, max_y) {}

    let n_sand = map.values().filter(|s| matches!(s, State::Sand)).count();

    Ok(n_sand)
}

type Dim = u32;
type Coord = (Dim, Dim);
type Map = BTreeMap<Coord, State>;

#[derive(Debug, Copy, Clone)]
enum State {
    Wall,
    Sand,
}

fn drop_one_sand(map: &mut Map, max_y: Dim) -> ControlFlow<()> {
    let mut sand = (500, 0);

    loop {
        let d = (sand.0, sand.1 + 1);
        let dl = (d.0 - 1, d.1);
        let dr = (d.0 + 1, d.1);

        let candidates = [d, dl, dr];
        match candidates.into_iter().find(|c| !map.contains_key(c)) {
            Some(next) => sand = next,
            None => {
                map.insert(sand, State::Sand);
                return ControlFlow::Continue(());
            }
        }

        if sand.1 > max_y {
            return ControlFlow::Break(());
        }
    }
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
}
