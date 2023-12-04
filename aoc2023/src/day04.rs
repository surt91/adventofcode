use std::collections::HashSet;
use std::str::FromStr;

use aoc2021::data_str;
use aoc2021::utils::{AdventError, split_lines};

struct Card {
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl FromStr for Card {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_card_no, rest) = s.trim()
            .split_once(':')
            .ok_or(AdventError::NotEnoughElements)?;
        let (winning, numbers) = rest
            .split_once('|')
            .ok_or(AdventError::NotEnoughElements)?;

        let winning = winning.split_ascii_whitespace()
            .map(|s| s.trim().parse().map_err(AdventError::Parser))
            .collect::<Result<HashSet<u32>, AdventError>>()?;

        let numbers = numbers.split_ascii_whitespace()
            .map(|s| s.trim().parse().map_err(AdventError::Parser))
            .collect::<Result<HashSet<u32>, AdventError>>()?;

        Ok(Card {
            winning,
            numbers,
        })
    }
}

pub fn run() -> (u32, u32) {

    let input = data_str!("day04");
    let cards: Vec<Card> = parse(input).expect("invalid input");

    (
        score(&cards),
        num_scratchcards(&cards),
    )
}

fn parse(input: &str) -> Result<Vec<Card>, AdventError> {
    split_lines(input).iter()
        .map(|line| line.parse())
        .collect()
}

fn score(cards: &[Card]) -> u32 {
    cards.iter()
        .map(|card| card.winning.intersection(&card.numbers).count())
        .map(|count|
            if count == 0 {
                0
            } else {
                2u32.pow(count as u32 - 1)
            }
        )
        .sum()
}

fn num_scratchcards(cards: &[Card]) -> u32 {
    let mut num_cards = vec![1; cards.len()];
    for (n, card) in cards.iter().enumerate() {
        let count = card.winning.intersection(&card.numbers).count();
        for i in 1..=count {
            if n+i >= cards.len() {
                break;
            }
            num_cards[n+i] += num_cards[n];
        }
    }

    num_cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        let cards: Vec<Card> = parse(input).expect("invalid input");

        assert_eq!(score(&cards), 13);
        assert_eq!(num_scratchcards(&cards), 30);
    }
}