use crate::subgraphs::Subgraphs;
use crate::Graph;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

mod state;
use self::state::State;

mod bitset;
mod context;
pub(crate) mod run;
pub use self::bitset::BitSet;

#[cfg(test)]
mod test;

/// This is this project's main entry point.
/// Given a multiset of subgraphs, this function returns a set of fully assembled molecular graphs.
/// Their subgraphs will be equal to the given set of subgraphs.
pub fn assemble<S: Subgraphs + Eq + Hash + Send + Sync + Debug>(
    s: S,
    max_queue_size: Option<usize>,
) -> Option<HashSet<Graph>> {
    let r = self::run::Run::new(s, max_queue_size);
    r.assemble()
}
