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
    head_position: Point,
    tail_position: Point,

    visited_places: FxHashSet<Point>,
}

impl PlanckRope {
    fn new() -> PlanckRope {
        let mut visited_places = FxHashSet::default();
        visited_places.insert(Point::new(0, 0));

        PlanckRope{
            head_position: Point::new(0, 0),
            tail_position: Point::new(0, 0),

            visited_places,
        }
    }

    fn step(&mut self, dir: &Direction) {
        let old_position = self.head_position.clone();
        match dir {
            Direction::Left(_) => self.head_position += Point::new(-1, 0),
            Direction::Right(_) => self.head_position += Point::new(1, 0),
            Direction::Up(_) => self.head_position += Point::new(0, 1),
            Direction::Down(_) => self.head_position += Point::new(0, -1),
        }
        // if the tail is not adjacent to the head anymore, it needs to move to the heads old position
        if !self.is_tail_adjacent() {
            self.tail_position = old_position;
            self.visited_places.insert(self.tail_position.clone());
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

    fn is_tail_adjacent(&self) -> bool {
        self.head_position.distance_l0(&self.tail_position) <= 1
    }
}

pub fn run() -> (usize, usize) {

    let input = data_str!("day09");
    let data: Vec<Direction> = parse(input).expect("invalid input");
    let mut rope = PlanckRope::new();

    (
        rope.visited_positions(&data),
        0
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
        let input = r"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        ";

        let data: Vec<Direction> = parse(input).expect("invalid input");
        let mut rope = PlanckRope::new();

        assert_eq!(rope.visited_positions(&data), 13);
    }
}