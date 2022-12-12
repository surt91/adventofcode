use std::ops::Index;
use std::hash::Hash;

use priority_queue::DoublePriorityQueue;
use rustc_hash::{FxHashSet, FxHashMap};

pub trait Neighborful<C> {
    fn neighbors(&self, coordinate: C) -> impl Iterator<Item=C> + '_;

    fn distance(c1: C, c2: C) -> usize;
}

pub fn astar<G, C>(graph: &G, start: C, end: C) -> Vec<C>
    where G: Neighborful<C> + Index<C, Output = u8>, C: Hash + Eq + Copy
{
    // https://de.wikipedia.org/wiki/A*-Algorithmus
    let mut open_list = DoublePriorityQueue::new();
    let mut closed_list = FxHashSet::default();
    let mut g: FxHashMap<C, usize> = FxHashMap::default();
    let mut predecessor: FxHashMap<C, C> = FxHashMap::default();

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

        for n in graph.neighbors(current) {
            if closed_list.contains(&n) {
                continue;
            }
            let tentative = g[&current] + graph[n] as usize;
            if open_list.get(&n).is_some() && tentative >= g[&n] {
                continue;
            }
            predecessor.insert(n, current);
            g.insert(n, tentative);
            // lower bound on the remaining cost
            let h = G::distance(end, n);
            let f = tentative + h;
            open_list.push(n, f);
        }
    }

    panic!("no path found")
}