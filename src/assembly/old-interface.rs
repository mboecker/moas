use super::State;
use crate::attach;

use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::subgraphs;
use crate::Graph;

#[derive(Debug)]
struct Queue {
    pub active: HashSet<State<HashMap<Graph, usize>>>,
    pub passive: HashSet<State<HashMap<Graph, usize>>>,
}

impl Default for Queue {
    fn default() -> Queue {
        let active = HashSet::default();
        let passive = HashSet::default();
        Queue { active, passive }
    }
}

pub fn assemble(subgraphs: HashMap<Graph, usize>) -> HashSet<Graph> {
    // print out the computed subgraphs.
    {
        let filename = "trace/subgraphs.dot";
        let f = std::fs::File::create(filename).unwrap();
        crate::prelude::dump_set(f, subgraphs.keys()).unwrap();
    }

    let mut q = Queue::default();
    let k = subgraphs.keys().next().unwrap().size();
    let g = subgraphs
        .keys()
        .max_by_key(|g| g.is_interesting())
        .unwrap()
        .clone();
    let used = subgraphs::count_subgraphs(&g, &subgraphs::get_all(&g, k), k);
    q.active.insert(State::new(g, used));

    inner(&subgraphs, &mut q);
    q.passive
        .into_iter()
        .filter(|state| state.is_successful(&subgraphs))
        .map(|state| state.g)
        .collect()
}

fn inner(subgraphs: &HashMap<Graph, usize>, queue: &mut Queue) {
    for iter in 0.. {
        println!(
            "iteration {}: {} active, {} passive states",
            iter,
            queue.active.len(),
            queue.passive.len()
        );

        {
            let filename = format!("trace/iter_{}.dot", iter);
            let f = std::fs::File::create(filename).unwrap();
            crate::prelude::dump_set(f, queue.active.iter().map(|s| &s.g)).unwrap();
        }

        // Iterate over active states. These are states that should be further pursued.
        let new_queue: HashSet<_> = queue
            .active
            .par_iter()
            .flat_map(|state| {
                // Iterate over all the subgraphs that are still available.
                subgraphs
                    .iter()
                    .filter(|(k, v)| {
                        // only consider subgraphs that are available at least once.
                        v.saturating_sub(*state.used.get(k).unwrap_or(&0)) > 0
                    })
                    .flat_map(|(sg, _)| {
                        // Iterate over the different options to attach this subgraph.
                        attach(&state.g, sg)
                            .into_iter()
                            .filter_map(move |attachment| {
                                let g = crate::attachment::perform(&state.g, sg, attachment);
                                let k = sg.size();
                                let used_subgraphs =
                                    subgraphs::count_subgraphs(&g, &subgraphs::get_all(&g, k), k);

                                for (k, v) in &used_subgraphs {
                                    if subgraphs.get(k).unwrap_or(&0) < v {
                                        return None;
                                    }
                                }

                                Some(State::new(g, used_subgraphs))
                            })
                    })
                    .collect::<HashSet<_>>()
                    .into_par_iter()
            })
            .collect();

        queue.passive.extend(queue.active.drain());

        if new_queue.is_empty() {
            return;
        }

        let max_interest = new_queue
            .iter()
            .map(|s| s.g.is_interesting())
            .max()
            .unwrap();
        for state in new_queue.into_iter() {
            if !queue.passive.contains(&state) {
                if state.g.is_interesting() == max_interest {
                    queue.active.insert(state);
                } else {
                    queue.passive.insert(state);
                }
            }
        }
    }
}
