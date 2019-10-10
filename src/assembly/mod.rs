use crate::Graph;
use crate::subgraphs::Subgraphs;
use std::collections::HashSet;
use std::hash::Hash;

mod state;
use self::state::State;

mod context;
mod run;

#[cfg(test)]
mod test;

pub fn assemble<S: Subgraphs + Eq + Hash>(s: S) -> HashSet<Graph> {
    let r = self::run::Run::new(s);
    r.assemble()
}