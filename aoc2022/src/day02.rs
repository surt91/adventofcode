use std::str::FromStr;

use itertools::Itertools;

use aoc2021::data_str;
use aoc2021::utils::{AdventError, split_lines};

#[derive(PartialEq, Eq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn value(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn needed_shape(&self, other: &Shape) -> Shape {
        match self {
            Outcome::Win => other.loses(),
            Outcome::Draw => other.draws(),
            Outcome::Loss => other.wins(),
        }
    }
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }

    fn score(&self, other: &Shape) -> u32 {
        self.value() + self.outcome(other).value()
    }

    fn outcome(&self, other: &Shape) -> Outcome {
        if &self.wins() == other {
            Outcome::Win
        } else if &self.draws() == other {
            Outcome::Draw
        } else if &self.loses() == other {
            Outcome::Loss
        } else {
            unreachable!()
        }
    }

    fn draws(&self) -> Shape {
        self.clone()
    }

    fn loses(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }
    }

    fn wins(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
    }
}

impl FromStr for Shape {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissor),
            _ => Err(AdventError::UnexpectedElement { found: s.to_string(), expected: &["A", "B", "C", "X", "Y", "Z"] })
        }
    }
}

impl FromStr for Outcome {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(AdventError::UnexpectedElement { found: s.to_string(), expected: &["X", "Y", "Z"] })
        }
    }
}

pub fn run() -> (u32, u32) {

    let input = data_str!("day02");
    let data = parse(input);
    let data2 = parse_for_outcome(input);

    (
        points_from_strategy(&data),
        points_from_strategy2(&data2),
    )
}

fn points_from_strategy(strategy: &[(Shape, Shape)]) -> u32 {
    strategy.iter()
        .map(|(opponent, myself)| myself.score(opponent))
        .sum()
}

fn points_from_strategy2(strategy: &[(Shape, Outcome)]) -> u32 {
    strategy.iter()
        .map(|(opponent, outcome)| {
            let myself = outcome.needed_shape(opponent);
            myself.score(opponent)
        })
        .sum()
}

fn parse(input: &str) -> Vec<(Shape, Shape)> {
    split_lines(input).iter()
        .map(|line| {
            line.split(' ')
            .map(|x|
                x.parse().unwrap()
            )
            .collect_tuple()
            .unwrap()
        })
        .collect()
}

fn parse_for_outcome(input: &str) -> Vec<(Shape, Outcome)> {
    split_lines(input).iter()
        .map(|line| {
            let mut chars = line.split(' ');
            let shape = chars.next().unwrap().parse().unwrap();
            let outcome = chars.next().unwrap().parse().unwrap();
            (shape, outcome)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            A Y
            B X
            C Z
        ";

        let data = parse(input);
        let data2 = parse_for_outcome(input);

        assert_eq!(points_from_strategy(&data), 15);
        assert_eq!(points_from_strategy2(&data2), 12);
    }
}