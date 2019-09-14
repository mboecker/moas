use crate::graph::Graph;
use itertools::Itertools;

fn normalize([a, b, c]: [usize; 3]) -> [usize; 3] {
    if a < b {
        if b < c {
            [a, b, c]
        } else {
            // c > b
            [a, c, b]
        }
    } else {
        // b < a
        if a < c {
            [b, a, c]
        } else {
            // c < a
            if b < c {
                [b, c, a]
            } else {
                // c < b
                [c, b, a]
            }
        }
    }
}

pub fn subgraphs3(g: &Graph) -> Vec<usize> {
    use std::collections::BTreeSet;

    // calculate the set of subgraphs
    let mut set = BTreeSet::new();
    for center in 0..g.size() {
        for (i, j) in (0..g.size())
            .filter(|i| g.bonds().get(center, *i) > &0)
            .tuple_combinations::<(_, _)>()
            .filter(|(a, b)| a < b)
        {
            set.insert(normalize([center, i, j]));
        }
    }

    // convert to vec
    let mut v = Vec::new();
    for entry in set {
        v.extend_from_slice(&entry);
    }
    v
}
