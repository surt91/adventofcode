use std::iter;
use std::str::FromStr;

use aoc2021::data_str;
use aoc2021::utils::shortest_path::Neighborful;
use aoc2021::utils::{AdventError, Map, Coord};

pub fn run() -> (usize, usize) {

    let input = data_str!("day10");
    let map: PipeMap = input.parse().expect("invalid input");

    (
        distance(&map),
        0,
    )
}

#[derive(PartialEq, Clone)]
enum NodeState {
    Unseen,
    Seen,
    Visited
}

struct PipeMap {
    map: Map<char>,
}

impl FromStr for PipeMap {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PipeMap {
            map: s.parse()?
        })
    }
}

impl Neighborful<Coord> for &PipeMap {
    fn neighbors(&self, coordinate: Coord) -> impl Iterator<Item=Coord> {
        let (x, y) = coordinate;
        let value = self.map[coordinate];

        let north = vec!['S', '|', 'L', 'J'];
        let south = vec!['S', '|', 'F', '7'];
        let east = vec!['S', '-', 'F', 'L'];
        let west = vec!['S', '-', '7', 'J'];

        iter::once({
            // north
            if y != 0 {
                let neighbor_value = self.map[(x, y-1)];
                if north.contains(&value) && south.contains(&neighbor_value) {
                    Some((x, y-1))
                } else {None}
            } else {None}
        }).chain(iter::once({
            // south
            if y < self.map.height - 1 {
                let neighbor_value = self.map[(x, y+1)];
                if south.contains(&value) && north.contains(&neighbor_value) {
                    Some((x, y+1))
                } else {None}
            } else {None}
        })).chain(iter::once({
            // west
            if x != 0 {
                let neighbor_value = self.map[(x-1, y)];
                if west.contains(&value) && east.contains(&neighbor_value) {
                    Some((x-1, y))
                } else {None}
            } else {None}
        })).chain(iter::once({
            // east
            if x < self.map.width - 1 {
                let neighbor_value = self.map[(x+1, y)];
                if east.contains(&value) && west.contains(&neighbor_value) {
                    Some((x+1, y))
                } else {None}
            } else {None}
        })).flatten()
    }

    fn distance(_c1: Coord, _c2: Coord) -> usize {
        unimplemented!()
    }
}

impl PipeMap {
    fn find_start(&self) -> Coord {
        self.map.find_one('S').expect("Invalid input! Expected 'S'")
    }

    fn dfs(&self) -> Vec<Coord> {
        let mut visited = vec![vec![NodeState::Unseen; self.map.width]; self.map.height];

        let start = self.find_start();
        let mut giant_loop = vec![start];

        let mut candidates: Vec<_> = self.neighbors(start).collect();
        while let Some((cx, cy)) = candidates.pop() {
            if visited[cy][cx] == NodeState::Visited {
                continue;
            }
            visited[cy][cx] = NodeState::Visited;
            let unseen_neighbors = self.neighbors((cx, cy))
                .filter(|&(i, j)| {
                    let b = visited[j][i] == NodeState::Unseen;
                    if b {visited[j][i] = NodeState::Seen};
                    b
                });
            candidates.extend(unseen_neighbors);
            giant_loop.push((cx, cy));
        }

        giant_loop
    }
}

fn distance(map: &PipeMap) -> usize {
    map.dfs().len() / 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        ";

        let map: PipeMap = input.parse().expect("invalid input");
        assert_eq!(distance(&map), 4);

        let input = r"
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        ";
        let map: PipeMap = input.parse().expect("invalid input");
        assert_eq!(distance(&map), 8);
    }
}