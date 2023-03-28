use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::{utils::{AdventError, AdjList, Indexable, shortest_path::Neighborful}, data_str};

pub fn run() -> (usize, usize) {
    let input = data_str!("day12a");
    let g: AdjList<Node> = input.parse().expect("invalid input");

    (
        g.count_paths(0),
        g.count_paths(1),
    )
}

#[derive(Clone, Copy, Debug)]
enum Node {
    Small(usize),
    Large(usize)
}

impl Indexable for Node {
    fn idx(&self) -> usize {
        *match self {
            Node::Large(x) => x,
            Node::Small(x) => x,
        }
    }
}

impl AdjList<Node> {
    fn count_paths(&self, num_twice: usize) -> usize {
        let mut visited: Vec<u8> = vec![0; self.size()];
        let mut path: Vec<usize> = Vec::new();
        let mut ctr = 0;
        let mut joker = num_twice;

        // 0 and 1 are magic values for start and end
        self.find_all_paths_until(0, 1, &mut visited, &mut path, &mut ctr, &mut joker);

        ctr
    }

    fn find_all_paths_until(
        &self,
        s: usize,
        t: usize,
        visited: &mut Vec<u8>,
        path: &mut Vec<usize>,
        ctr: &mut usize,
        joker: &mut usize
    ) {
        if let Node::Small(_) = self[s] {
            visited[s] += 1;
        }
        path.push(s);

        if s == t {
            // we are done
            *ctr += 1;
        } else {
            for n in self.neighbors(s) {
                if visited[n] == 0 {
                    self.find_all_paths_until(n, t, visited, path, ctr, joker);
                } else if *joker > 0 && visited[n] == 1 && n > 1 {
                    *joker -= 1;
                    self.find_all_paths_until(n, t, visited, path, ctr, joker);
                    *joker += 1;
                }
            }
        }

        path.pop();
        if let Node::Small(_) = self[s] {
            visited[s] -= 1;
        }
    }
}

impl FromStr for AdjList<Node>
{
    type Err = AdventError;

    fn from_str(input: &str) -> Result<Self, AdventError> {
        let mut ctr = 1;
        let mut map: HashMap<&str, Node> = HashMap::new();

        let mut str_to_id = |input: &str| -> Node {
            if input == "start" {
                return Node::Small(0);
            }
            if input == "end" {
                return Node::Small(1);
            }
            ctr += 1;
            if input.chars().next().unwrap().is_uppercase() {
                Node::Large(ctr)
            } else {
                Node::Small(ctr)
            }
        };

        // TODO: ideally, this should be done in a generic way in the graph module
        let mut edges = vec![];
        for line in input.trim().split('\n') {
            let mut elements = line.trim().split('-');
            let u = elements.next().ok_or(AdventError::NotEnoughElements)?;
            let ui = *map.entry(u).or_insert_with(|| str_to_id(u));
            let v = elements.next().ok_or(AdventError::NotEnoughElements)?;
            let vi = *map.entry(v).or_insert_with(|| str_to_id(v));

            edges.push((ui, vi));
        }

        let nodes = map.values()
            .sorted_by(|a, b| Ord::cmp(&a.idx(), &b.idx()))
            .cloned()
            .collect::<Vec<Node>>();

        let mut g = AdjList::new(nodes);
        for (s, t) in edges {
            g.add_edge(s, t);
        }

        Ok(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        ";
        let g: AdjList<Node> = input.parse().expect("invalid input");
        assert_eq!(g.count_paths(0), 10);
        assert_eq!(g.count_paths(1), 36);

        let input = r"
            dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc
        ";
        let g: AdjList<Node> = input.parse().expect("invalid input");
        assert_eq!(g.count_paths(0), 19);
        assert_eq!(g.count_paths(1), 103);

        let input = r"
            fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW
        ";
        let g: AdjList<Node> = input.parse().expect("invalid input");
        assert_eq!(g.count_paths(0), 226);
        assert_eq!(g.count_paths(1), 3509);
    }
}