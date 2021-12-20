use std::{fs};

use itertools::Itertools;

use crate::utils::{Map, AdventError};

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day20a.dat").expect("input file does not exist");
    let (rules, image) = parse(&input).expect("invalid input");

    let default = 0;
    let mut image = image.enhance(&rules, default);
    let default = image[(0,0)];
    image = image.enhance(&rules, default);
    let enhanced_twice = image.count_light();

    for _ in 0..48 {
        let default = image[(0,0)];
        image = image.enhance(&rules, default);
    }

    (
        enhanced_twice,
        image.count_light(),
    )
}

impl Map {
    fn new(width: usize, height: usize) -> Map {
        let values = vec![vec![0; width]; height];
        Map {
            width,
            height,
            values
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            None
        } else {
            Some(self.values[y as usize][x as usize])
        }
    }

    fn block(&self, center: (isize, isize), default: u8) -> usize {
        let mut out = Vec::new();
        let (x, y) = center;

        for j in y-1..=y+1 {
            for i in x-1..=x+1 {
                out.push(self.get(i, j).unwrap_or(default));
            }
        }

        let binary = out.iter().map(|&v| if v > 0 {'1'} else {'0'}).join("");
        usize::from_str_radix(&binary, 2).unwrap()
    }

    fn enhance(&self, rules: &[u8], default: u8) -> Map {
        let width = self.width+6;
        let height = self.height+6;
        let mut out = Map::new(width, height);

        for j in 0..height {
            for i in 0..width {
                let idx = self.block((i as isize - 3, j as isize - 3), default);
                let p = rules[idx];
                out[(i, j)] = p;
            }
        }

        out
    }

    fn count_light(&self) -> usize {
        let mut ctr = 0;
        for j in 0..self.height {
            for i in 0..self.width {
                if self[(i, j)] > 0 {
                    ctr += 1;
                }
            }
        }

        ctr
    }
}

fn parse(input: &str) -> Result<(Vec<u8>, Map), AdventError> {
    let mut it = input.split("\n\n");
    let rules = it.next()
        .ok_or(AdventError::NotEnoughElements)?
        .trim()
        .chars()
        .map(|p| if p == '#' {1} else {0})
        .collect();

    let map: Map = it.next()
        .ok_or(AdventError::NotEnoughElements)?
        .trim()
        .replace('#', "1")
        .replace('.', "0")
        .parse()?;

    Ok((rules, map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

            #..#.
            #....
            ##..#
            ..#..
            ..###
        ";

        let (rules, image) = parse(input).expect("invalid input");

        let default = 0;
        let mut image = image.enhance(&rules, default);
        let default = image[(0,0)];
        image = image.enhance(&rules, default);

        assert_eq!(image.count_light(), 35);

        for _ in 0..48 {
            let default = image[(0,0)];
            image = image.enhance(&rules, default);
        }

        assert_eq!(image.count_light(), 3351);
    }
}