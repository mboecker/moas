use crate::subgraphs;
use crate::Graph;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Eq, PartialEq, Debug)]
pub struct Only7 {
    atoms: HashMap<Graph, usize>,
    subgraphs: HashMap<Graph, usize>,
}

impl Only7 {
    fn check_for_partials(&self, other: &Self) -> bool {
        let mut missing: HashMap<&Graph, usize> = HashMap::new();
        let mut available: HashMap<&Graph, usize> = HashMap::new();

        // Scan for subgraphs that are still missing.
        for (sg, v) in self.subgraphs.iter() {
            let m = *other.subgraphs.get(sg).unwrap_or(&0) as isize - *v as isize;
            if m < 0 {
                missing.insert(sg, (-m) as usize);
            }
        }

        // Scan for subgraphs that are still available.
        for (sg, v) in other.subgraphs.iter() {
            let m = *self.subgraphs.get(sg).unwrap_or(&0) as isize - *v as isize;
            if m < 0 {
                available.insert(sg, (-m) as usize);
            }
        }

        for (sg, v) in missing {
            for _ in 0..v {
                let mut used_sg = None;
                for (avail_sg, v2) in &available {
                    if v2 > &0 {
                        use itertools::Itertools;
                        // Try removing edges from avail_sg and look if its isomorphic to sg.
                        for (i, j) in (0..avail_sg.size())
                            .tuple_combinations::<(_, _)>()
                            .filter(|(i, j)| i < j && avail_sg.bonds().get(*i, *j) > &0)
                        {
                            let mut tmp_graph: Graph = (*avail_sg).clone();
                            let v: u8 = *tmp_graph.bonds().get(i, j);
                            *tmp_graph.bonds_mut().get_mut(i, j) = 0;
                            *tmp_graph.bonds_mut().get_mut(j, i) = 0;

                            if tmp_graph.is_contiguous() && &tmp_graph == sg {
                                *tmp_graph.bonds_mut().get_mut(i, j) = v;
                                *tmp_graph.bonds_mut().get_mut(j, i) = v;
                                used_sg = Some(tmp_graph);
                                break;
                            }
                        }
                    }
                }

                if let Some(used_sg) = used_sg {
                    *available.get_mut(&used_sg).unwrap() -= 1;
                } else {
                    return false;
                }
            }
        }

        true
    }
}

impl subgraphs::Subgraphs for Only7 {
    fn new(g: &Graph) -> Self {
        assert!(g.size() >= 7);

        let atoms = g
            .label_counts()
            .into_iter()
            .map(|(i, c)| {
                let mut g = Graph::with_size(1);
                g.atoms_mut()[0] = i;
                (g, c)
            })
            .collect();

        let subgraphs = subgraphs::get_all(g, 7);
        let subgraphs = subgraphs::count_subgraphs(g, &subgraphs, 7);

        Only7 { atoms, subgraphs }
    }

    fn select_starting_graph(&self) -> Graph {
        self.subgraphs.keys().next().unwrap().clone()
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

        self.check_for_partials(other)
    }

    fn all_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a Graph>> {
        Box::new(self.subgraphs.keys().chain(self.atoms.keys()))
    }

    fn with_counts<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&'a Graph, &'a usize)>> {
        Box::new(self.subgraphs.iter().chain(self.atoms.iter()))
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
        self.atoms.values().sum()
    }
}

impl Hash for Only7 {
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
