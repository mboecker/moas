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

        let subgraphs = self.subgraphs;
        self.q_passive
            .into_iter()
            .filter(|state| state.is_successful(&subgraphs))
            .map(|state| state.g)
            .collect()
    }

    fn iterate(&self) -> HashSet<State<S>> {
        self.
            q_active
            .iter()
            .flat_map(|state| {
                self.explore_state(state)
            })
            .collect()
    }

    /// Explores one of the current states by trying to attach unused subgraphs.
    fn explore_state(&self, state: &State<S>) -> impl Iterator<Item=State<S>> {
        // Iterate over all the subgraphs that are still available.
        self.subgraphs
            .basic_subgraphs()
            .flat_map(|sg| {
                // Iterate over the different options to attach this subgraph.
                crate::attach(&state.g, sg)
                    .into_iter()
                    .filter_map(move |attachment| {
                        let g = crate::attachment::perform(&state.g, sg, attachment);
                        let used_subgraphs = S::new(&g);

                        if used_subgraphs.is_subset_of(&self.subgraphs) {
                            Some(State::new(g, used_subgraphs))
                        } else {
                            None
                        }
                    })
            })
            .collect::<HashSet<_>>()
            .into_iter()
    }
}
