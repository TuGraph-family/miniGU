use std::collections::HashMap;
use minigu_common::types::VertexId;

pub struct UnionFind {
    parent: HashMap<VertexId, VertexId>,
    rank: HashMap<VertexId, usize>,
}

impl UnionFind {
    pub fn new() -> Self {
        Self {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    pub fn make_set(&mut self, x: VertexId) {
        if !self.parent.contains_key(&x) {
            self.parent.insert(x, x);
            self.rank.insert(x, 0);
        }
    }

    pub fn find(&mut self, x: VertexId) -> VertexId {
        if let Some(&parent) = self.parent.get(&x) {
            if parent != x {
                let root = self.find(parent);
                self.parent.insert(x, root);
                return root;
            }
        } else {
            self.make_set(x);
        }
        x
    }

    pub fn union(&mut self, x: VertexId, y: VertexId) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        let rank_x = *self.rank.get(&root_x).unwrap_or(&0);
        let rank_y = *self.rank.get(&root_y).unwrap_or(&0);

        if rank_x < rank_y {
            self.parent.insert(root_x, root_y);
        } else if rank_x > rank_y {
            self.parent.insert(root_y, root_x);
        } else {
            self.parent.insert(root_y, root_x);
            *self.rank.entry(root_x).or_insert(0) += 1;
        }

        true
    }
}

