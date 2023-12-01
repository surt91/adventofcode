use std::{str::FromStr, fmt::Display, fmt::Debug};

use aoc2021::{data_str, utils::AdventError};
use itertools::Itertools;

#[derive(Clone, Eq)]
enum Element {
    Integer(usize),
    List(Vec<Element>),
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => l == r,
            (Self::List(l), Self::List(r)) => l == r,
            _ => false,
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Element::Integer(lhs), Element::Integer(rhs)) => lhs.cmp(rhs),
            (Element::List(_lhs), Element::Integer(_rhs)) => self.cmp(&Element::List(vec![other.clone()])),
            (Element::Integer(_lhs), Element::List(_rhs)) => Element::List(vec![self.clone()]).cmp(other),
            (Element::List(lhs), Element::List(rhs)) => {
                if lhs == rhs {
                    // this is not correct
                    return std::cmp::Ordering::Equal
                }
                for (l, r) in lhs.iter().zip(rhs.iter()) {
                    if l != r {
                        return l.cmp(r)
                    }
                }
                lhs.len().cmp(&rhs.len())
            }
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Element::List(x) => write!(f, "[{}]", x.iter().join(", ")),
            Element::Integer(x) => write!(f, "{}", x)
        }
    }
}

impl FromStr for Element {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // we start with a list, strip the left and right bracket
        let s = &s[1..s.len()-1];
        let mut splitted = Vec::new();
        let mut buffer = Vec::new();
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '[' => {
                    stack.push('[');
                    buffer.push(c);
                },
                ']' => {
                    stack.pop();
                    buffer.push(c);},
                ',' => {
                    if stack.is_empty() {
                        splitted.push(buffer.iter().join(""));
                        buffer.clear();
                    } else {
                        buffer.push(c);
                    }
                },
                _ => buffer.push(c),
            }
        }
        splitted.push(buffer.iter().join(""));

        let elements: Vec<_> = splitted.iter()
            .map(|el| if el.starts_with('[') {
                el.parse::<Element>()
            } else if el.is_empty() {
                Ok(Element::List(Vec::new()))
            } else {
                el.trim()
                    .parse()
                    .map(Element::Integer)
                    .map_err(AdventError::Parser)
            })
            .collect::<Result<Vec<_>, AdventError>>()?;

        Ok(
            Element::List(elements)
        )
    }
}

pub fn run() -> (usize, usize) {

    let input = data_str!("day13");
    let pairs: Vec<(Element, Element)> = parse_pairs(input).expect("invalid input");
    let singlets: Vec<Element> = parse_singlets(input).expect("invalid input");

    (
        sum_of_right_idices(&pairs),
        decoder_key(&singlets)
    )
}

fn parse_pairs(data: &str) -> Result<Vec<(Element, Element)>, AdventError> {
    data.split("\n\n")
        .map(|block| block.trim().split('\n')
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple()
            .ok_or(AdventError::NotEnoughElements)
        )
        .collect()
}

fn parse_singlets(data: &str) -> Result<Vec<Element>, AdventError> {
    data.split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse())
        .collect()
}

fn sum_of_right_idices(elements: &[(Element, Element)]) -> usize {
    elements.iter()
        .enumerate()
        .filter(|(_n, pair)| pair.0 < pair.1)
        .map(|(n, _pair)| n + 1)
        .sum()
}

fn decoder_key(elements: &[Element]) -> usize {
    let sorted: Vec<_> = elements.iter().sorted().collect();

    let divider1: Element = "[[2]]".parse().unwrap();
    let divider2: Element = "[[6]]".parse().unwrap();

    let idx1 = sorted.iter()
        .enumerate()
        .find(|x| x.1 > &&divider1)
        .unwrap().0 + 1;
    let idx2 = sorted.iter()
        .enumerate()
        .find(|x| x.1 > &&divider2)
        .unwrap().0 + 2;

    idx1 * idx2
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Element {
        fn new(list: &[usize]) -> Element {
            Element::List(list.iter().map(|&i| Element::Integer(i)).collect())
        }
    }

    #[test]
    fn example() {

        assert!(Element::new(&[1, 1, 3, 1, 1]) < Element::new(&[1, 1, 5, 1, 1]));
        assert!(Element::new(&[7,7,7,7]) >= Element::new(&[7,7,7]));
        assert_eq!(Element::new(&[1, 1, 3, 1, 1]), "[1, 1, 3, 1, 1]".parse::<Element>().unwrap());

        assert!("[[1],[2,3,4]]".parse::<Element>().unwrap() <= "[[1],4]".parse::<Element>().unwrap());
        assert!(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Element>().unwrap()
            >= "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Element>().unwrap()
        );

        let input = r"
            [1,1,3,1,1]
            [1,1,5,1,1]

            [[1],[2,3,4]]
            [[1],4]

            [9]
            [[8,7,6]]

            [[4,4],4,4]
            [[4,4],4,4,4]

            [7,7,7,7]
            [7,7,7]

            []
            [3]

            [[[]]]
            [[]]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        ";

        let data: Vec<(Element, Element)> = parse_pairs(input).expect("invalid input");
        assert_eq!(sum_of_right_idices(&data), 13);

        let data: Vec<Element> = parse_singlets(input).expect("invalid input");
        assert_eq!(decoder_key(&data), 140);
    }
}