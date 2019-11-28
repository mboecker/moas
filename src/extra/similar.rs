use crate::Graph;
use std::collections::HashMap;

pub struct Similar {
    // // Partitioned into buckets containing the node indices
    // // |---|----|-|
    // // 111122222334
    // // 057946...
    // node_indices: Vec<usize>,

    // // Index into node_indices
    // bucket_starts: Vec<usize>,
    old_strat: HashMap<usize, Vec<usize>>,
}

impl Similar {
    pub fn new(g: &Graph) -> Similar {
        // Build index
        let mut old_strat: HashMap<_, Vec<_>> = HashMap::new();
        for i in 0..g.size() {
            old_strat.entry(g.atoms()[i]).or_default().push(i);
        }

        Similar { old_strat }
    }

    pub fn find<'a>(&'a self, label: usize) -> impl 'a + Iterator<Item = usize> {
        self.old_strat
            .get(&label)
            .into_iter()
            .flat_map(|x| x.iter().cloned())
    }
}
