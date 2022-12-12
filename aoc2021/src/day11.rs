use std::iter;

use itertools::Itertools;

use crate::{utils::Map, data_str};

pub fn run() -> (usize, usize) {
    let input = data_str!("day11a");
    let mut map: Map = input.parse().expect("invalid input");

    let num = 100;

    (
        map.count_flashes(num),
        map.synchronized() + num,
    )
}

impl Map {
    fn count_flashes(&mut self, num: usize) -> usize {
        iter::repeat_with(|| self.step()).take(num).sum()
    }

    fn synchronized(&mut self) -> usize {
        iter::repeat_with(|| self.step()).take_while(|&x| x < 100).count() + 1
    }

    fn step(&mut self) -> usize {
        let mut ctr = 0;

        (0..self.width).cartesian_product(0..self.height).for_each(|p| self[p] += 1);

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
        let num = 100;

        assert_eq!(map.count_flashes(num), 1656);
        assert_eq!(map.synchronized() + num, 195);
    }
}