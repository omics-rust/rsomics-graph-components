//! Union-Find with path compression and union by size.

pub struct UnionFind {
    parent: Vec<u32>,
    size: Vec<u32>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n as u32).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            // path halving
            self.parent[x as usize] = self.parent[self.parent[x as usize] as usize];
            x = self.parent[x as usize];
        }
        x
    }

    pub fn union(&mut self, a: u32, b: u32) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        // union by size: attach smaller tree under larger
        if self.size[ra as usize] < self.size[rb as usize] {
            self.parent[ra as usize] = rb;
            self.size[rb as usize] += self.size[ra as usize];
        } else {
            self.parent[rb as usize] = ra;
            self.size[ra as usize] += self.size[rb as usize];
        }
    }
}
