use super::State;
use crate::subgraphs::Subgraphs;
use crate::Graph;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

pub struct Run<S>
where
    S: Eq,
{
    /// A set of subgraphs.
    subgraphs: S,

    /// States that are going to be explored in the next iteration.
    q_active: HashSet<State<S>>,

    /// Already explored states of assembly.
    q_passive: HashSet<State<S>>,

    /// The current iteration number.
    current_iter: usize,
}

impl<S> Run<S>
where
    S: Subgraphs + Eq + Hash + Send + Sync,
{
    pub fn new(subgraphs: S) -> Run<S> {
        // Generate inital state.
        let g = subgraphs.select_starting_graph();
        let sg = S::new(&g);
        let state = State::new(g, sg);

        let q_passive = HashSet::new();
        let mut q_active = HashSet::new();
        q_active.insert(state);

        Run {
            subgraphs,
            q_active,
            q_passive,
            current_iter: 0,
        }
    }

    pub fn assemble(mut self) -> HashSet<Graph> {
        for iter in 0.. {
            use chrono::Utc;
            {
                let filename = format!("trace/iter_{}.dot", iter);
                let f = std::fs::File::create(filename).unwrap();
                crate::prelude::dump_set(f, self.q_active.iter().map(|s| &s.g)).unwrap();
            }

            // Assemble current active graphs into new graphs.
            self.current_iter = iter;
            let start = std::time::Instant::now();

            println!("Starting iteration {} at {}", iter, Utc::now().to_rfc2822());
            println!("Active Queue: {}", self.q_active.len());
            println!("Passive Queue: {}", self.q_passive.len());

            let new_queue = self.iterate();
            let duration = std::time::Instant::now() - start;
            println!("Duration: {:.2}s", duration.as_secs_f64());
            println!();

            // Move active graphs into the passive graphs.
            self.q_passive.extend(self.q_active.drain());

            if new_queue.is_empty() {
                break;
            }

            // let max_score = new_queue.iter().map(|s| s.used.score()).max().unwrap();

            for x in new_queue.into_iter() {
                if
                /*x.used.score() >= max_score &&*/
                !self.q_passive.contains(&x) {
                    self.q_active.insert(x);
                }

                // if self.q_active.len() >= 30 {
                //     break;
                // }
            }

            if self.q_active.is_empty() {
                break;
            }
        }

        println!("final selection");

        let subgraphs = self.subgraphs;
        self.q_passive
            .into_iter()
            .filter(|state| state.is_successful(&subgraphs))
            .map(|state| state.g)
            .collect()
    }

    fn iterate(&self) -> HashSet<State<S>> {
        self.q_active
            .iter()
            .map(|state| self.explore_state(state))
            .flatten()
            .collect()
    }

    /// Explores one of the current states by trying to attach unused subgraphs.
    fn explore_state(&self, state: &State<S>) -> HashSet<State<S>> {
        use super::BitSet;

        // A data structure to keep track of already explored added-subgraphs in this state.
        let attached_nodes: Vec<BitSet> = (0..state.g.size()).map(|_| BitSet::default()).collect();
        let attached_nodes = Rc::new(RefCell::new(attached_nodes));

        let anchor = state
            .g
            .atoms()
            .iter()
            .enumerate()
            .filter(|(i, a)| {
                state.g.neighbors(*i).map(|j| state.g.bonds().get(*i,j)).sum::<u8>() < crate::get_max_bonds_for_element(**a)
            })
            .map(|(i, _)| i)
            .min();

        if anchor.is_none() {
            return HashSet::new();
        }

        let anchor = anchor.unwrap();

        // Iterate over all the subgraphs that are still available.
        self.subgraphs
            .attachable_subgraphs()
            .filter_map(|sg| {
                if state.used.amount_of(sg) >= self.subgraphs.amount_of(sg) {
                    return None;
                }

                // explicitly capture this
                let attached_nodes = attached_nodes.clone();

                // Iterate over the different options to attach this subgraph.
                Some(
                    crate::attach(&state.g, sg)
                        .into_iter()
                        .filter_map(move |attachment| {
                            let new_node = attachment.new_node;

                            // Find the node in sg that the new node is attached to.
                            let attached_node: Option<usize> = attachment
                                .new_node
                                .map(|new_node| sg.neighbors(new_node).next().unwrap());

                            let mapping: BTreeMap<_, _> = attachment.into();

                            // check if the attached_node already contained a similar new_node.
                            // this means checking if the combination of atom label and number of bonds is already present in the bitset.
                            let node_attachment_information: Option<_> =
                                if let Some(sgi) = attached_node {
                                    let gi = mapping[&sgi];
                                    let new_node = new_node.unwrap();
                                    let n_bonds = *sg.bonds().get(sgi, new_node);
                                    let label = sg.atoms()[new_node] as u8;

                                    if gi != anchor {
                                        return None;
                                    }
                                    if attached_nodes.borrow()[gi].is_set(label, n_bonds) {
                                        return None;
                                    }

                                    Some((gi, label, n_bonds))
                                } else {
                                    None
                                };

                            // Actually perform the attachment and create a graph.
                            let g = crate::attachment::perform(&state.g, &sg, mapping, new_node);

                            crate::STATISTICS
                                .lock()
                                .unwrap()
                                .graph_proposed(self.current_iter);

                            // Rule out graphs with too many atom bonds.
                            for i in 0..g.size() {
                                let s: u8 = (0..g.size()).map(|j| g.bonds().get(i, j)).sum();
                                if s > crate::get_max_bonds_for_element(g.atoms()[i]) {
                                    return None;
                                }
                            }

                            // Calculate newly used subgraphs.
                            let used_subgraphs = S::new(&g);

                            // Check, if by attaching this subgraph in this way,
                            // we used more subgraphs than we're allowed to.
                            if used_subgraphs.is_subset_of(&self.subgraphs) {
                                crate::STATISTICS
                                    .lock()
                                    .unwrap()
                                    .valid_graph_proposed(self.current_iter);

                                // mark this attachment option
                                if let Some((gi, label, n_bonds)) = node_attachment_information {
                                    attached_nodes.borrow_mut()[gi].set_flag(label, n_bonds);
                                }

                                Some(State::new(g, used_subgraphs))
                            } else {
                                crate::STATISTICS.lock().unwrap().report_invalid_graph(g);
                                None
                            }
                        }),
                )
            })
            .flatten()
            .collect::<HashSet<_>>()
    }
}
