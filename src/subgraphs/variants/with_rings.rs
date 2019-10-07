use crate::Graph;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use crate::subgraphs;

#[derive(Eq,PartialEq)]
pub struct SubgraphsAndRings {
    atoms: HashMap<Graph, usize>,
    subgraphs: HashMap<Graph, usize>,
    rings5: HashMap<Graph, usize>,
    rings6: HashMap<Graph, usize>,
}

impl subgraphs::Subgraphs for SubgraphsAndRings {
    fn new(g: &Graph) -> Self {
        assert!(g.size() > 4);
        let atoms = g.label_counts().into_iter().map(|(i,c)| {
            let mut g = Graph::with_size(1);
            g.atoms_mut()[0] = i;
            (g,c)
        }).collect();

        let subgraphs = subgraphs::get_all(g, 4);

        let rings5 = if g.size() > 5 {
            subgraphs::combine(g, &subgraphs, 5)
        } else {
            Vec::new()
        };

        let rings6 = if g.size() > 5 {
            subgraphs::combine(g, &rings5, 6)
        } else {
            Vec::new()
        };

        let subgraphs = subgraphs::count_subgraphs(g, &subgraphs, 4);
        let mut rings5 = subgraphs::count_subgraphs(g, &rings5, 5);
        let mut rings6 = subgraphs::count_subgraphs(g, &rings6, 6);

        // Retain only rings in the rings5 and rings6 sets.
        rings5.retain(|k, _| k.is_circular());
        rings6.retain(|k, _| k.is_circular());

        SubgraphsAndRings {
            atoms,
            subgraphs,
            rings5,
            rings6,
        }
    }

    fn select_starting_graph(&self) -> Graph {
        if !self.rings6.is_empty() {
            self.rings6.keys().next().unwrap().clone()
        } else if !self.rings5.is_empty() {
            self.rings5.keys().next().unwrap().clone()
        } else {
            self.subgraphs.keys().next().unwrap().clone()
        }
    }

    fn is_subset_of(&self, other: &Self) -> bool {
        true
    }

    fn basic_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=&'a Graph>> {
        Box::new(self.subgraphs.keys())
    }
}

impl Hash for SubgraphsAndRings {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        use std::collections::hash_map::DefaultHasher;
        use std::collections::BTreeSet;

        // calculate the hashes for all subgraphs and sort them according to their hash values.
        let subgraphs: BTreeSet<_> = self.atoms.iter()
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
