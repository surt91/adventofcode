use std::{fs, str::FromStr, cmp};

use itertools::{Itertools, iproduct};
use scan_fmt::scan_fmt;
use cached::proc_macro::cached;

use crate::utils::AdventError;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day21a.dat").expect("input file does not exist");
    let mut start: DiracDice = input.parse().expect("invalid input");
    let (w1, w2) = count_wins(start.players[0] as u8, start.players[1] as u8, 0, 0);

    (
        start.two_players(),
        cmp::max(w1, w2),
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


#[cached]
fn count_wins(start1: u8, start2: u8, score1: u8, score2: u8) -> (usize, usize) {
    let mut wins1 = 0;
    let mut wins2 = 0;

    for (i1, j1, k1) in iproduct!(1..=3, 1..=3, 1..=3) {
        let d1 = i1 + j1 + k1;
        let s1 = start1 + d1;
        let s1 = if s1 % 10 == 0 {10} else {s1 % 10};
        let sc1 = score1 + s1;

        if sc1 >= 21 {
            wins1 += 1;
            continue
        }

        for (i2, j2, k2) in iproduct!(1..=3, 1..=3, 1..=3) {
            let d2 = i2 + j2 + k2;
            let s2 = start2 + d2;
            let s2 = if s2 % 10 == 0 {10} else {s2 % 10};
            let sc2 = score2 + s2;

            if sc2 >= 21 {
                wins2 += 1;
                continue
            }

            let (w1, w2) = count_wins(s1, s2, sc1, sc2);
            wins1 += w1;
            wins2 += w2;
        }
    }

    (wins1, wins2)
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

        let (w1, w2) = count_wins(start.players[0] as u8, start.players[1] as u8, 0, 0);
        assert_eq!(start.two_players(), 739785);

        assert_eq!(w1, 444356092776315);
        assert_eq!(w2, 341960390180808);
    }
}