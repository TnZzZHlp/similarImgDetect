pub struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        DisjointSet {
            parent: (0..n).collect(),
        }
    }

    // Find the root node
    pub fn find_root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.find_root(self.parent[i]);
            self.parent[i]
        }
    }

    // Merge two sets
    pub fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find_root(i);
        let root_j = self.find_root(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
        }
    }
}
