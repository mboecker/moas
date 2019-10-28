#![feature(test)]
extern crate test;

use rusqlite::{Connection, NO_PARAMS};

mod assembly;
mod attachment;
mod extra;
mod graph;
mod isomorphism;
mod prelude;
mod statistics;
mod subgraphs;

use assembly::assemble;
use attachment::attach;
use graph::Graph;
use isomorphism::are_isomorphic;
use statistics::STATISTICS;

pub fn get_max_bonds_for_element(a: usize) -> u8 {
    match a {
        1 => 1,
        6 => 4,
        7 => 3,
        8 => 2,
        _ => 4,
    }
}

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
    let cid = 2519;
    let sql = format!("SELECT cid, structure, is_contiguous, n_atoms, n_edges FROM compounds where cid = {} LIMIT 1", cid);
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
        use crate::subgraphs::Subgraphs;

        let g = graph::Graph::new(x.structure);

        {
            use std::io::Write;
            let filename = "trace/original.dot";
            let mut f = std::fs::File::create(filename).unwrap();
            writeln!(&mut f, "graph g {{").unwrap();
            g.dump(&mut f, 0, true).unwrap();
            writeln!(&mut f, "}}").unwrap();
        }

        // determine the graphs' subgraphs.
        let sg = subgraphs::variants::SubgraphsAndRings::new(&g);

        {
            let filename = "trace/subgraphs.dot";
            let f = std::fs::File::create(filename).unwrap();
            crate::prelude::dump_set(f, sg.all_subgraphs()).unwrap();
        }

        // re-assemble the graph
        let gs = assemble(sg);

        assert!(gs.contains(&g));

        {
            let filename = "trace/result.dot";
            let f = std::fs::File::create(filename).unwrap();
            crate::prelude::dump_set(f, gs.iter()).unwrap();
        }
    }

    crate::STATISTICS.lock().unwrap().dump();
}
