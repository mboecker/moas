//! Implements a variant of the graph isomorphism algorithm by Weisefiler and Lehman.

use crate::graph::Graph;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

type Name = [(u8, usize); 4];

/// Relabels the graph according to its immediate neighbors.
pub fn relabel(g: &mut Graph) {
    debug_assert!((0..g.size())
        .map(|i| g.neighbors(i).map(|j| g.bonds().get(i, j)).sum())
        .all(|x: u8| x < 5));

    // Contains a new name for every node.
    let mut names: Vec<Name> = Vec::with_capacity(g.size());

    // Contains an index of the names in sorted order.
    let mut name_ids: BTreeSet<Name> = BTreeSet::new();

    // relabel all the nodes
    for i in 0..g.size() {
        // The new name for a node is a list of its neighbors.
        // For that, we iterate through all of its adjacent nodes
        // and note the node label as well as the amount of edges between the node and its neighbor.
        let mut name: [(u8, usize); 4] = [(0, 0); 4];

        for (idx, p) in g
            .neighbors(i as usize)
            .map(|j| (*g.bonds().get(i as usize, j), g.atoms()[j]))
            .enumerate()
            .take(4)
        {
            name[idx] = p;
        }

        // The adjacent nodes are sorted by node label and number of bonds.
        name.sort_unstable();

        // Insert into arrays.
        names.push(name);
        name_ids.insert(name);
    }

    // assign ids to the names, where the lexicographically smallest name has the smallest id.
    let name_ids: BTreeMap<Name, usize> = name_ids
        .into_iter()
        .enumerate()
        .map(|(name_id, name)| (name, name_id))
        .collect();

    let chunk = name_ids.len() + 2;
    for (i, a) in g.atoms_mut().iter_mut().enumerate() {
        let b = *a * chunk + name_ids[&names[i]];
        // println!("atom {}: {} + {} = {}", i, a, atoms2[i], b);
        *a = b;
    }
}

#[test]
fn test_relabel() {
    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 8], [4, 8], [5, 7], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1]],
                "bonds": [[1, 7, 1], [1, 13, 1], [2, 12, 1], [3, 12, 2], [4, 13, 2], [5, 6, 1], [5, 8, 1], [5, 9, 1], [5, 10, 1], [6, 7, 1], [6, 15, 1], [6, 16, 1], [7, 11, 1], [7, 17, 1], [8, 18, 1], [8, 19, 1], [8, 20, 1], [9, 21, 1], [9, 22, 1], [9, 23, 1], [10, 24, 1], [10, 25, 1], [10, 26, 1], [11, 12, 1], [11, 27, 1], [11, 28, 1], [13, 14, 1], [14, 29, 1], [14, 30, 1], [14, 31, 1]]}"#;
    let mut g = crate::graph::Graph::from_old_json(j);
    println!("{:?}", g);

    for _ in 1..10 {
        relabel(&mut g);
        println!("{:?}", g);
    }
}

#[test]
fn test_relabel_smol_variant1() {
    let j = r#"{"atoms": [[1, 1], [2, 1], [3, 1]],
                "bonds": [[1, 2, 1], [1, 3, 1]]}"#;
    let mut g = crate::graph::Graph::from_old_json(j);
    println!("{:?}", g);

    relabel(&mut g);
    println!("{:?}", g);
}

#[test]
fn test_relabel_smol_variant2() {
    let j = r#"{"atoms": [[1, 1], [2, 1], [3, 1]],
                "bonds": [[1, 2, 1], [2, 3, 1]]}"#;
    let mut g = crate::graph::Graph::from_old_json(j);
    println!("{:?}", g);

    relabel(&mut g);
    println!("{:?}", g);
}

#[test]
fn test_relabel_smol_variant3() {
    let j = r#"{"atoms": [[1, 1], [2, 1], [3, 1]],
                "bonds": [[1, 3, 1], [2, 3, 1]]}"#;
    let mut g = crate::graph::Graph::from_old_json(j);
    println!("{:?}", g);

    relabel(&mut g);
    println!("{:?}", g);
}
