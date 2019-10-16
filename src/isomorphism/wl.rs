//! Implements a variant of the graph isomorphism algorithm by Weisefiler and Lehman.

use crate::graph::Graph;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Name {
    neighbors: [usize; 8],
}

impl FromIterator<(usize, usize)> for Name {
    fn from_iter<I: IntoIterator<Item = (usize, usize)>>(iter: I) -> Self {
        let mut neighbors = [0; 8];

        for (i, (x, y)) in iter.into_iter().enumerate() {
            if i < 4 {
                neighbors[i * 2] = x;
                neighbors[i * 2 + 1] = y;
            }
        }

        Name { neighbors }
    }
}

/// Relabels the graph according to its immediate neighbors.
pub fn relabel(g: &mut Graph) {
    use itertools::Itertools;

    let mut names: HashMap<usize, Name> = HashMap::new();

    // relabel all the nodes
    for i in 0..g.size() {
        // The new name for a node is its own label, followed by a list of its neighbors.
        // For that, we iterate through all of its adjacent nodes
        // and note the node label as well as the amount of edges between the node and its neighbor.
        let name = g
            .neighbors(i as usize)
            .map(|j| (*g.bonds().get(i as usize, j) as usize, g.atoms()[j]))
            .sorted()
            .collect();

        names.insert(i, name);
    }

    // assign ids to the names, where the lexicographically smallest name has the smallest id.
    let mut tmp: Vec<_> = names.values().cloned().collect();
    tmp.sort_unstable();

    let name_ids: HashMap<Name, usize> = tmp
        .into_iter()
        .dedup()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    let chunk = name_ids.values().max().unwrap() + 1;
    for (i, a) in g.atoms_mut().iter_mut().enumerate() {
        let b = *a * chunk + name_ids[&names[&i]];
        // println!("atom {}: {} + {} = {}", i, a, atoms2[i], b);
        *a = b;
    }
}

#[test]
fn test_relabel() {
    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 8], [4, 8], [5, 7], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1]],
                "bonds": [[1, 7, 1], [1, 13, 1], [2, 12, 1], [3, 12, 2], [4, 13, 2], [5, 6, 1], [5, 8, 1], [5, 9, 1], [5, 10, 1], [6, 7, 1], [6, 15, 1], [6, 16, 1], [7, 11, 1], [7, 17, 1], [8, 18, 1], [8, 19, 1], [8, 20, 1], [9, 21, 1], [9, 22, 1], [9, 23, 1], [10, 24, 1], [10, 25, 1], [10, 26, 1], [11, 12, 1], [11, 27, 1], [11, 28, 1], [13, 14, 1], [14, 29, 1], [14, 30, 1], [14, 31, 1]]}"#;
    let mut g = crate::graph::Graph::new(j);
    println!("{:?}", g);

    for _ in 1..10 {
        relabel(&mut g);
        println!("{:?}", g);
    }
}

#[test]
fn test_relabel_smol() {
    let j = r#"{"atoms": [[1, 1], [2, 1], [3, 1]],
                "bonds": [[1, 2, 1], [1, 3, 1]]}"#;
    let mut g = crate::graph::Graph::new(j);
    println!("{:?}", g);

    for _ in 1..10 {
        relabel(&mut g);
        println!("{:?}", g);
    }
}

#[test]
fn test_relabel_smol_variant2() {
    let j = r#"{"atoms": [[1, 1], [2, 1], [3, 1]],
                "bonds": [[1, 2, 1], [2, 3, 1]]}"#;
    let mut g = crate::graph::Graph::new(j);
    println!("{:?}", g);

    for _ in 1..10 {
        relabel(&mut g);
        println!("{:?}", g);
    }
}
