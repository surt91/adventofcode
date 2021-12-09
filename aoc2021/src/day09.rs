use std::{str::FromStr, fs, iter};

use crate::utils::{AdventError, UnionFind};

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day09a.dat").expect("input file does not exist");
    let map: Map = input.parse().expect("invalid input");

    (
        map.risk(),
        map.basins(),
    )
}

#[derive(PartialEq, Clone)]
enum NodeState {
    Unseen,
    Seen,
    Visited
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
        let mut scores = Vec::new();

        let num = self.width * self.height;
        let mut uf = UnionFind::new(num);
        let mut low = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.depths[y][x] >= 9 {
                    continue;
                }
                let depth = self.depths[y][x];
                let mut local_min = true;
                for (nx, ny) in self.neighbors(x, y) {
                    if self.depths[ny][nx] < 9 {
                        uf.union(self.width * y + x, self.width * ny + nx);
                        local_min &= depth < self.depths[ny][nx]
                    }
                }
                if local_min {
                    low.push((x, y));
                }
            }
        }

        for (x, y) in low {
            let j = uf.find(self.width * y + x);
            let s = uf.size(j);
            scores.push(s);
        }

        scores.sort_unstable();
        scores
    }

    // the old DFS solution (which was 5% slower in my benchmarks)
    // but I like it enough that I do not want to delete it
    #[allow(dead_code)]
    fn basins_dfs(&self) -> usize {
        self.find_basin_sizes_dfs()
            .iter()
            .rev()
            .take(3)
            .product()
    }

    fn find_basin_sizes_dfs(&self) -> Vec<usize> {
        let mut scores = Vec::new();
        let mut visited = vec![vec![NodeState::Unseen; self.width]; self.height];

        for (x, y) in self.find_lowpoints() {
            let mut score: usize = 0;
            let mut candidates: Vec<_> = self.neighbors(x, y).collect();
            while let Some((cx, cy)) = candidates.pop() {
                if visited[cy][cx] == NodeState::Visited || self.depths[cy][cx] == 9 {
                    continue;
                }
                visited[cy][cx] = NodeState::Visited;
                let unseen_neighbors = self.neighbors(cx, cy)
                    .filter(|&(i, j)| {
                        let b = visited[j][i] == NodeState::Unseen;
                        if b {visited[j][i] = NodeState::Seen};
                        b
                    });
                candidates.extend(unseen_neighbors);
                score += 1;
            }
            scores.push(score)
        }

        scores.sort_unstable();
        scores
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
        )).flatten()
    }

    fn find_lowpoints(&self) -> Vec<(usize, usize)> {
        let mut output = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let depth = self.depths[y][x];

                if self.neighbors(x, y)
                    .all(|(x, y)| depth < self.depths[y][x])
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
        assert_eq!(map.basins_dfs(), 1134);
    }
}