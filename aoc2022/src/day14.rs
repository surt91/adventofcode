use std::{str::FromStr, cmp::{min, max}, fmt::Debug};

use aoc2021::{data_str, utils::{AdventError, split_lines}};
use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::utils::coordinate::Point;

#[derive(Debug, Clone)]
struct Rocks {
    map: FxHashSet<Point>,
    source: Point,
    height: isize,
}

impl Rocks {
    fn drop_sand(&mut self) -> bool {
        let mut pos = self.source.clone();
        let down = Point::new(0, 1);
        let down_left = Point::new(-1, 1);
        let down_right = Point::new(1, 1);
        'outer: loop {
            // we can end as soon as sand is falling into the abyss or blocks the source
            if pos.y >= self.height || self.map.contains(&self.source) {
                return false;
            }

            for &candiate in &[&down, &down_left, &down_right] {
                let new_pos = candiate + &pos;
                if !self.map.contains(&new_pos) {
                    pos = new_pos;
                    continue 'outer;
                }
            }

            self.map.insert(pos);
            return true;
        }
    }

    fn with_floor(mut self) -> Self {
        self.height += 2;

        let left_infty = self.source.x - self.height;
        let right_infty = self.source.x + self.height;

        for x in left_infty..=right_infty {
            let y = self.height;
            self.map.insert(Point::new(x, y));
        }

        self
    }

    fn count_sand(mut self) -> usize {
        let mut ctr = 0;
        while self.drop_sand() {
            ctr += 1
        }
        ctr
    }
}

impl FromStr for Rocks {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = Point::new(500, 0);
        let mut map = FxHashSet::default();
        let lines = split_lines(s);

        for line in lines {
            let corners = line.split(" -> ")
                .map(|c|
                    c.split(',')
                        .map(|i| i.parse().unwrap())
                        .collect_tuple()
                        .ok_or(AdventError::WrongNumberOfElements)
                )
                .collect::<Result<Vec<(isize, isize)>, _>>()?;

            for &[(x1, y1), (x2, y2)] in corners.array_windows() {
                assert!(y1 == y2 || x1 == x2);

                for x in min(x1, x2)..=max(x1, x2) {
                    let y = y1;
                    map.insert(Point::new(x, y));
                }
                for y in min(y1, y2)..=max(y1, y2) {
                    let x = x1;
                    map.insert(Point::new(x, y));
                }
            }
        }

        let &height = map.iter()
            .map(|Point {x: _, y}| y)
            .max()
            .ok_or(AdventError::NotEnoughElements)?;

        Ok(Rocks {
            map,
            source,
            height,
        })
    }
}



pub fn run() -> (usize, usize) {

    let input = data_str!("day14");
    let rocks: Rocks = input.parse().expect("invalid input");
    let rocks_with_floor = rocks.clone().with_floor();

    (
        rocks.count_sand(),
        rocks_with_floor.count_sand(),
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
        ";

        let rocks: Rocks = input.parse().expect("invalid input");
        let rocks_with_floor = rocks.clone().with_floor();

        assert_eq!(rocks.count_sand(), 24);

        assert_eq!(rocks_with_floor.count_sand(), 93);
    }
}