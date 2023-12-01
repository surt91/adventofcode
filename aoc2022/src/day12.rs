use std::{str::FromStr, ops::Index};

use aoc2021::{data_str, utils::{AdventError, Map, Coord, shortest_path::{Neighborful, astar}}};
use itertools::Itertools;

struct LetterMap{
    map: Map,
    start: Coord,
    end: Coord,
}

impl Neighborful<Coord> for &LetterMap {
    fn neighbors(&self, coordinate: Coord) -> impl Iterator<Item=Coord> + '_ {

        (&self.map).neighbors(coordinate)
            .filter(move |c| self.map[c] <= self.map[coordinate] + 1)
            .collect_vec().into_iter()
    }

    fn distance(c1: Coord, c2: Coord) -> usize {
        <&Map>::distance(c1, c2)
    }
}

impl Index<Coord> for &LetterMap {
    type Output = u8;

    fn index(&self, coordinate: Coord) -> &Self::Output {
        let (x, y) = coordinate;
        &self.map.values[y][x]
    }
}

impl FromStr for LetterMap {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<Vec<u8>> = s.trim().split('\n')
            .map(|line| line.trim().chars()
                .map(|c| c as u8)
                .collect()
            ).collect();

        let width = values[0].len();
        let height = values.len();

        assert!(values.iter().all(|l| l.len() == width));

        let mut map = Map {
            width,
            height,
            values,
        };

        let start = map.values.iter()
            .enumerate()
            .flat_map(|(n, values)| values.iter().find_position(|&&c| c == b'S').map(|(pos, _)| (pos, n)))
            .next()
            .unwrap();
        let end = map.values.iter()
            .enumerate()
            .flat_map(|(n, values)| values.iter().find_position(|&&c| c == b'E').map(|(pos, _)| (pos, n)))
            .next()
            .unwrap();

        map[start] = b'a';
        map[end] = b'z';

        assert!(map.values.iter().all(|v| v.iter().all(|&x| (b'a'..=b'z').contains(&x))));

        Ok(
            LetterMap{
                map,
                start,
                end,
            }
        )
    }
}

pub fn run() -> (usize, usize) {

    let input = data_str!("day12");
    let data: LetterMap = input.parse().expect("invalid input");

    (
        shortest_path_length(&data),
        shortest_possible_path_length(&data)
    )
}

fn shortest_path_length(map: &LetterMap) -> usize {
    let start = map.start;
    let end = map.end;
    astar(&map, start, end).len() - 1
}

fn shortest_possible_path_length(map: &LetterMap) -> usize {
    // we should use dynamic programming and remember for each site how far it is to the end,
    //  but this brute ansatz is fast enough for now (yay, A*)
    let possible_starts = map.map.values.iter()
        .enumerate()
        .flat_map(|(n, values)| values.iter().find_position(|&&c| c == b'a').map(|(pos, _)| (pos, n)));
    let end = map.end;
    possible_starts.map(|start| astar(&map, start, end).len() - 1)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
        ";

        let data: LetterMap = input.parse().expect("invalid input");
        assert_eq!(shortest_path_length(&data), 31);
        assert_eq!(shortest_possible_path_length(&data), 29);
    }
}