use std::{str::FromStr, fs};

use crate::utils::AdventError;


// 0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

// 5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day09a.dat").expect("input file does not exist");
    let map: Map = input.parse().expect("invalid input");

    (
        map.risk(),
        map.basins(),
    )
}

struct Map {
    width: usize,
    height: usize,
    depths: Vec<Vec<u8>>
}

impl Map {
    fn risk(&self) -> usize {
        self.find_lowpoints().iter().map(|&(x, y)| self.depths[y][x] as usize + 1).sum()
    }

    fn basins(&self) -> usize {
        self.find_basin_sizes()
            .iter()
            .rev()
            .take(3)
            .product()
    }

    fn find_basin_sizes(&self) -> Vec<usize> {
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut scores = Vec::new();

        for (x, y) in self.find_lowpoints() {
            let mut score: usize = 0;
            let mut candidates = self.neighbors(x, y);
            while !candidates.is_empty() {
                let (cx, cy) = candidates.pop().unwrap();
                if visited[cy][cx] || self.depths[cy][cx] == 9 {
                    continue;
                }
                visited[cy][cx] = true;
                candidates.append(&mut self.neighbors(cx, cy));
                score += 1;
            }
            scores.push(score)
        }

        scores.sort_unstable();
        scores
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let neighbors = vec![
            if y == 0 {None} else {Some((x, y-1))},
            if y >= self.height - 1  {None} else {Some((x, y+1))},
            if x == 0 {None} else {Some((x-1, y))},
            if x >= self.width - 1 {None} else {Some((x+1, y))},
        ];

        neighbors.into_iter()
            .flatten()
            .collect()
    }

    fn find_lowpoints(&self) -> Vec<(usize, usize)> {
        let mut output = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let depth = self.depths[y][x];

                if self.neighbors(x, y).iter()
                    .all(|&(x, y)| depth < self.depths[y][x])
                {
                    output.push((x, y));
                }
            }
        }

        output
    }
}

impl FromStr for Map {
    type Err = AdventError;

    fn from_str(input: &str) -> Result<Self, AdventError> {
        let depths: Vec<Vec<u8>> = input.trim().split('\n')
            .map(|line| line.trim().chars()
                .map(|c|
                    c.to_digit(10)
                    .map(|x| x as u8)
                    .ok_or(
                        AdventError::UnexpectedElement{found: c.to_string(), expected: vec!["a number".to_string()]})
                    ).collect::<Result<_, _>>()
                ).collect::<Result<_, _>>()?;

        let width = depths[0].len();
        let height = depths.len();
        assert!(depths.iter().all(|l| l.len() == width));

        Ok(
            Map {
                width,
                height,
                depths
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
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        ";

        let map: Map = input.parse().expect("invalid input");

        assert_eq!(map.risk(), 15);
        assert_eq!(map.basins(), 1134);
    }
}