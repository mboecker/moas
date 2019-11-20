use super::mapping::Mapping;
use super::result::Result;
use crate::graph::Graph;
use std::collections::HashSet;

struct Queue {
    pub active: HashSet<Mapping>,
    pub passive: HashSet<Mapping>,
}

impl Default for Queue {
    fn default() -> Queue {
        let active = HashSet::default();
        let passive = HashSet::default();
        Queue { active, passive }
    }
}

pub fn attach(g: &Graph, sg: &Graph) -> HashSet<Result> {
    let mut q = Queue::default();

    let similar = crate::extra::Similar::new(g);

    for (i, l) in sg.atoms().iter().enumerate() {
        for j in similar.find(*l) {
            q.active.insert(Mapping::new(i, j));
        }
    }

    inner(g, sg, &mut q);
    assert_eq!(q.active.len(), 0);
    q.passive
        .into_iter()
        .filter(|m| m.mapping.len() >= sg.size() - 1)
        .map(|m| Result::new(sg, m.mapping))
        .collect()
}

fn inner(g: &Graph, sg: &Graph, queue: &mut Queue) {
    loop {
        let mut new_queue = HashSet::new();
        for node in queue.active.iter() {
            // println!("{:?}", node);

            let mapped_sg_nodes: HashSet<_> = node.mapping.iter().map(|(a, _)| a).collect();
            let used_g_nodes: HashSet<_> = node.mapping.iter().map(|(_, a)| a).collect();
            let unmapped_sg_nodes: Vec<_> = (0..sg.size())
                .filter(|i| !mapped_sg_nodes.contains(i))
                .collect();
            let free_g_nodes: Vec<_> = (0..g.size())
                .filter(|i| !used_g_nodes.contains(i))
                .collect();

            for (i, mapped_i) in &node.mapping {
                let neighbors: Vec<_> = sg
                    .neighbors(*i)
                    .filter(|j| unmapped_sg_nodes.contains(j))
                    .collect();

                for j in neighbors {
                    // There is no mapping for edge (i,j) from SG.
                    assert!(sg.bonds().get(*i, j) > &0);

                    // Try to find an atom in G that is attached to the isomorphic pendant of i (from sg) in g.
                    // This is done by enumerating every edge in G from m[i] for every atom
                    // that is connected to m[i] by n edges and has the correct label.
                    let n = *sg.bonds().get(*i, j);
                    let label = sg.atoms()[j];

                    if unmapped_sg_nodes.len() > 1 {
                        // Try different matching nodes in the big graph g.
                        // This means that the new node in the newly added edge is mapped to `candidate` in g.
                        for candidate in g.neighbors(*mapped_i).filter(|&candidate| {
                            candidate > node.min
                                && free_g_nodes.contains(&candidate)
                                && g.atoms()[candidate] == label
                                && *g.bonds().get(*mapped_i, candidate) == n
                        }) {
                            // Add a new mapping (j, candidate) to a child node.
                            let mut mapping = node.clone();
                            mapping.add(j, candidate);
                            new_queue.insert(mapping);
                        }
                    } else {
                        // Try different matching nodes in the big graph g.
                        // This means that the new node in the newly added edge is mapped to `candidate` in g.
                        for candidate in free_g_nodes.iter().filter(|i| g.atoms()[**i] == label) {
                            if g.bonds().get(*mapped_i, *candidate) == &0 && sg.bonds().get(*i, j) != &0 {
                                // Add a new mapping (j, candidate) to a child node.
                                let mut mapping = node.clone();
                                mapping.add(j, *candidate);
                                new_queue.insert(mapping);
                            }
                        }
                    }
                }
            }
        }

        queue.passive.extend(queue.active.drain());
        for node in new_queue.into_iter() {
            if !queue.passive.contains(&node) {
                queue.active.insert(node);
            }
        }

        if queue.active.is_empty() {
            return;
        }
    }
}
