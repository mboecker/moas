use crate::subgraphs;
use crate::Graph;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Eq, PartialEq)]
pub struct Only4 {
    subgraphs: HashMap<Graph, usize>,
}

impl subgraphs::Subgraphs for Only4 {
    fn new(g: &Graph) -> Self {
        assert!(g.size() >= 4);
        let subgraphs = subgraphs::get_all(g, 4);
        let subgraphs = subgraphs::count_subgraphs(g, &subgraphs, 4);

        Only4 { subgraphs }
    }

    fn select_starting_graph(&self) -> Graph {
        self.subgraphs.keys().next().unwrap().clone()
    }

    fn is_subset_of(&self, other: &Self) -> bool {
        // self is supposed to be a subset of other.
        // therefore, if self contains any subgraphs that are not contained in other,
        // or contains more of said subgraphs, its not a subset.

        for (k, v) in self.subgraphs.iter() {
            if other.subgraphs.get(k).unwrap_or(&0) < v {
                return false;
            }
        }
        true
    }

    fn all_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a Graph>> {
        Box::new(self.subgraphs.keys())
    }

    fn with_counts<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&'a Graph, &'a usize)>> {
        Box::new(self.subgraphs.iter())
    }

    fn attachable_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a Graph>> {
        Box::new(self.subgraphs.keys())
    }

    fn score(&self) -> usize {
        0
    }

    fn amount_of(&self, g: &Graph) -> usize {
        *self.subgraphs.get(g).unwrap_or(&0)
    }

    fn molecule_size(&self) -> usize {
        0
    }
}

impl Hash for Only4 {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        use std::collections::hash_map::DefaultHasher;
        use std::collections::BTreeSet;

        // calculate the hashes for all subgraphs and sort them according to their hash values.
        let subgraphs: BTreeSet<_> = self
            .subgraphs
            .iter()
            .map(|(g, c)| -> (u64, usize) {
                let mut h = DefaultHasher::default();
                g.hash(&mut h);
                (h.finish(), *c)
            })
            .collect();

        for sg in subgraphs {
            // hash the hashes from the subgraphs, in-order
            sg.hash(h);
        }
    }
}
