use rayon::prelude::*;

use crate::{utils::{Map, AdventError, binary}, data_str};

pub fn run() -> (usize, usize) {
    let input = data_str!("day20a");
    let (rules, image) = parse(input).expect("invalid input");

    (
        image.multi_enhance(&rules, 2).count_light(),
        image.multi_enhance(&rules, 50).count_light(),
    )
}

impl Map<u8> {
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

        binary::to_usize(&out)
    }

    fn enhance(&self, rules: &[u8], default: u8) -> Map<u8> {
        let width = self.width+2;
        let height = self.height+2;

        let values = (0..height).into_par_iter().map(|j| {
            (0..width).map(|i| {
                let idx = self.block((i as isize - 1, j as isize - 1), default);
                rules[idx]
            }).collect()
        }).collect();

        Map {
            width,
            height,
            values
        }
    }

    fn multi_enhance(&self, rules: &[u8], n: usize) -> Map<u8> {
        let default = 0;
        let mut image = self.enhance(rules, default);
        for _ in 0..n-1 {
            let default = image[(0,0)];
            image = image.enhance(rules, default);
        }

        image
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

fn parse(input: &str) -> Result<(Vec<u8>, Map<u8>), AdventError> {
    let mut it = input.split("\n\n");
    let rules = it.next()
        .ok_or(AdventError::NotEnoughElements)?
        .trim()
        .chars()
        .map(|p| if p == '#' {1} else {0})
        .collect();

    let map: Map<u8> = it.next()
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

        assert_eq!(image.multi_enhance(&rules, 2).count_light(), 35);
        assert_eq!(image.multi_enhance(&rules, 50).count_light(), 3351);
    }
}