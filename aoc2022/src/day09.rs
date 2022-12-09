use std::str::FromStr;

use rustc_hash::FxHashSet;
use scan_fmt::scan_fmt;

use aoc2021::{data_str, utils::{AdventError, split_lines}};

use crate::utils::coordinate::Point;

enum Direction {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

impl Direction {
    fn count(&self) -> usize{
        *match self {
            Direction::Left(n) => n,
            Direction::Right(n) => n,
            Direction::Up(n) => n,
            Direction::Down(n) => n,
        }
    }
}

impl FromStr for Direction {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        scan_fmt!(
            s,
            "{} {}",
            char, usize
        )
        .map_err(AdventError::Scan)
        .and_then(|(direction_char, num_steps)| match direction_char {
            'L' => Ok(Direction::Left(num_steps)),
            'R' => Ok(Direction::Right(num_steps)),
            'U' => Ok(Direction::Up(num_steps)),
            'D' => Ok(Direction::Down(num_steps)),
            _ => Err(AdventError::UnexpectedElement { found: s.to_string(), expected: &["L", "R", "U", "D"] })
        })
    }
}

struct PlanckRope {
    knots: Vec<Point>,

    visited_places: FxHashSet<Point>,
}

impl PlanckRope {
    fn new(num_knots: usize) -> PlanckRope {
        let mut visited_places = FxHashSet::default();
        visited_places.insert(Point::new(0, 0));
        PlanckRope{
            knots: vec![Point::new(0, 0); num_knots],

            visited_places,
        }
    }

    fn step(&mut self, dir: &Direction) {
        let offset = match dir {
            Direction::Left(_) => Point::new(-1, 0),
            Direction::Right(_) => Point::new(1, 0),
            Direction::Up(_) => Point::new(0, 1),
            Direction::Down(_) => Point::new(0, -1),
        };

        self.knots[0] += offset;

        for i in 1..self.knots.len() {
            // if the knot is not adjacent to the previous anymore, it needs to move
            if !self.is_adjacent(i) {
                let offset = self.knots[i].octant(&self.knots[i-1]);
                self.knots[i] += offset;

                // record the positions of the last one
                if i == self.knots.len() - 1 {
                    self.visited_places.insert(self.knots[i].clone());
                }
            }
        }
    }

    fn steps(&mut self, dir: &Direction) {
        for _ in 0..dir.count() {
            self.step(dir)
        }
    }

    fn visited_positions(&mut self, directions: &[Direction]) -> usize {
        for dir in directions {
            self.steps(dir)
        }
        self.visited_places.len()
    }

    fn is_adjacent(&self, idx: usize) -> bool {
        assert!(idx > 0);
        assert!(idx < self.knots.len());
        self.knots[idx - 1].distance_l0(&self.knots[idx]) <= 1
    }
}

pub fn run() -> (usize, usize) {

    let input = data_str!("day09");
    let data: Vec<Direction> = parse(input).expect("invalid input");

    (
        PlanckRope::new(2).visited_positions(&data),
        PlanckRope::new(10).visited_positions(&data),
    )
}

fn parse(input: &str) -> Result<Vec<Direction>, AdventError> {
    split_lines(input).iter()
        .map(|line| line.parse())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input1 = r"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        ";

        let data1: Vec<Direction> = parse(input1).expect("invalid input");

        assert_eq!(PlanckRope::new(2).visited_positions(&data1), 13);
        assert_eq!(PlanckRope::new(10).visited_positions(&data1), 1);

        let input2 = r"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        ";

        let data2: Vec<Direction> = parse(input2).expect("invalid input");

        assert_eq!(PlanckRope::new(10).visited_positions(&data2), 36);
    }
}