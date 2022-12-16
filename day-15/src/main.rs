use itertools::Itertools;
use snafu::prelude::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::{RangeInclusive, Sub},
};

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = n_positions_cannot_contain_a_beacon_at_y(INPUT, 2_000_000)?;
    println!("{part1}");
    assert_eq!(5100463, part1);

    let part2 = tuning_frequency_in_square(INPUT, 4_000_000, 4_000_000)?;
    println!("{part2}");
    assert_eq!(11557863040754, part2);

    Ok(())
}

type Dim = i64;
type Coord = (Dim, Dim);
type Map = BTreeMap<Coord, Coord>;

fn n_positions_cannot_contain_a_beacon_at_y(s: &str, interesting_y: Dim) -> Result<usize> {
    let map = parse_map(s).context(MapMalformedSnafu)?;
    let beacons = map.values().copied().collect::<BTreeSet<_>>();

    let no_beacon_locations = areas_of_interest(&map, interesting_y)
        .filter(|c| !beacons.contains(c))
        .collect::<BTreeSet<_>>();

    Ok(no_beacon_locations.len())
}

fn tuning_frequency_in_square(s: &str, x_max: Dim, y_max: Dim) -> Result<Dim> {
    let map = parse_map(s).context(MapMalformedSnafu)?;
    let map = map
        .into_iter()
        .map(|(s, b)| (s, manhattan_distance(s, b)))
        .collect::<BTreeMap<_, _>>();

    // Walk the Y axis, "casting rays" along the X axis
    // We can calculate where we'd exit each sensor's area, speeding up the runtime
    for y in 0..=y_max {
        let mut x = 0;

        'cast: while x < x_max {
            let c = (x, y);

            for (&s, &radius) in &map {
                let my_radius = manhattan_distance(s, c);

                if my_radius > radius {
                    // We are not inside this sensor's range
                    continue;
                }

                // Calculate what X position we'd leave this sensor's area at
                let delta_y = sub_positive(y, s.1);
                let delta_x = sub_positive(delta_y, radius);
                let last_point_in_area = s.0 + delta_x;
                let exit_point = last_point_in_area + 1;

                if exit_point <= x {
                    continue;
                }

                x = exit_point;
                continue 'cast;
            }

            // We checked all the sensors but didn't make it across; this must be our location
            return Ok(4_000_000 * c.0 + c.1);
        }
    }

    unreachable!("The problem could not be solved; algorithm error");
}

fn parse_map(s: &str) -> ParseMapResult<Map> {
    s.lines()
        .map(|l| {
            use parse_map_error::*;

            let l = l
                .strip_prefix("Sensor at x=")
                .context(SensorXNotFoundSnafu)?;
            let (sx, l) = l.split_once(", y=").context(SensorYNotFoundSnafu)?;
            let (sy, l) = l
                .split_once(": closest beacon is at x=")
                .context(BeaconXNotFoundSnafu)?;
            let (bx, by) = l.split_once(", y=").context(BeaconYNotFoundSnafu)?;

            let sx = sx.parse().context(SensorXMalformedSnafu)?;
            let sy = sy.parse().context(SensorYMalformedSnafu)?;
            let bx = bx.parse().context(BeaconXMalformedSnafu)?;
            let by = by.parse().context(BeaconYMalformedSnafu)?;

            Ok(((sx, sy), (bx, by)))
        })
        .collect()
}

#[derive(Debug, Snafu)]
#[snafu(module)]
enum ParseMapError {
    SensorXNotFound,

    SensorYNotFound,

    BeaconXNotFound,

    BeaconYNotFound,

    SensorXMalformed { source: std::num::ParseIntError },

    SensorYMalformed { source: std::num::ParseIntError },

    BeaconXMalformed { source: std::num::ParseIntError },

    BeaconYMalformed { source: std::num::ParseIntError },
}

type ParseMapResult<T, E = ParseMapError> = std::result::Result<T, E>;

fn areas_of_interest(map: &Map, interesting_y: Dim) -> impl Iterator<Item = Coord> + '_ {
    map.iter().flat_map(move |(&s, &b)| {
        let distance = manhattan_distance(s, b);

        let x_min = s.0 - distance;
        let x_max = s.0 + distance;
        let y_min = s.1 - distance;
        let y_max = s.1 + distance;

        let x = x_min..=x_max;
        let y = y_min..=y_max;
        let y = intersect(y, interesting_y..=interesting_y);

        let square = x.cartesian_product(y);
        square
            .map(move |c| {
                let d = manhattan_distance(s, c);
                (c, d)
            })
            .filter_map(move |(c, d)| (d <= distance).then_some(c))
    })
}

fn intersect<T>(a: RangeInclusive<T>, b: RangeInclusive<T>) -> RangeInclusive<T>
where
    T: Ord,
{
    let (ax, ay) = a.into_inner();
    let (bx, by) = b.into_inner();

    let x = T::max(ax, bx);
    let y = T::min(ay, by);

    x..=y
}

fn manhattan_distance(c0: Coord, c1: Coord) -> i64 {
    sub_positive(c0.0, c1.0) + sub_positive(c0.1, c1.1)
}

fn sub_positive<T>(a: T, b: T) -> T
where
    T: Ord + Sub<Output = T>,
{
    let [a, b] = sort_oneshot([a, b]);
    b - a
}

fn sort_oneshot<T, const N: usize>(mut a: [T; N]) -> [T; N]
where
    T: Ord,
{
    a.sort();
    a
}

#[derive(Debug, Snafu)]
enum Error {
    MapMalformed { source: ParseMapError },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        assert_eq!(26, n_positions_cannot_contain_a_beacon_at_y(INPUT, 10)?);
        Ok(())
    }

    #[test]
    #[snafu::report]
    fn example_part2() -> Result<()> {
        assert_eq!(56000011, tuning_frequency_in_square(INPUT, 20, 20)?);
        Ok(())
    }
}
