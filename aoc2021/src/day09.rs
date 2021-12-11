use std::fs;

use itertools::Itertools;

use crate::utils::{UnionFind, Map};

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

impl Map {
    fn risk(&self) -> usize {
        self.find_lowpoints().iter().map(|&p| self[p] as usize + 1).sum()
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
                if self[(x, y)] >= 9 {
                    continue;
                }
                let depth = self[(x, y)];
                let mut local_min = true;
                for (nx, ny) in self.neighbors((x, y)) {
                    if self[(nx, ny)] < 9 {
                        uf.union(self.width * y + x, self.width * ny + nx);
                        local_min &= depth < self[(nx, ny)]
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
            let mut candidates: Vec<_> = self.neighbors((x, y)).collect();
            while let Some((cx, cy)) = candidates.pop() {
                if visited[cy][cx] == NodeState::Visited || self[(cx, cy)] == 9 {
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
                score += 1;
            }
            scores.push(score)
        }

        scores.sort_unstable();
        scores
    }

    fn find_lowpoints(&self) -> Vec<(usize, usize)> {
        let mut output = Vec::new();
        for p in (0..self.width).cartesian_product(0..self.height) {
            let depth = self[p];

            if self.neighbors(p)
                .all(|p| depth < self[p])
            {
                output.push(p);
            }
        }

        output
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