use super::State;
use crate::attach;

use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use flamer::flame;

use crate::subgraphs;
use crate::Graph;

#[derive(Debug)]
struct Queue {
    pub active: HashSet<State>,
    pub passive: HashSet<State>,
}

impl Default for Queue {
    fn default() -> Queue {
        let active = HashSet::default();
        let passive = HashSet::default();
        Queue { active, passive }
    }
}

#[flame]
pub fn assemble(subgraphs: HashMap<Graph, usize>) -> HashSet<Graph> {
    let mut q = Queue::default();

    let mut blueprint = subgraphs.clone();

    for sg in subgraphs.keys() {
        let mut sub_subgraphs = blueprint.clone();
        *sub_subgraphs.get_mut(&sg).unwrap() -= 1;
        if sub_subgraphs[&sg] == 0 {
            sub_subgraphs.remove(&sg);
        }
        q.active.insert(State::new(sg.clone(), sub_subgraphs));
    }

    inner(&subgraphs, &mut q);
    q.passive
        .into_iter()
        .filter(|state| state.is_successful(&subgraphs))
        .map(|state| state.g)
        .collect()
}

fn inner(subgraphs: &HashMap<Graph, usize>, queue: &mut Queue) {
    loop {
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
                        attach(&state.g, sg).into_iter().map(move |attachment| {
                            let g = if attachment.new_node.is_none() {
                                // only a new edge has been added
                                let mapping: HashMap<_, _> =
                                    attachment.mapping.iter().cloned().collect();
                                let mut g = state.g.clone();
                                for i in 0..sg.size() {
                                    for j in 0..sg.size() {
                                        let mi = mapping[&i];
                                        let mj = mapping[&j];
                                        *g.bonds_mut().get_mut(mi, mj) = *sg.bonds().get(i, j);
                                    }
                                }
                                g
                            } else {
                                // a new node has been added
                                // println!("adding new node");
                                let mapping: HashMap<_, _> =
                                    attachment.mapping.iter().cloned().collect();
                                let new_node_sg_id = attachment.new_node.unwrap();
                                let mut g = state.g.clone_with_extraspace(1);

                                let j = new_node_sg_id;
                                let mj = g.size() - 1;
                                g.atoms_mut()[mj] = sg.atoms()[j];

                                for i in 0..sg.size() {
                                    if let Some(&mi) = mapping.get(&i) {
                                        let v = *sg.bonds().get(i, j);
                                        *g.bonds_mut().get_mut(mi, mj) = v;
                                        *g.bonds_mut().get_mut(mj, mi) = v;
                                    }
                                }
                                g
                            };

                            let k = sg.size();
                            let used_subgraphs =
                                subgraphs::count_subgraphs(&g, &subgraphs::subgraphs(&g, k), k);

                            State::new(g, used_subgraphs)
                        })
                    })
                    .collect::<HashSet<_>>()
                    .into_par_iter()
            })
            .collect();

        queue.passive.extend(queue.active.drain());
        for state in new_queue.into_iter() {
            if !queue.passive.contains(&state) {
                queue.active.insert(state);
            }
        }

        println!(
            "{:?} active, {:?} passive states",
            queue.active.len(),
            queue.passive.len()
        );

        if queue.active.is_empty() {
            return;
        }
    }
}
