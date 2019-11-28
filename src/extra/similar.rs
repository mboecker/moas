use crate::Graph;

pub struct Similar {
    // Partitioned into buckets containing the node indices
    // |---|----|-|
    // 111122222334
    // 057946...
    node_indices: Vec<usize>,

    // Index into node_indices
    bucket_starts: Vec<(usize, (usize, usize))>,
}

impl Similar {
    pub fn new(g: &Graph) -> Similar {
        let mut node_indices: Vec<_> = (0..g.size()).collect();
        node_indices.sort_by_key(|i| g.atoms()[*i]);

        let mut bucket_starts = Vec::new();

        let mut last_start = 0;
        let mut current_label = g.atoms()[0];
        for i in 1..g.size() {
            let node_index = node_indices[i];
            let node_label = g.atoms()[node_index];

            if node_label != current_label {
                // Bucket complete, insert its bounds into the index.
                bucket_starts.push((current_label, (last_start, i)));

                // Update helper variables for next label.
                last_start = i;
                current_label = node_label;
            }
        }
        bucket_starts.push((current_label, (last_start, g.size())));

        Similar {
            node_indices,
            bucket_starts
        }
    }

    pub fn find<'a>(&'a self, label: usize) -> &'a [usize] {
        let idx = self.bucket_starts.binary_search_by_key(&label, |(key, _)| *key);
        if let Ok(idx) = idx {
            let (found_label, (from, to)) = self.bucket_starts[idx];
            assert_eq!(found_label, label);
            &self.node_indices[from..to]
        } else {
            &self.node_indices[0..0]
        }
    }
}
