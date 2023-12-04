use std::str::FromStr;

use aoc2021::data_str;
use aoc2021::utils::{AdventError, Map};

pub fn run() -> (u32, u32) {

    let input = data_str!("day03");
    let map: WideNumberMap = input.parse().expect("invalid input");

    (
        sum_of_numbers_with_neighbors(&map),
        0,
    )
}

struct WideNumberMap {
    map: Map<char>,
}

impl FromStr for WideNumberMap {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WideNumberMap {
            map: s.parse()?
        })
    }
}

impl WideNumberMap {
    fn numbers_with_neighbor_symbols(&self) -> Vec<u32> {
        let mut digit_string: Vec<char> = Vec::new();
        let mut has_symbol_neighbor = false;
        let mut numbers_with_neighbors: Vec<u32> = Vec::new();

        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.map.values[y][x].is_ascii_digit() {
                    digit_string.push(self.map.values[y][x]);
                    has_symbol_neighbor |= self.map.diagonal_neighbors((x, y))
                        .map(|(x, y)| self.map.values[y][x])
                        .any(|c| !(c.is_ascii_digit() || c == '.'))
                } else {
                    // in this case the number is complete
                    if !digit_string.is_empty() && has_symbol_neighbor {
                        let number: u32 = digit_string.iter().collect::<String>().parse().unwrap();
                        numbers_with_neighbors.push(number)
                    }
                    digit_string.clear();
                    has_symbol_neighbor = false;
                }
            }
            // in this case the number is complete
            if !digit_string.is_empty() && has_symbol_neighbor {
                let number: u32 = digit_string.iter().collect::<String>().parse().unwrap();
                numbers_with_neighbors.push(number)
            }
            digit_string.clear();
            has_symbol_neighbor = false;
        }

        numbers_with_neighbors
    }
}

fn sum_of_numbers_with_neighbors(map: &WideNumberMap) -> u32 {
    map.numbers_with_neighbor_symbols().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";

        let map: WideNumberMap = input.parse().expect("invalid input");

        assert_eq!(sum_of_numbers_with_neighbors(&map), 4361);
    }
}