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
                if v > 0 {
                    if g.bonds().get(mi, mj) == &0 {
                        *g.bonds_mut().get_mut(mi, mj) = v;
                        *g.bonds_mut().get_mut(mj, mi) = v;

                        if !g.is_edge_possible(mi, mj) {
                            // if g.size() == 9 {
                            //     use std::io::Write;
                            //     let hash: u64 = rand::random();
                            //     let filename = format!("trace/edge_{}.dot", hash);
                            //     let mut f = std::fs::File::create(filename).unwrap();
                            //     writeln!(&mut f, "graph invalid {{").unwrap();
                            //     g.dump(&mut f, 0, true).unwrap();
                            //     writeln!(&mut f, "}}").unwrap();
                            // }

                            return None;
                        }
                    }

                    // Since the edges (mi, mj) would have been added by this newly attached subgraph,
                    // its not allowed to add them in later iterations.
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
            // If i is not the newly added node but one of the nodes that's supposed to be mapped to an existing node.
            if let Some(&mi) = mapping.get(&i) {
                // How many bonds does this existing node have to the newly added node.
                let v = *sg.bonds().get(i, j);

                // Since this subgraph added no edge, no edge can be added in the future.
                // That would contradict the full edge information of this subgraph.
                if v == 0 {
                    g.set_edge_impossible(mi, mj);
                    g.set_edge_impossible(mj, mi);
                } else {
                    // Bonds are initialized with 0, so we just need to set them when they're not zero.
                    // Same with edge possibility, which is initialized with "possible".
                    *g.bonds_mut().get_mut(mi, mj) = v;
                    *g.bonds_mut().get_mut(mj, mi) = v;
                }
            }
        }

        // Dont make edges with hydrogen possible.
        for i in 0..g.size() {
            for j in 0..i {
                if crate::Atoms::max_bonds(g.atoms()[i]) == 1
                    || crate::Atoms::max_bonds(g.atoms()[j]) == 1
                {
                    g.set_edge_impossible(i, j);
                    g.set_edge_impossible(j, i);
                }
            }
        }

        Some(g)
    }
}
