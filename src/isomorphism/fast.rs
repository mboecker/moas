use crate::Graph;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

/// A fast method that checks some simple graph properties.
/// For two graphs to be isomorphic, these have to line up.
/// Since this function has an one-sided error,
/// a positive result of this function needs to be checked.
/// You should use the `is_isomorphic` function, which calls this to sort out simple cases.
pub fn are_isomorphic(g1: &Graph, g2: &Graph) -> bool {
    // Check if the graphs have the same size.
    if g1.size() != g2.size() {
        return false;
    }

    if g1.number_of_edges() != g2.number_of_edges() {
        return false;
    }

    // Check if they have the same hash value.
    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = hasher1.clone();
    g1.hash(&mut hasher1);
    g2.hash(&mut hasher2);
    hasher1.finish() == hasher2.finish()
}
