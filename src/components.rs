//! Connected-component queries via union-find.

use crate::io::Graph;
use crate::unionfind::UnionFind;

pub struct Components {
    uf: UnionFind,
    pub n_nodes: usize,
}

impl Components {
    pub fn compute(g: &Graph) -> Self {
        let n = g.n();
        let mut uf = UnionFind::new(n);
        for u in 0..n {
            for &v in &g.adj[u] {
                uf.union(u as u32, v);
            }
        }
        Self { uf, n_nodes: n }
    }

    /// Number of connected components.
    pub fn count(&mut self) -> usize {
        let mut seen = vec![false; self.n_nodes];
        let mut c = 0usize;
        for i in 0..self.n_nodes {
            let r = self.uf.find(i as u32) as usize;
            if !seen[r] {
                seen[r] = true;
                c += 1;
            }
        }
        c
    }

    /// Component sizes descending; canonical order: size desc, smallest node-id asc on ties.
    /// Returns `(sizes, groups)` where `groups[i]` is the node-ID list for component `i`.
    pub fn sizes_and_groups(&mut self) -> (Vec<usize>, Vec<Vec<u32>>) {
        let mut groups: std::collections::HashMap<u32, Vec<u32>> = std::collections::HashMap::new();
        for i in 0..self.n_nodes as u32 {
            let r = self.uf.find(i);
            groups.entry(r).or_default().push(i);
        }
        let mut group_list: Vec<Vec<u32>> = groups.into_values().collect();
        group_list.sort_unstable_by(|a, b| {
            b.len().cmp(&a.len()).then_with(|| {
                let min_a = a.iter().copied().min().unwrap_or(u32::MAX);
                let min_b = b.iter().copied().min().unwrap_or(u32::MAX);
                min_a.cmp(&min_b)
            })
        });
        let sizes: Vec<usize> = group_list.iter().map(|g| g.len()).collect();
        (sizes, group_list)
    }
}
