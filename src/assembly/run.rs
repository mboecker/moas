use super::State;
use crate::subgraphs::Subgraphs;
use crate::Graph;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

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

    /// Optionally, give a size the current queue can't pass.
    max_queue_size: Option<usize>,
}

impl<S> Run<S>
where
    S: Subgraphs + Eq + Hash + Send + Sync + Debug,
{
    pub fn new(subgraphs: S, max_queue_size: Option<usize>) -> Run<S> {
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
            max_queue_size,
        }
    }

    #[cfg(test)]
    pub fn with_starting_graph(subgraphs: S, g: Graph) -> Run<S> {
        // Generate inital state.
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
            max_queue_size: None,
        }
    }

    pub fn assemble(mut self) -> Option<HashSet<Graph>> {
        for iter in 0.. {
            use chrono::Utc;
            if crate::statistics::trace_enabled() {
                let filename = format!("trace/iter_{}.dot", iter);
                let f = std::fs::File::create(filename).unwrap();
                crate::prelude::dump_set(f, self.q_active.iter().map(|s| &s.g)).unwrap();
            }

            // Assemble current active graphs into new graphs.
            self.current_iter = iter;
            let start = std::time::Instant::now();

            if crate::statistics::trace_enabled() {
                println!("Starting iteration {} at {}", iter, Utc::now().to_rfc2822());
                println!(
                    "Active Queue: {} ({:.2} KiB)",
                    self.q_active.len(),
                    self.q_active
                        .iter()
                        .map(|s| s.g.bytes() as f64 / 1024f64)
                        .sum::<f64>()
                );
                println!(
                    "Passive Queue: {} ({:.2} KiB)",
                    self.q_passive.len(),
                    self.q_passive
                        .iter()
                        .map(|s| s.g.bytes() as f64 / 1024f64)
                        .sum::<f64>()
                );
            }

            let new_queue = self.iterate();

            if let Some(max_queue_size) = self.max_queue_size {
                if new_queue.len() >= max_queue_size {
                    return None;
                }
            }

            if crate::statistics::trace_enabled() {
                let duration = std::time::Instant::now() - start;
                println!("Duration: {:.2}s", duration.as_secs_f64());
                println!();
            }

            // move things from q_active to q_passive if theyre less or equal to the min.
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
        }

        let subgraphs = self.subgraphs;

        #[cfg(feature = "parallel")]
        let result: HashSet<Graph> = self
            .q_passive
            .into_par_iter()
            .filter(|state| state.is_successful(&subgraphs))
            .map(|state| state.g)
            .collect();

        #[cfg(not(feature = "parallel"))]
        let result: HashSet<Graph> = self
            .q_passive
            .into_iter()
            .filter(|state| state.is_successful(&subgraphs))
            .map(|state| state.g)
            .collect();

        if crate::statistics::trace_enabled() {
            println!(
                "Size of resulting set: {:.2} KiB",
                result
                    .iter()
                    .map(|g| g.bytes() as f64 / 1024f64)
                    .sum::<f64>()
            );
        }

        Some(result)
    }

    fn iterate(&self) -> HashSet<State<S>> {
        #[cfg(feature = "parallel")]
        return self
            .q_active
            .par_iter()
            .map(|state| self.explore_state(state))
            .reduce(
                || HashSet::new(),
                |mut a, b| {
                    a.extend(b);
                    a
                },
            );

        #[cfg(not(feature = "parallel"))]
        {
            // let min = self.q_active.iter().min().unwrap();
            return self
                .q_active
                .iter()
                // .filter(|s| s <= &min)
                .map(|state| self.explore_state(state))
                .flatten()
                .collect();
        }
    }

    /// Explores one of the current states by trying to attach unused subgraphs.
    fn explore_state(&self, state: &State<S>) -> HashSet<State<S>> {
        let anchor = state
            .g
            .atoms()
            .iter()
            .enumerate()
            .filter(|(i, a)| {
                state
                    .g
                    .neighbors(*i)
                    .map(|j| state.g.bonds().get(*i, j))
                    .sum::<u8>()
                    < crate::Atoms::max_bonds(**a)
            })
            .map(|(i, _)| i)
            .next();

        if anchor.is_none() {
            return HashSet::new();
        }

        let anchor = anchor.unwrap();

        // Iterate over all the subgraphs that are still available.
        self.subgraphs
            .attachable_subgraphs()
            .enumerate()
            .filter_map(|(_idx, sg)| {
                // Skip this subgraph if we cant legally use it again.
                if state.used.amount_of(sg) >= self.subgraphs.amount_of(sg) {
                    return None;
                }

                // Iterate over the different options to attach this subgraph.
                Some(
                    crate::attach(&state.g, sg)
                        .into_iter()
                        .filter_map(move |attachment| {
                            let new_node = attachment.new_node;

                            // Find the node(s) in sg that the new node is attached to.
                            let attached_node: Option<Vec<usize>> = attachment
                                .new_node
                                .map(|new_node| sg.neighbors(new_node).collect());

                            let mapping: BTreeMap<_, _> = attachment.into();

                            // check if the attached_node already contained a similar new_node.
                            // this means checking if the combination of atom label and number of bonds is already present in the bitset.
                            if let Some(attached_node) = attached_node {
                                // Only the anchor can gain neighbors.
                                if attached_node
                                    .iter()
                                    .map(|sgi| mapping[sgi])
                                    .all(|gi| anchor != gi)
                                {
                                    return None;
                                }

                                for sgi in attached_node {
                                    let gi = mapping[&sgi];

                                    // If this atom cannot have another neighbor, skip this.
                                    if state
                                        .g
                                        .neighbors(gi)
                                        .map(|j| state.g.bonds().get(gi, j))
                                        .sum::<u8>()
                                        >= crate::Atoms::max_bonds(state.g.atoms()[gi])
                                    {
                                        // println!("this would violate bonding rules, so we skip him.");
                                        return None;
                                    }
                                }
                            }

                            // Actually perform the attachment and create a graph.
                            let g = crate::attachment::perform(&state.g, &sg, mapping, new_node);

                            // Rule out graphs with too many atom bonds.
                            for i in 0..g.size() {
                                let s: u8 = (0..g.size()).map(|j| g.bonds().get(i, j)).sum();
                                let e = g.atoms()[i];
                                if s > crate::Atoms::max_bonds(e) {
                                    // println!("this would violate bonding rules.");
                                    return None;
                                }
                            }

                            // Calculate newly used subgraphs.
                            let used_subgraphs = S::new(&g);

                            // Check, if by attaching this subgraph in this way,
                            // we used more subgraphs than we're allowed to.
                            if used_subgraphs.is_subset_of(&self.subgraphs) {
                                // println!("{}", idx);
                                Some(State::new(g, used_subgraphs))
                            } else {
                                use std::hash::Hasher;
                                use std::io::Write;
                                let mut hasher =
                                    std::collections::hash_map::DefaultHasher::default();
                                sg.hash(&mut hasher);
                                let hash = hasher.finish();

                                if crate::statistics::trace_enabled() {
                                    let filename = format!("trace/sgs_{}_invalid.dot", hash);
                                    let mut f = std::fs::File::create(filename).unwrap();
                                    writeln!(&mut f, "graph invalid {{").unwrap();
                                    sg.dump(&mut f, 0, false).unwrap();
                                    g.dump(&mut f, sg.size(), true).unwrap();
                                    writeln!(&mut f, "}}").unwrap();
                                }

                                if crate::statistics::trace_enabled() {
                                    let filename = format!("trace/sgs_{}_used.dot", hash);
                                    let f = std::fs::File::create(filename).unwrap();
                                    crate::prelude::dump_map(f, used_subgraphs.with_counts())
                                        .unwrap();
                                }

                                if crate::statistics::trace_enabled() {
                                    let filename = format!("trace/sgs_{}_avail.dot", hash);
                                    let f = std::fs::File::create(filename).unwrap();
                                    crate::prelude::dump_map(f, self.subgraphs.with_counts())
                                        .unwrap();
                                }

                                // println!("the subgraphs just dont add up.");
                                None
                            }
                        }),
                )
            })
            .flatten()
            .collect::<HashSet<_>>()
    }
}
