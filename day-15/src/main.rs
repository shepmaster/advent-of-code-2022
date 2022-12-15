use itertools::{Either, Itertools};
use snafu::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    let part1 = n_positions_cannot_contain_a_beacon_at_y(INPUT, 2_000_000)?;
    println!("{part1}");
    assert_eq!(5100463, part1);

    Ok(())
}

type Dim = i64;
type Coord = (Dim, Dim);
type Map = BTreeMap<Coord, Coord>;

fn n_positions_cannot_contain_a_beacon_at_y(s: &str, interesting_y: Dim) -> Result<usize> {
    let map = parse_map(s).context(MapMalformedSnafu)?;
    let beacons = map.values().copied().collect::<BTreeSet<_>>();

    let no_beacon_locations = areas_of_interest(&map, interesting_y)
        .filter_ok(|c| !beacons.contains(c))
        .collect::<Result<BTreeSet<_>>>()?;

    Ok(no_beacon_locations.len())
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

fn areas_of_interest(map: &Map, interesting_y: Dim) -> impl Iterator<Item = Result<Coord>> + '_ {
    map.iter().flat_map(move |(&s, &b)| {
        let distance = match manhattan_distance(s, b).context(BeaconDistanceTooLargeSnafu) {
            Ok(d) => d,
            Err(e) => return Either::Left(std::iter::once(Err(e))),
        };

        let x_min = s.0 - distance;
        let x_max = s.0 + distance;
        let y_min = s.1 - distance;
        let y_max = s.1 + distance;

        let x = x_min..=x_max;

        if !(y_min..=y_max).contains(&interesting_y) {
            return Either::Right(Either::Left(std::iter::empty()));
        }

        let y = interesting_y..=interesting_y;

        let square = x.cartesian_product(y);
        let circle = square
            .map(move |c| {
                let d = manhattan_distance(s, c).context(InterestDistanceTooLargeSnafu)?;
                Ok((c, d))
            })
            .filter_map_ok(move |(c, d)| (d <= distance).then_some(c));

        Either::Right(Either::Right(circle))
    })
}

fn manhattan_distance(c0: Coord, c1: Coord) -> ManhattanDistanceResult<i64> {
    let distance = Dim::abs_diff(c0.0, c1.0) + Dim::abs_diff(c0.1, c1.1);
    Dim::try_from(distance).context(ManhattanDistanceSnafu)
}

#[derive(Debug, Snafu)]
struct ManhattanDistanceError {
    source: std::num::TryFromIntError,
}

type ManhattanDistanceResult<T, E = ManhattanDistanceError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
enum Error {
    MapMalformed { source: ParseMapError },

    BeaconDistanceTooLarge { source: ManhattanDistanceError },

    InterestDistanceTooLarge { source: ManhattanDistanceError },
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
}
