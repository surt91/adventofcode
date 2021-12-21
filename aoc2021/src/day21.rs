use std::{fs, str::FromStr};

use itertools::Itertools;
use scan_fmt::scan_fmt;

use crate::utils::AdventError;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day21a.dat").expect("input file does not exist");
    let mut start: DiracDice = input.parse().expect("invalid input");

    (
        start.two_players(),
        0,
    )
}

trait Rollable {
    fn roll(&mut self) -> usize;
    fn num_rolls(&self) -> usize;
}

struct DeterministicDice {
    ctr: usize,
}

impl DeterministicDice {
    fn new() -> DeterministicDice {
        DeterministicDice {
            ctr: 0
        }
    }
}

impl Rollable for DeterministicDice {
    fn roll(&mut self) -> usize {
        self.ctr += 1;
        if self.ctr % 100 == 0 {
            100
        } else {
            self.ctr % 100
        }
    }

    fn num_rolls(&self) -> usize {
        self.ctr
    }
}

struct DiracDice {
    dice: Box<dyn Rollable>,
    players: Vec<usize>,
    scores: Vec<usize>,
}

impl DiracDice {
    fn two_players(&mut self) -> usize {
        let mut idx = 0;


        while self.scores.iter().max().unwrap() < &1000 {
            let d = (0..3).map(|_| self.dice.roll()).sum::<usize>();
            self.players[idx] += d;
            self.players[idx] = if self.players[idx] % 10 == 0 {10} else {self.players[idx] % 10};

            self.scores[idx] += self.players[idx];
            idx += 1;
            idx %= 2;
        }
        self.scores.iter().min().unwrap() * self.dice.num_rolls()
    }
}

impl FromStr for DiracDice {
    type Err = AdventError;

    fn from_str(lines: &str) -> Result<Self, AdventError> {
        let players: Vec<usize> = lines.trim().split('\n').map(|line|
            scan_fmt!(
                line.trim(),
                "Player {} starting position: {}",
                usize, usize
            )
        )
        .map_ok(|x| x.1)
        .collect::<Result<_, _>>()?;

        Ok(
            DiracDice {
                dice: Box::new(DeterministicDice::new()),
                scores: vec![0; players.len()],
                players,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            Player 1 starting position: 4
            Player 2 starting position: 8
        ";

        let mut start: DiracDice = input.parse().expect("invalid input");

        assert_eq!(start.two_players(), 739785);
    }
}