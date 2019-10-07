use crate::Graph;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use crate::subgraphs;

#[derive(Eq,PartialEq)]
pub struct Only4 {
    subgraphs: HashMap<Graph, usize>,
}

impl subgraphs::Subgraphs for Only4 {
    fn new(g: &Graph) -> Self {
        assert!(g.size() > 4);
        let subgraphs = subgraphs::subgraphs(g, 4);
        let subgraphs = subgraphs::count_subgraphs(g, &subgraphs, 4);
        Only4 {
            subgraphs,
        }
    }

    fn select_starting_graph(&self) -> Graph {
        self.subgraphs.keys().next().unwrap().clone()
    }

    fn basic_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=&'a Graph>> {
        Box::new(self.subgraphs.keys())
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
        let subgraphs: BTreeSet<_> = self.subgraphs.iter()
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
