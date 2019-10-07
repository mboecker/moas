use crate::Graph;

use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Eq, PartialEq, Debug, Clone)]
pub(super) struct State<S> {
    pub g: Graph,
    pub used: S,
    hash: u64,
}

impl<S> State<S> where S: PartialEq + Hash {
    pub fn new(g: Graph, used: S) -> State<S> {
        let hash = Self::hash(&g, &used);
        State { g, used, hash }
    }

    /// Pre-calculate the hash of such a `State`
    fn hash<'a>(g: &Graph, used: &S) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        // construct a hasher and hash the big graph
        let mut h = DefaultHasher::default();
        g.hash(&mut h);
        used.hash(&mut h);
        
        // return the computed hash
        h.finish()
    }

    pub fn is_successful(&self, avail: &S) -> bool {
        &self.used == avail
    }
}

impl<S> Hash for State<S> {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        self.hash.hash(h);
    }
}
