#![feature(test)]

extern crate test;

use rusqlite::{Connection, NO_PARAMS};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod assembly;
mod attachment;
mod graph;
mod prelude;
mod subgraphs;
mod weisfeiler_lehman;

#[derive(Debug)]
/// An entry from the SQLite Database of all the pubchem molecules.
struct CompoundEntry {
    pub cid: i64,
    pub structure: String,
    pub is_contiguous: bool,
    pub atoms: i64,
    pub bonds: i64,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
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
        use itertools::Itertools;

        let g = graph::Graph::new(x.structure);
        //println!("{:?}", g);

        println!("graph subgraphs {{");

        //       for i in 3..=12 {
        let i = 5;
        let sg = subgraphs::subgraphs(&g, i);
        //  println!("für k = {} gibt es {} subgraphen", i, sg.len() / i);
        for (j, subgraph) in sg
            .into_iter()
            .chunks(i)
            .into_iter()
            .map(|x| g.subgraph(&x.collect::<Vec<_>>()))
            .enumerate()
        {
            subgraph.dump(i * j, true);
            eprintln!("{:x}", calculate_hash(&subgraph));
        }
        //     }

        println!("}}");
    }
}
