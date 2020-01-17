use crate::Graph;
use std::collections::BTreeMap;

pub fn perform(
    g: &Graph,
    sg: &Graph,
    mapping: BTreeMap<usize, usize>,
    new_node: Option<usize>,
) -> Option<Graph> {
    if new_node.is_none() {
        // only a new edge has been added
        let mut g = g.clone();
        for i in 0..sg.size() {
            let mi = mapping[&i];

            for j in 0..i {
                let mj = mapping[&j];
                let v = *sg.bonds().get(i, j);
                if v > 0  {
                    if g.bonds().get(mi, mj) == &0 {
                        if !g.is_edge_possible(mi, mj) {
                            return None;
                        }

                        *g.bonds_mut().get_mut(mi, mj) = v;
                        *g.bonds_mut().get_mut(mj, mi) = v;
                    }

                    // // Since the edges (mi, mj) would have been added by this newly attached subgraph,
                    // // its not allowed to add them in later iterations.
                    g.set_edge_impossible(mi, mj);
                    g.set_edge_impossible(mj, mi);
                }
            }
        }
        Some(g)
    } else {
        // a new node has been added
        // println!("adding new node");
        let mut g = g.clone_with_extraspace(1);

        // node ids of the new node in both sg and g.
        let j = new_node.unwrap();
        let mj = g.size() - 1;

        // set correct label.
        g.atoms_mut()[mj] = sg.atoms()[j];

        for i in 0..sg.size() {
            if let Some(&mi) = mapping.get(&i) {
                let v = *sg.bonds().get(i, j);

                if v != *g.bonds().get(mi, mj) && !g.is_edge_possible(mi, mj) {
                    return None;
                }

                // No edge from the new node to the nodes of the just attached subgraph can be added in the future.
                if v == 0 {
                    g.set_edge_impossible(mi, mj);
                    g.set_edge_impossible(mj, mi);
                } else {

                    // Bonds are initialized with 0, so we just need to set them when they're not zero.
                    *g.bonds_mut().get_mut(mi, mj) = v;
                    *g.bonds_mut().get_mut(mj, mi) = v;
                }
            }
        }
        Some(g)
    }
}
