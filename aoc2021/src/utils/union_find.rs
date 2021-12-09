#[derive(Clone)]
struct Node{
    parent: Option<usize>,
    size: usize,
}

pub struct UnionFind {
    forest: Vec<Node>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind { forest: vec![Node {parent: None, size: 1}; n] }
    }

    pub fn size(&mut self, i: usize) -> usize {
        let root = self.find(i);
        self.forest[root].size
    }

    // we need mut because of path compression
    pub fn find(&mut self, i: usize) -> usize {
        let root = match self.forest[i].parent {
            Some(above) => self.find(above),
            None => i,
        };

        // path compression
        if self.forest[i].parent.is_some() {
            self.forest[i].parent = Some(root)
        };

        root
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        // we are already in the same cluster
        if root_i == root_j {
            return
        }

        // weighted
        let smaller;
        let larger;
        if self.forest[root_i].size > self.forest[root_j].size {
            larger = root_i;
            smaller = root_j;
        } else {
            larger = root_j;
            smaller = root_i;
        }

        self.forest[smaller].parent = Some(larger);
        self.forest[larger].size += self.forest[smaller].size;
        self.forest[smaller].size = 0;
    }
}
