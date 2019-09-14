use crate::graph::Graph;
use crate::subgraphs::base_case::subgraphs3;
use std::collections::HashSet;

pub fn subgraphs(g: &Graph, k: usize) -> Vec<usize> {
    if k < 3 {
        panic!("subgraphs <3 not supported");
    } else if k == 3 {
        // Special case for subgraphs of size 3.
        subgraphs3(g)
    } else {
        // Generate parts with size one less.
        let parts = subgraphs(g, k - 1);

        let mut new_parts = HashSet::with_capacity(3 * parts.len());
        for chunk in parts.chunks(k - 1) {
            for current in chunk {
                for neighbor in g.neighbors(*current) {
                    if !chunk.contains(&neighbor) {
                        let mut v = Vec::with_capacity(k);
                        v.extend_from_slice(&chunk);
                        v.push(neighbor);
                        v.sort_unstable();
                        new_parts.insert(v);
                    }
                }
            }
        }

        let mut v = Vec::new();
        for part in new_parts {
            v.extend_from_slice(&part);
        }
        v
    }
}
