use std::{fs, iter};

use itertools::Itertools;

use crate::utils::Map;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day11a.dat").expect("input file does not exist");
    let mut map: Map = input.parse().expect("invalid input");

    (
        map.count_flashes(),
        map.synchronized() + 100,
    )
}

impl Map {
    fn count_flashes(&mut self) -> usize {
        iter::repeat_with(|| self.step()).take(100).sum()
    }

    fn synchronized(&mut self) -> usize {
        let mut ctr = 1;
        while self.step() < 100 {
            ctr += 1
        }
        ctr
    }

    fn step(&mut self) -> usize {
        let mut ctr = 0;

        for p in (0..self.width).cartesian_product(0..self.height) {
            self[p] += 1
        }

        let mut will_flash: Vec<_> = (0..self.width).cartesian_product(0..self.height)
            .filter(|&p| self[p] > 9)
            .collect();

        while let Some(p) = will_flash.pop() {
            if self[p] == 0 {
                continue
            }
            self[p] = 0;
            ctr += 1;
            for neighbor in self.diagonal_neighbors(p) {
                if self[neighbor] == 0 {
                    continue
                }
                self[neighbor] += 1;
                if self[neighbor] > 9 {
                    will_flash.push(neighbor);
                }
            }
        }

        ctr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        ";

        let mut map: Map = input.parse().expect("invalid input");

        assert_eq!(map.count_flashes(), 1656);
        assert_eq!(map.synchronized() + 100, 195);
    }
}