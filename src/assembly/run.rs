use super::State;
use crate::subgraphs::Subgraphs;
use crate::Graph;
use rayon::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;

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
            let new_queue = self.iterate();
            let duration = std::time::Instant::now() - start;
            println!("Starting iteration {} at {}", iter, Utc::now().to_rfc2822());
            println!("Active Queue: {}", self.q_active.len());
            println!("Passive Queue: {}", self.q_passive.len());
            println!("Duration: {:.2}s", duration.as_secs_f64());
            println!();

            // Move active graphs into the passive graphs.
            self.q_passive.extend(self.q_active.drain());

            if new_queue.is_empty() {
                break;
            }

            for x in new_queue.into_iter() {
                if !self.q_passive.contains(&x) {
                    self.q_active.insert(x);
                }
            }

            if self.q_active.is_empty() {
                break;
            }

            let max_score = self.q_active.iter().map(|s| s.used.score()).max().unwrap();
            // let max_interest = self.q_active.iter().map(|s| s.g.is_interesting()).max().unwrap();
            self.q_active.retain(|x| x.used.score() >= max_score);
        }

        let subgraphs = self.subgraphs;
        self.q_passive
            .into_iter()
            .filter(|state| state.is_successful(&subgraphs))
            .map(|state| state.g)
            .collect()
    }

    fn iterate(&self) -> HashSet<State<S>> {
        self.q_active
            .par_iter()
            .flat_map(|state| self.explore_state(state))
            .collect()
    }

    /// Explores one of the current states by trying to attach unused subgraphs.
    fn explore_state(&self, state: &State<S>) -> impl ParallelIterator<Item = State<S>> {
        // Iterate over all the subgraphs that are still available.
        self.subgraphs
            .attachable_subgraphs()
            .flat_map(|sg| {
                // Iterate over the different options to attach this subgraph.
                crate::attach(&state.g, sg)
                    .into_iter()
                    .filter_map(move |attachment| {
                        // Actually perform the attachment and create a graph.
                        let g = crate::attachment::perform(&state.g, sg, attachment);

                        // Rule out graphs with too many atom bonds.
                        for i in 0..g.size() {
                            let s: u8 = (0..g.size()).map(|j| g.bonds().get(i, j)).sum();
                            if s > crate::get_max_bonds_for_element(g.atoms()[i]) {
                                return None;
                            }
                        }

                        crate::STATISTICS
                            .lock()
                            .unwrap()
                            .graph_proposed(self.current_iter);

                        // Calculate newly used subgraphs.
                        let used_subgraphs = S::new(&g);

                        // Check, if by attaching this subgraph in this way,
                        // we used more subgraphs than we're allowed to.
                        if used_subgraphs.is_subset_of(&self.subgraphs) {
                            crate::STATISTICS
                                .lock()
                                .unwrap()
                                .valid_graph_proposed(self.current_iter);
                            Some(State::new(g, used_subgraphs))
                        } else {
                            crate::STATISTICS.lock().unwrap().report_invalid_graph(g);
                            None
                        }
                    })
            })
            .collect::<HashSet<_>>()
            .into_par_iter()
    }
}
