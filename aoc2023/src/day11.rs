use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;

use aoc2021::data_str;
use aoc2021::utils::{AdventError, Map, Coord};


pub fn run() -> (usize, usize) {

    let input = data_str!("day11");
    let map: GalaxyMap = input.parse().expect("invalid input");

    (
        expanded_distance_sum(map.clone(), 2),
        expanded_distance_sum(map, 1000000),
    )
}

#[derive(Clone)]
struct GalaxyMap {
    map: Map<char>,
}

impl FromStr for GalaxyMap {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GalaxyMap {
            map: s.parse()?
        })
    }
}

impl GalaxyMap {
    fn find_galaxies(&self) -> Vec<Coord> {
        self.map.find_all('#')
    }

    fn expand(&mut self, factor: usize) -> Vec<Coord> {
        let galaxies = self.find_galaxies();
        let rows_with_galaxies: HashSet<usize> = galaxies.iter().map(|&(_x, y)| y).collect();
        let cols_with_galaxies: HashSet<usize> = galaxies.iter().map(|&(x, _y)| x).collect();
        let rows: HashSet<usize> = (0..self.map.height).collect();
        let cols: HashSet<usize> = (0..self.map.width).collect();
        let rows_without_galaxies: HashSet<&usize> = rows.difference(&rows_with_galaxies).collect();
        let cols_without_galaxies: HashSet<&usize> = cols.difference(&cols_with_galaxies).collect();

        let mut expanded_coords = galaxies;
        for c in &mut expanded_coords {
            for &&row in rows_without_galaxies.iter().sorted().rev() {
                if row < c.1 {
                    c.1 += factor - 1
                }
            }
            for &&col in cols_without_galaxies.iter().sorted().rev() {
                if col < c.0 {
                    c.0 += factor - 1
                }
            }
        }

        expanded_coords
    }
}

fn distance(c1: &Coord, c2: &Coord) -> usize {
    ((c1.0 as isize - c2.0 as isize).abs() + (c1.1 as isize - c2.1 as isize).abs()) as usize
}

fn expanded_distance_sum(mut map: GalaxyMap, expansion: usize) -> usize {
    map.expand(expansion)
        .iter()
        .combinations(2)
        .map(|i| distance(i[0], i[1]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        ";
        let map: GalaxyMap = input.parse().expect("invalid input");
        assert_eq!(expanded_distance_sum(map.clone(), 2), 374);
        assert_eq!(expanded_distance_sum(map.clone(), 10), 1030);
        assert_eq!(expanded_distance_sum(map, 100), 8410);
    }
}