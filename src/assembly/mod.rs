use crate::subgraphs::Subgraphs;
use crate::Graph;
use std::collections::HashSet;
use std::hash::Hash;

mod state;
use self::state::State;

mod bitset;
mod context;
mod run;
pub use self::bitset::BitSet;

#[cfg(test)]
mod test;

/// This is this project's main entry point.
/// Given a multiset of subgraphs, this function returns a set of fully assembled molecular graphs.
/// Their subgraphs will be equal to the given set of subgraphs.
pub fn assemble<S: Subgraphs + Eq + Hash + Send + Sync>(s: S) -> HashSet<Graph> {
    let r = self::run::Run::new(s);
    r.assemble()
}
