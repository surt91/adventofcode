use std::{collections::HashSet, str::FromStr};

use aoc2021::{data_str, utils::{Map, AdventError}};

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
        0
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
    }
}