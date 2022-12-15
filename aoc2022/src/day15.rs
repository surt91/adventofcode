use std::str::FromStr;

use itertools::Itertools;
use scan_fmt::scan_fmt;
use rustc_hash::{FxHashSet, FxHashMap};

use aoc2021::{data_str, utils::{AdventError, split_lines}};

use crate::utils::coordinate::Point;

struct Map {
    sensors: FxHashMap<Point, isize>,
    beacons: FxHashSet<Point>,
}

impl Map {
    fn extreme_beacons_x(&self) -> (isize, isize) {
        let (min_x, max_x) = self.beacons.iter()
            .map(|Point{x, y: _}| *x)
            .minmax()
            .into_option()
            .unwrap();
        let max_d = self.sensors.values()
            .max()
            .unwrap();

        (min_x - max_d, max_x + max_d)
    }

    fn is_exclusion_zone(&self, coord: Point) -> bool {
        self.sensors.iter()
            .any(|(s, &d)| s.distance_l1(&coord) <= d && !self.beacons.contains(&coord))
    }

    fn excluded_sites_at_y(&self, y: isize) -> usize {
        // this will be slow, we should rather collect a range for each sensor
        // and add them
        let (min_x, max_x) = self.extreme_beacons_x();

        (min_x..=max_x).filter(|&x| self.is_exclusion_zone(Point::new(x, y)))
            .count()
    }
}

impl FromStr for Map {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sensors = FxHashMap::default();
        let mut beacons = FxHashSet::default();
        for line in split_lines(s) {
            let (sx, sy, bx, by) = scan_fmt!(
                &line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                isize, isize, isize, isize
            )?;
            let sensor = Point::new(sx, sy);
            let beacon = Point::new(bx, by);
            let distance = sensor.distance_l1(&beacon);
            sensors.insert(sensor, distance);
            beacons.insert(beacon);
        }
        Ok(Map {
            sensors,
            beacons
        })
    }
}

pub fn run() -> (usize, usize) {

    let input = data_str!("day15");
    let map: Map = input.parse().expect("invalid input");

    (
        map.excluded_sites_at_y(2000000),
        0
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
        ";

        let map: Map = input.parse().expect("invalid input");

        assert_eq!(map.excluded_sites_at_y(10), 26);
    }
}