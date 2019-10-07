#![feature(test)]
extern crate test;

use rusqlite::{Connection, NO_PARAMS};

mod prelude;

mod assembly;
use assembly::assemble;

mod attachment;
use attachment::attach;

mod graph;
use graph::Graph;

mod subgraphs;

mod isomorphism;
use isomorphism::are_isomorphic;

#[derive(Debug)]
/// An entry from the SQLite Database of all the pubchem molecules.
struct CompoundEntry {
    pub cid: i64,
    pub structure: String,
    pub is_contiguous: bool,
    pub atoms: i64,
    pub bonds: i64,
}

fn main() {
    let conn = Connection::open("sqlite/pubchem.db").unwrap();
    let sql = "SELECT cid, structure, is_contiguous, n_atoms, n_edges FROM compounds where cid = 2519 LIMIT 1";
    let mut stmt = conn.prepare(&sql).unwrap();
    let iter = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(CompoundEntry {
                cid: row.get(0)?,
                structure: row.get(1)?,
                is_contiguous: row.get(2)?,
                atoms: row.get(3)?,
                bonds: row.get(4)?,
            })
        })
        .unwrap()
        .map(|x| x.unwrap());

    for x in iter {
        let g = graph::Graph::new(x.structure);

        // determine the graphs' subgraphs.
        let sg = subgraphs::get_all(&g, 7);
        let sg = subgraphs::count_subgraphs(&g, &sg, 7);

        // re-assemble the graph
        let g = assemble(sg);
    }
}
