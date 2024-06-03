use std::collections::HashSet;
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
        enclosed(&map),
    )
}

const NORTH: [char; 4] = ['S', '|', 'L', 'J'];
const SOUTH: [char; 4] = ['S', '|', 'F', '7'];
const EAST: [char; 4] = ['S', '-', 'F', 'L'];
const WEST: [char; 4] = ['S', '-', '7', 'J'];

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

        iter::once({
            // north
            if y != 0 {
                let neighbor_value = self.map[(x, y-1)];
                if NORTH.contains(&value) && SOUTH.contains(&neighbor_value) {
                    Some((x, y-1))
                } else {None}
            } else {None}
        }).chain(iter::once({
            // south
            if y < self.map.height - 1 {
                let neighbor_value = self.map[(x, y+1)];
                if SOUTH.contains(&value) && NORTH.contains(&neighbor_value) {
                    Some((x, y+1))
                } else {None}
            } else {None}
        })).chain(iter::once({
            // west
            if x != 0 {
                let neighbor_value = self.map[(x-1, y)];
                if WEST.contains(&value) && EAST.contains(&neighbor_value) {
                    Some((x-1, y))
                } else {None}
            } else {None}
        })).chain(iter::once({
            // east
            if x < self.map.width - 1 {
                let neighbor_value = self.map[(x+1, y)];
                if EAST.contains(&value) && WEST.contains(&neighbor_value) {
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

    fn value_of_s(&self) -> char {
        let s = self.find_start();
        let mut neighbors = self.neighbors(s);
        let before = neighbors.next().expect("invalid giant loop");
        let after = neighbors.next().expect("invalid giant loop");

        if before.0 == after.0 {
            '-'
        } else if before.1 == after.1 {
            '|'
        } else if s.1 + 1 == before.1 {
            if s.0 + 1 == after.0 {
                'F'
            } else if s.0 - 1 == after.0 {
                '7'
            } else {
                panic!()
            }
        } else if s.1 - 1 == before.1 {
            if s.0 + 1 == after.0 {
                'L'
            } else if s.0 - 1 == after.0 {
                'J'
            } else {
                panic!()
            }
        } else if s.0 + 1 == before.0 {
            if s.1 + 1 == after.1 {
                'F'
            } else if s.1 - 1 == after.1 {
                'L'
            } else {
                panic!()
            }
        } else if s.0 - 1 == before.0 {
            if s.1 + 1 == after.1 {
                '7'
            } else if s.1 - 1 == after.1 {
                'J'
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}

fn distance(map: &PipeMap) -> usize {
    map.dfs().len() / 2
}



fn enclosed(map: &PipeMap) -> usize {
    // a site is inside the loop, if we have an odd number of crossings with the loop via an arbitrary path to the border
    let mut num_enclosed = 0;

    // TODO: I need to find the correct shape of "S"
    let giant_loop = map.dfs();

    let giant_loop_set: HashSet<Coord> = giant_loop.iter().cloned().collect();
    for y in 0..map.map.height {
        for x in 0..map.map.width {
            let coord = (x, y);
            if giant_loop_set.contains(&coord) {
                continue;
            }
            let mut crossings = 0;
            let mut entry: Option<Coord> = None;
            let mut last: Option<Coord>;
            for ix in x..map.map.width {
                if giant_loop_set.contains(&(ix, y)) {
                    if entry.is_none() {
                        entry = Some((ix, y));
                    }
                    last = Some((ix, y));
                } else {
                    entry = None;
                    last = None;
                }

                if entry.is_some() {
                    let mut entry_value: char = map.map[entry.unwrap()];
                    let mut last_value: char = map.map[last.unwrap()];
                    if entry_value == 'S' {
                        entry_value = map.value_of_s();
                    }
                    if last_value == 'S' {
                        last_value = map.value_of_s();
                    }


                    if last_value == '-' {
                        continue;
                    } else if last_value == '|' {
                        crossings += 1;
                        entry = None;
                        continue;
                    } else if entry != last {
                        if (NORTH.contains(&entry_value) && NORTH.contains(&last_value))
                            || (SOUTH.contains(&entry_value) && SOUTH.contains(&last_value)) {
                            // tangent
                        } else {
                            crossings += 1;
                        }
                    }
                }
            }
            if crossings % 2 == 1 {
                num_enclosed += 1;
            }
        }
    }

    num_enclosed
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

        let input = r"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        ";
        let map: PipeMap = input.parse().expect("invalid input");
        assert_eq!(enclosed(&map), 4);

        let input = r"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        ";
        let map: PipeMap = input.parse().expect("invalid input");
        assert_eq!(enclosed(&map), 8);

        let input = r"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        ";
        let map: PipeMap = input.parse().expect("invalid input");
        assert_eq!(enclosed(&map), 10);
    }
}