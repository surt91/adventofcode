use std::str::FromStr;

use aoc2021::data_str;
use aoc2021::utils::{AdventError, split_lines};
use counter::Counter;

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Number(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl FromStr for Card {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            "9" | "8" | "7" | "6" | "5" | "4" | "3" | "2" | "1" => Ok(Card::Number(s.parse().unwrap())),
            _ => Err(AdventError::UnexpectedElement {
                found: s.to_string(),
                expected: &["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "1"]
            })
        }
    }
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

impl FromStr for Hand {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<Card> = s.chars().map(|c| c.to_string().parse()).collect::<Result<_, AdventError>>()?;
        let card_counter: Counter<Card> = cards.iter().cloned().collect();
        match card_counter.len() {
            1 => Ok(Hand::FiveOfAKind(cards)),
            2 => {
                if card_counter.values().max().unwrap() == &4 {
                    Ok(Hand::FourOfAKind(cards))
                } else {
                    Ok(Hand::FullHouse(cards))
                }
            },
            3 => {
                if card_counter.values().max().unwrap() == &3 {
                    Ok(Hand::ThreeOfAKind(cards))
                } else {
                    Ok(Hand::TwoPair(cards))
                }
            },
            4 => Ok(Hand::OnePair(cards)),
            5 => Ok(Hand::HighCard(cards)),
            _ => unreachable!()
        }
    }
}

pub fn run() -> (usize, u64) {

    let input = data_str!("day07");
    let hands_and_bids = parse(input).expect("invalid input");

    (
        total_winnings(hands_and_bids),
        0
    )
}

fn parse(s: &str) -> Result<Vec<(Hand, u32)>, AdventError> {
    split_lines(s).iter().map(|line| {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let hand: Hand = parts[0].parse()?;
        let bid: u32 = parts[1].parse()?;
        Ok((hand, bid))
    }).collect()
}

fn total_winnings(mut hands_and_bids: Vec<(Hand, u32)>) -> usize {
    hands_and_bids.sort();
    hands_and_bids.into_iter()
        .enumerate()
        .map(|(rank, (_hand, bid))| (rank+1) * bid as usize )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        ";

        let hands_and_bids = parse(input).expect("invalid input");

        assert_eq!(total_winnings(hands_and_bids), 6440);
    }
}