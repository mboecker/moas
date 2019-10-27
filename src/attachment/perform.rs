use crate::Graph;
use std::collections::BTreeMap;

pub fn perform(
    g: &Graph,
    sg: &Graph,
    mapping: BTreeMap<usize, usize>,
    new_node: Option<usize>,
) -> Graph {
    if new_node.is_none() {
        // only a new edge has been added
        let mut g = g.clone();
        for i in 0..sg.size() {
            for j in 0..i {
                let mi = mapping[&i];
                let mj = mapping[&j];
                let v = *sg.bonds().get(i, j);
                if g.bonds().get(mi, mj) == &0 {
                    *g.bonds_mut().get_mut(mi, mj) = v;
                    *g.bonds_mut().get_mut(mj, mi) = v;
                }
            }
        }
        g
    } else {
        // a new node has been added
        // println!("adding new node");
        let new_node_sg_id = new_node.unwrap();
        let mut g = g.clone_with_extraspace(1);

        let j = new_node_sg_id;
        let mj = g.size() - 1;
        g.atoms_mut()[mj] = sg.atoms()[j];

        for i in 0..sg.size() {
            if let Some(&mi) = mapping.get(&i) {
                let v = *sg.bonds().get(i, j);
                *g.bonds_mut().get_mut(mi, mj) = v;
                *g.bonds_mut().get_mut(mj, mi) = v;
            }
        }
        g
    }
}
