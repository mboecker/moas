use crate::Graph;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub struct Subgraphs {
    atoms: BTreeMap<usize, usize>,
    subgraphs: HashMap<Graph, usize>,
    rings5: HashMap<Graph, usize>,
    rings6: HashMap<Graph, usize>,
}

// Calculate a composite set of subgraphs,
pub fn complex_subgraphs(g: &Graph) -> Subgraphs {
    let atoms = g.label_counts();

    let subgraphs = super::subgraphs(g, 4);
    let rings5 = super::combine(g, &subgraphs, 5);
    let rings6 = super::combine(g, &rings5, 6);

    let subgraphs = super::count_subgraphs(g, &subgraphs, 4);
    let mut rings5 = super::count_subgraphs(g, &rings5, 5);
    let mut rings6 = super::count_subgraphs(g, &rings6, 6);

    // Retain only rings in the rings5 and rings6 sets.
    rings5.retain(|k, _| k.is_circular());
    rings6.retain(|k, _| k.is_circular());

    Subgraphs {
        atoms,
        subgraphs,
        rings5,
        rings6,
    }
}
