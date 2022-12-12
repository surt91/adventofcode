use std::{str::FromStr, cmp};

use itertools::{Itertools, iproduct};
use scan_fmt::scan_fmt;
use cached::proc_macro::cached;

use crate::{utils::AdventError, data_str};

pub fn run() -> (usize, usize) {
    let input = data_str!("day21a");
    let mut start: DiracDice = input.parse().expect("invalid input");
    let (w1, w2) = dirac_dice_wins(start.players[0] as u8, start.players[1] as u8);

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

fn dirac_dice_wins(start1: u8, start2: u8) -> (usize, usize) {
    count_wins(start1, start2, 0, 0, true)
}

#[cached]
fn count_wins(start1: u8, start2: u8, score1: u8, score2: u8, p1active: bool) -> (usize, usize) {
    if score1 >= 21 {
        return (1, 0)
    }
    if score2 >= 21 {
        return (0, 1)
    }

    let mut wins1 = 0;
    let mut wins2 = 0;

    for (i, j, k) in iproduct!(1..=3, 1..=3, 1..=3) {
        let d = i + j + k;
        let mut s1 = start1;
        let mut s2 = start2;
        let mut sc1 = score1;
        let mut sc2 = score2;

        if p1active {
            s1 += d;
            s1 = if s1 % 10 == 0 {10} else {s1 % 10};
            sc1 += s1;
        } else {
            s2 += d;
            s2 = if s2 % 10 == 0 {10} else {s2 % 10};
            sc2 += s2;
        }

        let (w1, w2) = count_wins(s1, s2, sc1, sc2, !p1active);
        wins1 += w1;
        wins2 += w2;
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

        let (w1, w2) = dirac_dice_wins(start.players[0] as u8, start.players[1] as u8);
        assert_eq!(start.two_players(), 739785);

        assert_eq!(w1, 444356092776315);
        assert_eq!(w2, 341960390180808);
    }
}