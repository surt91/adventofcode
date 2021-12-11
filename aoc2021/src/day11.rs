use core::fmt;
use std::{str::FromStr, fs, iter};

use itertools::Itertools;

use crate::utils::{AdventError};

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day11a.dat").expect("input file does not exist");
    let mut map: Map = input.parse().expect("invalid input");

    (
        map.count_flashes(),
        0,
    )
}

struct Map {
    width: usize,
    height: usize,
    energies: Vec<Vec<u8>>
}

impl Map {
    fn count_flashes(&mut self) -> usize {
        iter::repeat_with(|| self.step()).take(100).sum()
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item=(usize, usize)> {
        iter::once(
            if y == 0 {None} else {Some((x, y-1))}
        ).chain(iter::once(
            if y >= self.height - 1  {None} else {Some((x, y+1))},
        )).chain(iter::once(
            if x == 0 {None} else {Some((x-1, y))},
        )).chain(iter::once(
            if x >= self.width - 1 {None} else {Some((x+1, y))},
        )).chain(iter::once(
            if x >= self.width - 1 || y >= self.height - 1 {None} else {Some((x+1, y+1))},
        )).chain(iter::once(
            if x ==0 || y >= self.height - 1 {None} else {Some((x-1, y+1))},
        )).chain(iter::once(
            if x >= self.width - 1 || y == 0 {None} else {Some((x+1, y-1))},
        )).chain(iter::once(
            if x == 0 || y == 0 {None} else {Some((x-1, y-1))},
        )).flatten()
    }

    fn step(&mut self) -> usize {
        let mut ctr = 0;

        for (x, y) in (0..self.width).cartesian_product(0..self.height) {
            self.energies[y][x] += 1
        }

        let mut will_flash: Vec<_> = (0..self.width).cartesian_product(0..self.height)
            .filter(|&(x, y)| self.energies[y][x] > 9)
            .collect();

        while let Some((x, y)) = will_flash.pop() {
            if self.energies[y][x] == 0 {
                continue
            }
            self.energies[y][x] = 0;
            ctr += 1;
            for (nx, ny) in self.neighbors(x, y) {
                if self.energies[ny][nx] == 0 {
                    continue
                }
                self.energies[ny][nx] += 1;
                if self.energies[ny][nx] > 9 {
                    will_flash.push((nx, ny));
                }
            }
        }

        ctr
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{} ", self.energies[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = AdventError;

    fn from_str(input: &str) -> Result<Self, AdventError> {
        let energies: Vec<Vec<u8>> = input.trim().split('\n')
            .map(|line| line.trim().chars()
                .map(|c|
                    c.to_digit(10)
                    .map(|x| x as u8)
                    .ok_or(
                        AdventError::UnexpectedElement{found: c.to_string(), expected: vec!["a number".to_string()]})
                    ).collect::<Result<_, _>>()
                ).collect::<Result<_, _>>()?;

        let width = energies[0].len();
        let height = energies.len();
        assert!(energies.iter().all(|l| l.len() == width));

        Ok(
            Map {
                width,
                height,
                energies
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

        // map.step();
        // map.step();
        // map.step();
        // assert_eq!(0, 1656);
        assert_eq!(map.count_flashes(), 1656);
    }
}