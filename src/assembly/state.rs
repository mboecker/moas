use crate::Graph;

use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Eq, PartialEq, Debug, Clone)]
pub(super) struct State {
    pub g: Graph,
    pub used: HashMap<Graph, usize>,
    hash: u64,
}

impl State {
    pub fn new(g: Graph, used: HashMap<Graph, usize>) -> State {
        assert!(used.values().all(|x| x > &0));
        let hash = Self::hash(&g, used.iter());
        State { g, used, hash }
    }

    /// Pre-calculate the hash of such a `State`
    fn hash<'a>(g: &Graph, used: impl Iterator<Item = (&'a Graph, &'a usize)>) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::collections::BTreeSet;

        // construct a hasher and hash the big graph
        let mut h = DefaultHasher::default();
        g.hash(&mut h);

        // calculate the hashes for all subgraphs and sort them according to their hash values.
        let subgraphs: BTreeSet<_> = used
            .map(|(g, c)| -> (u64, usize) {
                let mut h = DefaultHasher::default();
                g.hash(&mut h);
                (h.finish(), *c)
            })
            .collect();

        for sg in subgraphs {
            // hash the hashes from the subgraphs, in-order
            sg.hash(&mut h);
        }

        // return the computed hash
        h.finish()
    }

    pub fn is_successful(&self, avail: &HashMap<Graph, usize>) -> bool {
        &self.used == avail
    }
}

impl Hash for State {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        self.hash.hash(h);
    }
}
