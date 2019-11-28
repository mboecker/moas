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
        // An array of node ids, sorted by their label.
        // Stably sorted, so the node ids in a label-bucket are in ascending order.
        let mut node_indices: Vec<_> = (0..g.size()).collect();
        node_indices.sort_by_key(|i| g.atoms()[*i]);

        // Build an index into the array.
        let mut bucket_starts = Vec::new();

        // Special case: start with the index and label of the first node.
        let mut last_start = 0;
        let mut current_label = g.atoms()[node_indices[0]];

        // Start on second node, because the first one has already been handled.
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

        // Add the last, unfinished bucket to the index.
        bucket_starts.push((current_label, (last_start, g.size())));

        if cfg!(debug) {
            for (label, (i, j)) in &bucket_starts {
                for k in *i..*j {
                    let idx = node_indices[k];
                    debug_assert_eq!(label, &g.atoms()[idx], "bucket {} was wrong: {} ({}) in {}..{} had a wrong label", label, k, idx, i, j);
                }
            }
        }

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
