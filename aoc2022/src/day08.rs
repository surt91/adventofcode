use std::{collections::HashSet, str::FromStr};

use aoc2021::{data_str, utils::{Map, AdventError}};
use itertools::Itertools;

struct TreeMap(Map);

impl TreeMap {

    fn visible_trees(&self) -> usize {
        // TODO: refactor this mess
        let map = &self.0;
        let mut total_visible: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..map.height {
            // left -> right
            let mut last_heigth = -1;
            for j in 0..(map.width-1) {
                let coord = (j, i);
                if map[coord] as i8 > last_heigth {
                    last_heigth = map[coord] as i8;
                    total_visible.insert(coord);
                }
            }
            // right -> left
            let mut last_heigth = -1;
            for j in (0..map.width).rev() {
                let coord = (j, i);
                if map[coord] as i8 > last_heigth {
                    last_heigth = map[coord] as i8;
                    total_visible.insert(coord);
                }
            }
        }
        for i in 0..map.width {
            // top -> down
            let mut last_heigth = -1;
            for j in 0..map.height {
                let coord = (i, j);
                if map[coord] as i8 > last_heigth {
                    last_heigth = map[coord] as i8;
                    total_visible.insert(coord);
                }
            }
            // bottom -> up
            let mut last_heigth = -1;
            for j in (0..map.height).rev() {
                let coord = (i, j);
                if map[coord] as i8 > last_heigth {
                    last_heigth = map[coord] as i8;
                    total_visible.insert(coord);
                }
            }
        }
        total_visible.len()
    }

    fn scenic_score(&self, tree: (usize, usize)) -> usize {
        let map = &self.0;
        let mut score = 1;
        for direction in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
            let mut view = 0;
            let mut coord = tree;
            score *= loop {
                if let Some(c) = self.step(coord, direction) {
                    view += 1;
                    if map[c] < map[tree] {
                        coord = c;
                    } else {
                        break view
                    }
                } else {
                    break view
                }
            }
        }
        score
    }

    fn step(&self, (x, y): (usize, usize), (dx, dy): (i8, i8)) -> Option<(usize, usize)> {
        let new_x = x as isize + dx as isize;
        let new_y = y as isize + dy as isize;
        if new_x < self.0.width as isize && new_x >= 0
            && new_y < self.0.height as isize && new_y >= 0 {
            Some((new_x as usize, new_y as usize))
        } else {
            None
        }
    }

    fn max_scenic_score(&self) -> usize {
        let map = &self.0;
        (0..map.width).cartesian_product(0..map.height)
            .map(|coord| self.scenic_score(coord))
            .max()
            .unwrap()
    }
}

impl FromStr for TreeMap {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Map = s.parse()?;
        Ok(TreeMap(map))
    }
}

pub fn run() -> (usize, usize) {

    let input = data_str!("day08");
    let map: TreeMap = input.parse().unwrap();

    (
        map.visible_trees(),
        map.max_scenic_score()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            30373
            25512
            65332
            33549
            35390
        ";

        let map: TreeMap = input.parse().unwrap();

        assert_eq!(map.visible_trees(), 21);
        assert_eq!(map.scenic_score((2, 1)), 4);
        assert_eq!(map.scenic_score((2, 3)), 8);
        assert_eq!(map.max_scenic_score(), 8);
    }
}