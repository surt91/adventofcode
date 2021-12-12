use std::{collections::HashMap};

use itertools::Itertools;

use crate::utils::read_lines;

pub fn run() -> (usize, usize) {
    let lines = read_lines("data/day12a.dat");
    let g = parse(&lines);

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

impl Node {
    fn unwrap(&self) -> usize {
        *match self {
            Node::Large(x) => x,
            Node::Small(x) => x,
        }
    }
}

struct AdjList {
    nodes: Vec<Node>,
    adj: Vec<Vec<usize>>,
}

impl AdjList {
    fn new(nodes: Vec<Node>) -> AdjList {
        let adj = vec![Vec::new(); nodes.len()];
        AdjList {
            nodes,
            adj,
        }
    }

    fn add_edge(&mut self, s: Node, t: Node) {
        let s = s.unwrap();
        let t = t.unwrap();

        self.adj[s].push(t);
        self.adj[t].push(s);
    }

    fn neighbors(&self, u: usize) -> impl Iterator<Item=usize> + '_ {
        self.adj[u].iter().cloned()
    }

    fn count_paths(&self, num_twice: usize) -> usize {
        let mut visited: Vec<u8> = vec![0; self.nodes.len()];
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
        if let Node::Small(_) = self.nodes[s] {
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
        if let Node::Small(_) = self.nodes[s] {
            visited[s] -= 1;
        }
    }
}

fn parse(input: &[String]) -> AdjList {
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

    let mut edges = vec![];
    for line in input.iter() {
        let mut elements = line.split('-');
        let u = elements.next().expect("invalid input");
        let ui = *map.entry(u).or_insert_with(|| str_to_id(u));
        let v = elements.next().expect("invalid input");
        let vi = *map.entry(v).or_insert_with(|| str_to_id(v));

        edges.push((ui, vi));
    }

    let nodes = map.values()
        .sorted_by(|a, b| Ord::cmp(&a.unwrap(), &b.unwrap()))
        .cloned()
        .collect::<Vec<Node>>();

    let mut g = AdjList::new(nodes);
    for (s, t) in edges {
        g.add_edge(s, t);
    }

    g
}

#[cfg(test)]
mod tests {
    use crate::utils::split_lines;

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
        let lines = split_lines(input);
        let g = parse(&lines);
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
        let lines = split_lines(input);
        let g = parse(&lines);
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
        let lines = split_lines(input);
        let g = parse(&lines);
        assert_eq!(g.count_paths(0), 226);
        assert_eq!(g.count_paths(1), 3509);
    }
}