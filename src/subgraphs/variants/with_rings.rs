use crate::subgraphs;
use crate::Graph;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubgraphsAndRings {
    atoms: HashMap<Graph, usize>,
    subgraphs: HashMap<Graph, usize>,
    rings5: HashMap<Graph, usize>,
    rings6: HashMap<Graph, usize>,
}

impl SubgraphsAndRings {
    fn check_for_partials(&self, other: &Self) -> bool {
        let mut missing: HashMap<Graph, usize> = HashMap::new();
        let mut available: HashMap<&Graph, usize> = HashMap::new();

        // Scan for subgraphs that are still missing.
        for (sg, v) in self.subgraphs.iter() {
            let m = *other.subgraphs.get(sg).unwrap_or(&0) as isize - *v as isize;
            if m < 0 {
                use itertools::Itertools;

                // Check, if they contain exactly two atoms that have only one bond.
                // This indicates that the missing subgraph is a chain.
                let iter: Option<(usize, usize)> = (0..sg.size()).filter(|i| sg.neighbors(*i).count() == 1).collect_tuple();

                if let Some((i, j)) = iter {

                    // Close the chain to a cycle and check if there are any of those available.
                    let mut closed_sg = sg.clone();
                    *closed_sg.bonds_mut().get_mut(i, j) = 1;
                    *closed_sg.bonds_mut().get_mut(j, i) = 1;
                    *missing.entry(closed_sg).or_default() += -m as usize;
                } else {
                    // We dont handle other missing graphs.
                    return false;
                }
            }
        }

        // Scan for subgraphs that are still available.
        for (sg, v) in other.subgraphs.iter() {
            let m = *self.subgraphs.get(sg).unwrap_or(&0) as isize - *v as isize;
            if m < 0 && sg.is_circular() {
                available.insert(sg, (-m) as usize);
            }
        }

        for (sg, v) in missing {
            if available.get(&sg).unwrap_or(&0) < &v {
                return false;
            }
        }

        true
    }
}

impl subgraphs::Subgraphs for SubgraphsAndRings {
    fn new(g: &Graph) -> Self {
        assert!(g.size() >= 4);
        let atoms = g
            .label_counts()
            .into_iter()
            .map(|(i, c)| {
                let mut g = Graph::with_size(1);
                g.atoms_mut()[0] = i;
                (g, c)
            })
            .collect();

        let subgraphs = subgraphs::get_all(g, 4);

        let rings5 = if g.size() >= 5 {
            subgraphs::combine(g, &subgraphs, 5)
        } else {
            Vec::new()
        };

        let rings6 = if g.size() >= 6 {
            subgraphs::combine(g, &rings5, 6)
        } else {
            Vec::new()
        };

        let mut subgraphs = subgraphs::count_subgraphs(g, &subgraphs, 4);
        let mut rings5 = subgraphs::count_subgraphs(g, &rings5, 5);
        let mut rings6 = subgraphs::count_subgraphs(g, &rings6, 6);

        // Retain only rings in the rings5 and rings6 sets.
        rings5.retain(|k, _| k.is_circular());
        rings6.retain(|k, _| k.is_circular());

        subgraphs.iter_mut().for_each(|(k, v)| {
            if k.is_circular() {
                *v *= 4;
            }
        });

        SubgraphsAndRings {
            atoms,
            subgraphs,
            rings5,
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

        for (k, v) in self.rings5.iter() {
            if other.rings5.get(k).unwrap_or(&0) < v {
                return false;
            }
        }

        // for (k, v) in self.subgraphs.iter() {
        //     if other.subgraphs.get(k).unwrap_or(&0) < v {
        //         return false;
        //     }
        // }

        self.check_for_partials(other)
    }

    fn all_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a Graph>> {
        Box::new(
            self.rings6
                .keys()
                .chain(self.rings5.keys())
                .chain(self.subgraphs.keys())
                .chain(self.atoms.keys()),
        )
    }

    fn with_counts<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&'a Graph, &'a usize)>> {
        Box::new(
            self.rings6
                .iter()
                .chain(self.rings5.iter())
                .chain(self.subgraphs.iter())
                .chain(self.atoms.iter()),
        )
    }

    fn attachable_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a Graph>> {
        Box::new(self.subgraphs.keys())
    }

    fn score(&self) -> usize {
        let has_ring6 = (self.rings6.len() > 0) as usize;
        let has_ring5 = (self.rings5.len() > 0) as usize;
        let has_subgraph = (self.subgraphs.len() > 0) as usize;
        has_ring6 * 3 + has_ring5 * 2 + has_subgraph
    }

    fn amount_of(&self, g: &Graph) -> usize {
        *self.subgraphs.get(g).unwrap_or(&0)
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
