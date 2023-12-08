use std::cmp::{max, Ordering};
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

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
enum CardJ {
    J,
    Number(u8),
    T,
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

impl From<Card> for CardJ {

    fn from(card: Card) -> Self {
        match card {
            Card::A => CardJ::A,
            Card::K => CardJ::K,
            Card::Q => CardJ::Q,
            Card::J => CardJ::J,
            Card::T => CardJ::T,
            Card::Number(n) => CardJ::Number(n),
        }
    }
}

impl Hand<Card> {
    fn recognize_hand(cards: Vec<Card>) -> Hand<Card> {
        let card_counter: Counter<Card> = cards.iter().cloned().collect();
        match card_counter.len() {
            1 => Hand::FiveOfAKind(cards),
            2 => {
                if card_counter.values().max().unwrap() == &4 {
                    Hand::FourOfAKind(cards)
                } else {
                    Hand::FullHouse(cards)
                }
            },
            3 => {
                if card_counter.values().max().unwrap() == &3 {
                    Hand::ThreeOfAKind(cards)
                } else {
                    Hand::TwoPair(cards)
                }
            },
            4 => Hand::OnePair(cards),
            5 => Hand::HighCard(cards),
            _ => unreachable!()
        }
    }
}

impl Hand<CardJ> {
    fn recognize_hand(cards: Vec<CardJ>) -> Hand<CardJ> {
        let mut card_counter: Counter<CardJ> = cards.iter().cloned().collect();
        if card_counter[&CardJ::J] > 0 {
            let target = card_counter.iter()
                .filter(|(c, _n)| c != &&CardJ::J)
                .fold((0, CardJ::J), |acc, (c, &n)| {
                    match acc.0.cmp(&n) {
                        Ordering::Less => (n, c.clone()),
                        Ordering::Equal => (n, max(c.clone(), acc.1)),
                        Ordering::Greater => acc,
                    }
                });
            let target = if target.1 == CardJ::J {
                CardJ::A
            } else {
                target.1
            };

            card_counter[&target] += card_counter[&CardJ::J];
            card_counter[&CardJ::J] = 0
        }
        match card_counter.iter().filter(|(_c, &n)| n > 0).count() {
            1 => Hand::<CardJ>::FiveOfAKind(cards),
            2 => {
                if card_counter.values().max().unwrap() == &4 {
                    Hand::<CardJ>::FourOfAKind(cards)
                } else {
                    Hand::<CardJ>::FullHouse(cards)
                }
            },
            3 => {
                if card_counter.values().max().unwrap() == &3 {
                    Hand::<CardJ>::ThreeOfAKind(cards)
                } else {
                    Hand::<CardJ>::TwoPair(cards)
                }
            },
            4 => Hand::<CardJ>::OnePair(cards),
            5 => Hand::<CardJ>::HighCard(cards),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
enum Hand<C: Ord> {
    HighCard(Vec<C>),
    OnePair(Vec<C>),
    TwoPair(Vec<C>),
    ThreeOfAKind(Vec<C>),
    FullHouse(Vec<C>),
    FourOfAKind(Vec<C>),
    FiveOfAKind(Vec<C>),
}

impl FromStr for Hand<Card> {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<Card> = s.chars().map(|c| c.to_string().parse()).collect::<Result<_, AdventError>>()?;
        Ok(Hand::<Card>::recognize_hand(cards))
    }
}

impl FromStr for Hand<CardJ> {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<CardJ> = s.chars().map(|c| CardJ::from(c.to_string().parse::<Card>().unwrap())).collect();
        Ok(Hand::<CardJ>::recognize_hand(cards))
    }
}

pub fn run() -> (usize, usize) {
    let input = data_str!("day07");
    let hands_and_bids = parse(input).expect("invalid input");
    let hands_and_bids_with_joker = parse_with_joker(input).expect("invalid input");

    (
        total_winnings(hands_and_bids),
        total_winnings(hands_and_bids_with_joker)
    )
}

fn parse(s: &str) -> Result<Vec<(Hand<Card>, u32)>, AdventError> {
    split_lines(s).iter().map(|line| {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let hand: Hand<Card> = parts[0].parse()?;
        let bid: u32 = parts[1].parse()?;
        Ok((hand, bid))
    }).collect()
}

fn parse_with_joker(s: &str) -> Result<Vec<(Hand<CardJ>, u32)>, AdventError> {
    split_lines(s).iter().map(|line| {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let hand: Hand<CardJ> = parts[0].parse()?;
        let bid: u32 = parts[1].parse()?;
        Ok((hand, bid))
    }).collect()
}

fn total_winnings<H: Ord>(mut hands_and_bids: Vec<(H, u32)>) -> usize {
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
        let hands_and_bids_with_joker = parse_with_joker(input).expect("invalid input");

        assert_eq!(total_winnings(hands_and_bids), 6440);
        assert_eq!(total_winnings(hands_and_bids_with_joker), 5905);
    }
}