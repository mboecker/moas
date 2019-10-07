use crate::graph::Graph;
use crate::subgraphs::base_case::subgraphs3;
use std::collections::HashSet;

pub fn combine(g: &Graph, parts: &[usize], k: usize) -> Vec<usize> {
    // Allocate space for new subgraphs.
    let mut new_parts = HashSet::with_capacity(parts.len());

    // Iterate over subgraphs
    for chunk in parts.chunks(k - 1) {
        // Iterate over nodes in the subgraph
        for current in chunk {
            // Iterate over adjacent nodes in the graph that are not already in the subgraph.
            for neighbor in g.neighbors(*current) {
                if !chunk.contains(&neighbor) {
                    // New subgraph detected
                    let mut v = Vec::with_capacity(k);
                    v.extend_from_slice(&chunk);
                    v.push(neighbor);
                    v.sort_unstable();
                    new_parts.insert(v);
                }
            }
        }
    }

    // Convert HashSet into flattened vector representation.
    let mut v = Vec::new();
    for part in new_parts {
        v.extend_from_slice(&part);
    }
    v
}

pub fn get_all(g: &Graph, k: usize) -> Vec<usize> {
    if k < 3 {
        panic!("subgraphs <3 not supported");
    } else if k == 3 {
        // Special case for subgraphs of size 3.
        subgraphs3(g)
    } else {
        // Generate parts with size one less.
        let parts = get_all(g, k - 1);

        // Combine them to make larger subgraphs.
        combine(g, &parts, k)
    }
}
