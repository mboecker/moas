use super::State;
use crate::Graph;
use crate::subgraphs::Subgraphs;
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
}

impl<S> Run<S>
where
    S: Subgraphs + Eq + Hash,
{
    pub fn new(subgraphs: S) -> Run<S> {
        let mut q_active = HashSet::new();
        let g = subgraphs.select_starting_graph();
        let sg = S::new(&g);
        let state = State::new(g, sg);
        q_active.insert(state);

        let q_passive = HashSet::new();

        Run {
            subgraphs,
            q_active,
            q_passive,
        }
    }

    pub fn assemble(mut self) -> HashSet<Graph> {
        loop {
            // Assemble current active graphs into new graphs.
            let new_queue = self.iterate();

            // Move active graphs into the passive graphs.
            self.q_passive.extend(self.q_active.drain());

            if new_queue.is_empty() {
                break;
            }
        }

        self.q_passive
            .into_iter()
            .filter(|state| state.is_successful(&self.subgraphs))
            .map(|state| state.g)
            .collect()
    }

    fn iterate(&mut self) -> HashSet<State<S>> {
        self.
            q_active
            .iter()
            .flat_map(|state| {
                // Iterate over all the subgraphs that are still available.
                self.subgraphs
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
                                let g = crate::attachment::graph(&state.g, sg, attachment);
                                let k = sg.size();
                                let used_subgraphs =
                                    subgraphs::count_subgraphs(&g, &subgraphs::subgraphs(&g, k), k);

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
            .collect()
    }
}
