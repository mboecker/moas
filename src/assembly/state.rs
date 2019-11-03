use crate::Graph;

// use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(PartialEq, Eq, Debug, Clone)]
pub(super) struct State<S> {
    pub g: Graph,
    pub used: S,
    hash: u64,
}

impl<S> State<S>
where
    S: PartialEq + Hash,
{
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

// impl<S> PartialEq for State<S> {
//     fn eq(&self, other: &State<S>) -> bool {
//         self.cmp(other) == Ordering::Equal
//     }
// }

// impl<S> Eq for State<S> {}

// impl<S> PartialOrd for State<S> {
//     fn partial_cmp(&self, other: &State<S>) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl<S> Ord for State<S> {
//     fn cmp(&self, other: &State<S>) -> Ordering {
//         let v1 = self.g.first_unfull_node_id();
//         let v2 = other.g.first_unfull_node_id();

//         if v1.is_some() && v2.is_none() {
//             return Ordering::Less;
//         } else if v2.is_some() && v1.is_none() {
//             return Ordering::Greater;
//         } else {
//             v1.unwrap().cmp(&v2.unwrap())
//         }
//     }
// }
