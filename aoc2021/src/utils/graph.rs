use std::ops::Index;

use super::shortest_path::Neighborful;

pub trait Indexable {
    fn idx(&self) -> usize;
}

pub struct AdjList<T: Indexable> {
    nodes: Vec<T>,
    adj: Vec<Vec<usize>>,
}

impl<T: Indexable> AdjList<T> {
    pub fn new(nodes: Vec<T>) -> AdjList<T> {
        let adj = vec![Vec::new(); nodes.len()];
        AdjList {
            nodes,
            adj,
        }
    }

    pub fn add_edge(&mut self, s: T, t: T) {
        let s = s.idx();
        let t = t.idx();

        self.adj[s].push(t);
        self.adj[t].push(s);
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}

impl<T: Indexable> Index<usize> for AdjList<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.nodes[idx]
    }
}

impl<T: Indexable> Index<T> for AdjList<T> {
    type Output = T;

    fn index(&self, node: T) -> &Self::Output {
        &self.nodes[node.idx()]
    }
}

impl<T: Indexable> Neighborful<usize> for AdjList<T> {
    fn neighbors(&self, coordinate: usize) -> impl Iterator<Item=usize> + '_ {
        self.adj[coordinate].iter().cloned()
    }

    fn distance(_c1: usize, _c2: usize) -> usize {
        unimplemented!()
    }
}
