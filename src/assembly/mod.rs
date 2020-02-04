use crate::subgraphs::Subgraphs;
use crate::Graph;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Duration;
use std::time::Instant;

mod state;
use self::state::State;

// mod bitset;
mod context;
pub(crate) mod run;
// pub use self::bitset::BitSet;

mod tree_statistics;
pub use self::tree_statistics::TreeStatistics;

#[cfg(test)]
mod test;

/// This is this project's main entry point.
/// Given a multiset of subgraphs, this function returns a set of fully assembled molecular graphs.
/// Their subgraphs will be equal to the given set of subgraphs.
pub fn assemble<S: Subgraphs + Eq + Hash + Send + Sync + Debug>(
    s: S,
    max_queue_size: Option<usize>,
    time_limit: Option<Duration>,
) -> Option<(HashSet<Graph>, TreeStatistics)> {
    let time_limit = time_limit.map(|d| Instant::now() + d);
    let r = self::run::Run::new(s, max_queue_size, time_limit);
    r.assemble()
}
