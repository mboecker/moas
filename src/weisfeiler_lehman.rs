use crate::graph::Graph;
use std::collections::HashMap;

// Relabels the graph according to its immediate neighbors.Graph
pub fn relabel(g: &Graph) -> (Graph, HashMap<String, usize>) {
    use itertools::Itertools;

    // Create a copy of the graph.
    let mut g2 = Graph::with_size(g.size() as usize);

    // the bonds are the same
    *g2.bonds_mut() = g.bonds().clone();

    // Store unique ids for every node label in a HashMap.
    // This way we don't have to save the node label multiple times.
    let mut hm: HashMap<String, usize> = HashMap::new();
    let mut counter = 0;

    // relabel all the nodes
    for i in 0..g.size() {
        // The new name for a node is its own label, followed by a list of its neighbors.
        // For that, we iterate through all of its adjacent nodes and note the node label as well as the amount of edges between the node and its neighbor.
        let name = std::iter::once((0u8, g.atoms()[i as usize]))
            .chain(
                g.neighbors(i as usize)
                    .map(|j| (*g.bonds().get(i as usize, j), g.atoms()[j])),
            )
            .sorted()
            .map(|x| format!("({}-{})", x.0, x.1))
            .join("-");

        // Compress given name by looking it up in the dictionary.
        // If a node was given this name before, use its compressed name.
        // Otherwise give it a new compressed name.
        let compressed_name = *hm.entry(name).or_insert_with(|| {
            counter += 1;
            counter
        });

        *g2.atoms_mut().get_mut(i as usize).unwrap() = compressed_name;
    }

    (g2, hm)
}

#[test]
fn test_relabel() {
    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 8], [4, 8], [5, 7], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1]],
                "bonds": [[1, 7, 1], [1, 13, 1], [2, 12, 1], [3, 12, 2], [4, 13, 2], [5, 6, 1], [5, 8, 1], [5, 9, 1], [5, 10, 1], [6, 7, 1], [6, 15, 1], [6, 16, 1], [7, 11, 1], [7, 17, 1], [8, 18, 1], [8, 19, 1], [8, 20, 1], [9, 21, 1], [9, 22, 1], [9, 23, 1], [10, 24, 1], [10, 25, 1], [10, 26, 1], [11, 12, 1], [11, 27, 1], [11, 28, 1], [13, 14, 1], [14, 29, 1], [14, 30, 1], [14, 31, 1]]}"#;
    let mut g = crate::graph::Graph::new(j);
    println!("{:?}", g);

    for _ in 1..10 {
        let tmp = relabel(&g);
        //g.dump(g.size() as usize * i, false);
        println!("{:?}", tmp);
        g = tmp.0;
    }
}
