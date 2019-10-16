use crate::extra::Similar;
use crate::Graph;
use std::collections::HashSet;

fn verify_mapping(g1: &Graph, g2: &Graph, partial_mapping: &[usize]) -> bool {
    let n = g1.size();

    for (i, j) in partial_mapping.iter().enumerate() {
        if g1.atoms()[i] != g2.atoms()[*j] {
            return false;
        }
    }

    for i in 0..n {
        for j in 0..i {
            if g1.bonds().get(i, j) != g2.bonds().get(partial_mapping[i], partial_mapping[j]) {
                return false;
            }
        }
    }

    return true;
}

pub(super) fn are_isomorphic(g1: &Graph, g2: &Graph) -> bool {
    assert_eq!(g1.size(), g2.size());
    assert_eq!(g1.label_counts(), g2.label_counts());

    // Amount of nodes in the graphs.
    let n = g1.size();

    // Create an index structure to efficiently answer queries for nodes with the same label.
    let similar = Similar::new(&g2);

    // Space for the projective mapping.
    // partial_mapping[x] = y
    let mut partial_mapping: Vec<usize> = vec![0; n];
    let mut undecided_nodes: HashSet<_> = (0..n).collect();
    let mut taken_g2_nodes: HashSet<usize> = HashSet::new();
    let mut impossible = HashSet::new();

    // inner recursive function
    fn inner(
        g1: &Graph,
        g2: &Graph,
        similar: &Similar,
        undecided_nodes: &mut HashSet<usize>,
        taken_g2_nodes: &mut HashSet<usize>,
        partial_mapping: &mut Vec<usize>,
        impossible: &mut HashSet<Vec<usize>>,
    ) -> bool {
        // If this is a leaf node, check if the resulting mapping is valid.
        if undecided_nodes.len() == 0 {
            if impossible.contains(partial_mapping) {
                return false;
            }

            if verify_mapping(g1, g2, partial_mapping) {
                impossible.insert(partial_mapping.clone());
                return true;
            } else {
                return false;
            }
        }

        // try every undecided node in g1
        for current in undecided_nodes.clone().iter() {
            let label = g1.atoms()[*current];

            // nodes with the same label
            let similar_nodes: Vec<_> = similar
                .find(label)
                .filter(|i| !taken_g2_nodes.contains(i))
                .collect();

            // select possible candidates from g2
            for similar_node in similar_nodes {
                partial_mapping[*current] = similar_node;
                assert!(undecided_nodes.remove(current));
                assert!(taken_g2_nodes.insert(similar_node));

                // check if the rest is ok
                if inner(
                    g1,
                    g2,
                    similar,
                    undecided_nodes,
                    taken_g2_nodes,
                    partial_mapping,
                    impossible,
                ) {
                    return true;
                }

                assert!(undecided_nodes.insert(*current));
                assert!(taken_g2_nodes.remove(&similar_node));
            }

            return false;
        }

        // No isomorphism found
        false
    }

    inner(
        g1,
        g2,
        &similar,
        &mut undecided_nodes,
        &mut taken_g2_nodes,
        &mut partial_mapping,
        &mut impossible,
    )
}
