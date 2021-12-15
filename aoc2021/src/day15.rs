use std::{fs, collections::{HashSet, HashMap}};

use priority_queue::DoublePriorityQueue;

use crate::utils::Map;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("data/day15a.dat").expect("input file does not exist");
    let map: Map = input.parse().expect("invalid input");

    (
        map.lowest_risk(),
        0
    )
}

impl Map {
    fn lowest_risk(&self) -> usize {
        let start = (0, 0);
        let end = (self.width-1, self.height-1);

        let shortest = self.astar(start, end);

        println!("{:?}", shortest);

        shortest.iter().rev().skip(1).map(|p| self[*p] as usize).sum()
    }

    fn astar(&self, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
        // https://de.wikipedia.org/wiki/A*-Algorithmus
        let mut open_list = DoublePriorityQueue::new();
        let mut closed_list = HashSet::new();
        let mut g: HashMap<(usize, usize), usize> = HashMap::new();
        let mut predecessor: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        open_list.push(start, 0);
        g.insert(start, 0);
        while let Some((current, _weight)) = open_list.pop_min() {
            if current == end {
                // path complete
                let mut tmp = end;
                let mut path = vec![end];
                while tmp != start {
                    let next = predecessor[&tmp];
                    path.push(next);
                    tmp = next;
                }
                return path
            }
            closed_list.insert(current);

            for n in self.neighbors(current) {
                if closed_list.contains(&n) {
                    continue;
                }
                let tentative = g[&current] + self[n] as usize;
                if open_list.get(&n).is_some() && tentative >= g[&n] {
                    continue;
                }
                predecessor.insert(n, current);
                g.insert(n, tentative);
                // lower bound on the remaining cost
                // assume all costs are >= 1 => h is manhattan distance
                let h = end.0 - n.0 + end.1 - n.1;
                let f = tentative + h;
                open_list.push(n, f);
            }
        }

        panic!("no path found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581
        ";

        let map: Map = input.parse().expect("invalid input");

        assert_eq!(map.lowest_risk(), 40);
    }
}