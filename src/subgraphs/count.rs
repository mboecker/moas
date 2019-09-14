use crate::graph::Graph;
use std::collections::HashMap;

pub fn count_subgraphs(g: &Graph, idx: &[usize], k: usize) -> HashMap<Graph, usize> {
    use itertools::Itertools;

    // A hashmap to store the graphs and their count values in.
    let mut hm = HashMap::new();

    // Iterate over subgraphs, construct the graphs and count equal graphs (isomorphism) as the same graph.
    for sg in &idx.iter().chunks(k) {
        // Construct graph.
        let sg: Vec<_> = sg.cloned().collect();
        let sg = g.subgraph(&sg);

        // Increase count of these subgraphs by one.
        *hm.entry(sg).or_insert(0) += 1;
    }

    hm
}
