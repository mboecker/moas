use super::State;
use crate::Graph;
use std::collections::HashSet;

struct Run<S, F>
where
    S: Eq,
    F: Fn(Graph) -> S,
{
    /// A set of subgraphs.
    subgraphs: S,

    /// A function to calculate the set of subgraphs of a graph.
    /// This is templated to allow for different sets of subgraphs to be calculated,
    /// like including all atom counts in addition to, for example, 3-subgraphs.
    calc_subgraphs: F,

    /// States that are going to be explored in the next iteration.
    q_active: HashSet<State<S>>,

    /// Already explored states of assembly.
    q_passive: HashSet<State<S>>,
}

impl<S, F> Run<S, F>
where
    S: Eq,
    F: Fn(Graph) -> S,
{
    pub fn new(subgraphs: S, calc_subgraphs: F) -> Run<S, F> {
        let q_active = Self::select_starting_nodes(&subgraphs);
        let q_passive = HashSet::new();

        Run {
            subgraphs,
            calc_subgraphs,
            q_active,
            q_passive,
        }
    }

    fn select_starting_nodes(subgraphs: &S) -> HashSet<State<S>> {
        unimplemented!()
    }

    pub fn assemble(&mut self) -> HashSet<Graph> {
        unimplemented!()
    }
}
