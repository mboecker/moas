use crate::subgraphs;
use crate::Graph;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subgraphs5AndRings {
    atoms: HashMap<Graph, usize>,
    subgraphs: HashMap<Graph, usize>,
    rings6: HashMap<Graph, usize>,
}

impl subgraphs::Subgraphs for Subgraphs5AndRings {
    fn new(g: &Graph) -> Self {
        assert!(g.size() >= 5);
        let atoms = g
            .label_counts()
            .into_iter()
            .map(|(i, c)| {
                let mut g = Graph::with_size(1);
                g.atoms_mut()[0] = i;
                (g, c)
            })
            .collect();

        let subgraphs = subgraphs::get_all(g, 5);

        let rings6 = if g.size() >= 6 {
            subgraphs::combine(g, &subgraphs, 6)
        } else {
            Vec::new()
        };

        let subgraphs = subgraphs::count_subgraphs(g, &subgraphs, 5);
        let mut rings6 = subgraphs::count_subgraphs(g, &rings6, 6);

        // Retain only rings in the rings5 and rings6 sets.
        rings6.retain(|k, _| k.is_circular());

        Subgraphs5AndRings {
            atoms,
            subgraphs,
            rings6,
        }
    }

    fn select_starting_graph(&self) -> Graph {
        self.all_subgraphs().next().unwrap().clone()
    }

    fn is_subset_of(&self, other: &Self) -> bool {
        // self is supposed to be a subset of other.
        // therefore, if self contains any subgraphs that are not contained in other,
        // or contains more of said subgraphs, its not a subset.

        for (k, v) in self.atoms.iter() {
            if other.atoms.get(k).unwrap_or(&0) < v {
                return false;
            }
        }

        for (k, v) in self.rings6.iter() {
            if other.rings6.get(k).unwrap_or(&0) < v {
                return false;
            }
        }

        for (k, v) in self.subgraphs.iter() {
            if other.subgraphs.get(k).unwrap_or(&0) < v {
                return false;
            }
        }

        true
    }

    fn all_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a Graph>> {
        Box::new(
            self.rings6
                .keys()
                .chain(self.subgraphs.keys())
                .chain(self.atoms.keys()),
        )
    }

    fn with_counts<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&'a Graph, &'a usize)>> {
        Box::new(
            self.rings6
                .iter()
                .chain(self.subgraphs.iter())
                .chain(self.atoms.iter()),
        )
    }

    fn attachable_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a Graph>> {
        Box::new(self.subgraphs.keys())
    }

    fn score(&self) -> usize {
        let has_ring6 = (self.rings6.len() > 0) as usize;
        let has_subgraph = (self.subgraphs.len() > 0) as usize;
        has_ring6 * 2 + has_subgraph
    }

    fn amount_of(&self, g: &Graph) -> usize {
        *self.subgraphs.get(g).unwrap_or(&0)
    }
}

impl Hash for Subgraphs5AndRings {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        use std::collections::hash_map::DefaultHasher;
        use std::collections::BTreeSet;

        // calculate the hashes for all subgraphs and sort them according to their hash values.
        let subgraphs: BTreeSet<_> = self
            .atoms
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
