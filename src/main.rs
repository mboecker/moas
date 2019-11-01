#![feature(let_chains)]

#![feature(test)]
extern crate test;

use rusqlite::{Connection, NO_PARAMS};
use clap::{Arg, App};

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
    let matches = App::new("moas")
                    .version("0.1")
                    .author("Marvin Böcker <marvin.boecker@tu-dortmund.de>")
                    .about("brute force assembler for molecular graphs")
                    .arg(Arg::with_name("compound id")
                        .short("c")
                        .long("cid")
                        .help("Dis- and reassemble the specified compound from the PubChem database.")
                        .takes_value(true))
                    .arg(Arg::with_name("database file name")
                        .short("d")
                        .long("database")
                        .help("Specify the name of the SQLite database containing the compound data.")
                        .takes_value(true))
                    .arg(Arg::with_name("min")
                        .long("min")
                        .help("Dis- and reassemble the compounds between min and max.")
                        .takes_value(true))
                    .arg(Arg::with_name("max")
                        .long("max")
                        .help("Dis- and reassemble the compounds between min and max.")
                        .takes_value(true))
                    .get_matches();

    let sqlite_name = matches.value_of("database file name").unwrap_or("sqlite/pubchem.db");
    let conn = Connection::open(sqlite_name).unwrap();

    if let Some(cid) = matches.value_of("compound id") {
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

            if crate::statistics::trace_enabled() {
                use std::io::Write;
                let filename = "trace/original.dot";
                let mut f = std::fs::File::create(filename).unwrap();
                writeln!(&mut f, "graph g {{").unwrap();
                g.dump(&mut f, 0, true).unwrap();
                writeln!(&mut f, "}}").unwrap();
            }

            // determine the graphs' subgraphs.
            let sg = subgraphs::variants::SubgraphsAndRings::new(&g);

            if crate::statistics::trace_enabled() {
                let filename = "trace/subgraphs.dot";
                let f = std::fs::File::create(filename).unwrap();
                crate::prelude::dump_set(f, sg.all_subgraphs()).unwrap();
            }

            // re-assemble the graph
            let gs = assemble(sg);

            assert!(gs.contains(&g));

            if crate::statistics::trace_enabled() {
                let filename = "trace/result.dot";
                let f = std::fs::File::create(filename).unwrap();
                crate::prelude::dump_set(f, gs.iter()).unwrap();
            }
        }
    }

    if let Some(min) = matches.value_of("min") {
        if let Some(max) = matches.value_of("max") {
            let sql = format!("SELECT cid, structure, is_contiguous, n_atoms, n_edges FROM compounds where cid >= {} AND cid < {} LIMIT 1", min, max);
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

                // determine the graphs' subgraphs.
                let sg = subgraphs::variants::SubgraphsAndRings::new(&g);

                // re-assemble the graph
                let gs = assemble(sg);

                assert!(gs.contains(&g));

                if crate::statistics::trace_enabled() {
                    let filename = format!("trace/results_{}.dot", x.cid);
                    let f = std::fs::File::create(filename).unwrap();
                    crate::prelude::dump_set(f, gs.iter()).unwrap();
                }
            }
        }
    }
}
