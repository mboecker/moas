//! # moas - Molecular Assembler
//!
//! A brute-force assembler for molecular graphs.
//! Performs a total enumeration of all possible graphs given a multiset of induced subgraphs.

#![deny(warnings)]
#![deny(missing_docs)]
#![feature(test)]
extern crate test;

use clap::{App, Arg};
use rusqlite::{Connection, NO_PARAMS};
use std::time::Duration;

mod assembly;
mod atoms;
mod attachment;
mod extra;
mod generator;
mod graph;
mod isomorphism;
mod prelude;
mod statistics;
mod subgraphs;

pub use assembly::assemble;
pub use atoms::Atoms;
use attachment::attach;
pub use graph::Graph;
pub use isomorphism::are_isomorphic;

fn main() {
    #[derive(Debug)]
    /// An entry from the SQLite Database of all the pubchem molecules.
    struct CompoundEntry {
        pub cid: i64,
        pub structure: String,
        pub is_contiguous: bool,
        pub atoms: i64,
        pub bonds: i64,
    }

    let matches = App::new("moas")
        .version("0.1")
        .author("Marvin Böcker <marvin.boecker@tu-dortmund.de>")
        .about("A brute-force assembler for molecular graphs")
        .arg(
            Arg::with_name("compound id")
                .short("c")
                .long("cid")
                .help("Dis- and reassemble the specified compound from the PubChem database.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("database file name")
                .short("d")
                .long("database")
                .help("Specify the name of the SQLite database containing the compound data.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("min")
                .long("min")
                .help("Dis- and reassemble the compounds between min and max.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("max")
                .long("max")
                .help("Dis- and reassemble the compounds between min and max.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cidsfile")
                .long("file")
                .help("A file to read CIDs from.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("queue_max")
                .short("q")
                .help("The maximum of partially assembled graphs in a current queue.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("time_limit")
                .short("t")
                .help("The time limit for the reconstruction of a single molecule. Given in seconds.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dna")
                .long("dna")
                .help("Instead of working on real molecule data, generate synthetic molecules.")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("dot")
                .long("dot")
                .help("if you set this flag, only the JSON data of this molecule will be printed.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("hash")
                .long("hash")
                .help("if you set this flag, only the cid and its identifier will be printed in csv format.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("cycles")
                .long("cycles")
                .help("if you set this flag a csv will be printed with the cid and the number of cycles.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("find_matches")
                .long("matches")
                .help("after reassembly of a molecule check the database for any matches of the non-original molecules.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("dump_siblings")
                .long("dump_siblings")
                .help("after reassembly of a molecule dump all siblings in dot format into a file in the supplied folder.")
                .takes_value(true)
        )
        .get_matches();

    let max_queue_size = matches.value_of("queue_max").map(|x| x.parse().unwrap());
    let time_limit = matches
        .value_of("time_limit")
        .map(|x| Duration::from_secs(x.parse().unwrap()));

    if let Some(i) = matches.value_of("dna") {
        use crate::subgraphs::Subgraphs;
        use generator::MoleculeGenerator;

        let i = i.parse().unwrap();
        let g = generator::DnaEsque::generate(i);

        if matches.is_present("dot") {
            println!("graph g {{");
            g.dump(std::io::stdout(), 0, true).unwrap();
            println!("}}");
            return;
        }

        let start = std::time::Instant::now();
        let sg = subgraphs::variants::SubgraphsAndRings::new(&g);
        let sg_dur = std::time::Instant::now() - start;

        // re-assemble the graph
        let start = std::time::Instant::now();
        let op = assemble(sg, max_queue_size, time_limit);
        let dur = std::time::Instant::now() - start;

        if op.is_some() {
            let (gs, ts) = op.unwrap();
            println!(
                "{}, {}, {}, {}, {}, {}, {}",
                i,
                gs.len(),
                sg_dur.as_secs_f64(),
                dur.as_secs_f64(),
                ts.iterations_needed,
                ts.total_active_graphs,
                ts.max_active_graphs
            );
        } else {
            println!(
                "{}, NA, {}, {}, NA, NA",
                i,
                sg_dur.as_secs_f64(),
                dur.as_secs_f64()
            );
        }

        return;
    }

    let sqlite_name = matches
        .value_of("database file name")
        .unwrap_or("sqlite/pubchem.db");
    let conn = Connection::open(sqlite_name).unwrap();

    if let Some(cid) = matches.value_of("compound id") {
        let sql = format!("SELECT cid, structure, is_contiguous, n_atoms, n_edges FROM compounds WHERE cid = {} LIMIT 1", cid);
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

            let g = graph::Graph::from_json(x.structure);

            if matches.is_present("dot") {
                println!("graph g {{");
                g.dump(std::io::stdout(), 0, true).unwrap();
                println!("}}");
                return;
            }

            if matches.is_present("hash") {
                let id = g.identifier();
                println!("{}, {}", x.cid, id);
                continue;
            }

            if crate::statistics::trace_enabled() {
                use std::io::Write;
                let filename = "trace/original.dot";
                let mut f = std::fs::File::create(filename).unwrap();
                writeln!(&mut f, "graph g {{").unwrap();
                g.dump(&mut f, 0, true).unwrap();
                writeln!(&mut f, "}}").unwrap();
            }

            // determine the graph's subgraphs.
            let sg = subgraphs::variants::SubgraphsAndRings::new(&g);

            if crate::statistics::trace_enabled() {
                let filename = "trace/subgraphs.dot";
                let f = std::fs::File::create(filename).unwrap();
                crate::prelude::dump_set(f, sg.all_subgraphs()).unwrap();
            }

            // re-assemble the graph
            let gs = assemble(sg, max_queue_size, time_limit)
                .expect("Overshot maximum queue size or reached time limit.")
                .0;

            if crate::statistics::trace_enabled() {
                let filename = "trace/result.dot";
                let f = std::fs::File::create(filename).unwrap();
                crate::prelude::dump_set(f, gs.iter()).unwrap();
            }

            if matches.is_present("find_matches") {
                let db_gs = crate::prelude::get_database_similar(&conn, g.identifier());

                'outer: for reconstructed_graph in gs {
                    // Check non-original graphs with the database.
                    if g != reconstructed_graph {
                        for (cid2, g2) in &db_gs {
                            if g2 == &reconstructed_graph {
                                println!("{}", cid2);
                                continue 'outer;
                            }
                        }
                        println!("NA");
                    }
                }
            } else if matches.value_of("dump_siblings").is_some() {
                for (gid, reconstructed_graph) in gs.into_iter().enumerate() {
                    use std::io::Write;
                    let filename = format!("{}/{}.dot", matches.value_of("dump_siblings").unwrap(), gid);
                    let mut f = std::fs::File::create(filename).unwrap();
                    writeln!(&mut f, "graph g {{").unwrap();
                    reconstructed_graph.dump(&mut f, 0, true).unwrap();
                    writeln!(&mut f, "}}").unwrap();    
                }
            } else {
                println!(
                    "Reconstruction of compound {cid} resulted in {len} molecules.",
                    cid = x.cid,
                    len = gs.len()
                );
            }
        }
    }

    if let Some(min) = matches.value_of("min") {
        if let Some(max) = matches.value_of("max") {
            let sql = format!("SELECT cid, structure, is_contiguous, n_atoms, n_edges FROM compounds WHERE is_contiguous == 1 AND cid >= {} AND cid < {} AND common_bonds = 1 AND chnops_only = 1 AND n_atoms >= 3 AND n_atoms <= 15", min, max);
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

                let g = graph::Graph::from_json(x.structure);

                if matches.is_present("hash") {
                    let id = g.identifier();
                    println!("{}, {}", x.cid, id);
                    continue;
                }

                print!("{cid}", cid = x.cid);

                // determine the graphs' subgraphs.
                let start = std::time::Instant::now();
                let sg = subgraphs::variants::SubgraphsAndRings::new(&g);
                let sg_dur = std::time::Instant::now() - start;

                // re-assemble the graph
                let start = std::time::Instant::now();
                let op = assemble(sg, max_queue_size, time_limit);
                let dur = std::time::Instant::now() - start;

                // assert!(gs.contains(&g), "The assembly of cid {} failed.", x.cid);

                if let Some((gs, ts)) = op {
                    // cid, duplicates, secs
                    println!(
                        ", {}, {}, {}, {}, {}, {}",
                        gs.len(),
                        sg_dur.as_secs_f64(),
                        dur.as_secs_f64(),
                        ts.iterations_needed,
                        ts.total_active_graphs,
                        ts.max_active_graphs
                    );

                    // assert!(gs.contains(&g), "The assembly of cid {} failed.", x.cid);

                    if crate::statistics::trace_enabled() {
                        let filename = format!("trace/results_{}.dot", x.cid);
                        let f = std::fs::File::create(filename).unwrap();
                        crate::prelude::dump_set(f, gs.iter()).unwrap();
                    }
                } else {
                    println!(
                        ", NA, {}, {}, NA, NA, NA",
                        sg_dur.as_secs_f64(),
                        dur.as_secs_f64()
                    );
                }
            }
        }
    }

    if let Some(filename) = matches.value_of("cidsfile") {
        use std::io::BufRead;
        use std::io::BufReader;

        let file = std::fs::File::open(filename).expect("Failed to open cids file.");
        let file = BufReader::new(file);
        let file_iter = file
            .lines()
            .map(|x| -> String { x.expect("Failed to read from cids file.") });

        let sql =
            "SELECT cid, structure, is_contiguous, n_atoms, n_edges FROM compounds WHERE cid = ? LIMIT 1";
        let mut stmt = conn.prepare(&sql).unwrap();

        for cid in file_iter {
            let iter = stmt
                .query_map(&[cid], |row| {
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

                if x.atoms < 5 {
                    continue;
                }

                let g = graph::Graph::from_json(x.structure);

                if matches.is_present("hash") {
                    let id = g.identifier();
                    println!("{}, {}", x.cid, id);
                    continue;
                }

                print!("{cid}", cid = x.cid);

                // determine the graphs' subgraphs.
                let start = std::time::Instant::now();
                let sg = subgraphs::variants::SubgraphsAndRings::new(&g);
                let sg_dur = std::time::Instant::now() - start;

                if matches.is_present("cycles") {
                    let (three, four) = g.cycles();
                    let (five, six) = sg.n_rings();
                    println!(
                        ", {three}, {four}, {five}, {six}",
                        three = three,
                        four = four,
                        five = five,
                        six = six
                    );
                    continue;
                }

                // re-assemble the graph
                let start = std::time::Instant::now();
                let op = assemble(sg, max_queue_size, time_limit);
                let dur = std::time::Instant::now() - start;

                if let Some((gs, ts)) = op {
                    // cid, duplicates, secs
                    println!(
                        ", {}, {}, {}, {}, {}, {}",
                        gs.len(),
                        sg_dur.as_secs_f64(),
                        dur.as_secs_f64(),
                        ts.iterations_needed,
                        ts.total_active_graphs,
                        ts.max_active_graphs
                    );

                    // assert!(gs.contains(&g), "The assembly of cid {} failed.", x.cid);

                    if crate::statistics::trace_enabled() {
                        let filename = format!("trace/results_{}.dot", x.cid);
                        let f = std::fs::File::create(filename).unwrap();
                        crate::prelude::dump_set(f, gs.iter()).unwrap();
                    }
                } else {
                    println!(
                        ", NA, {sg_dur}, {dur}, NA, NA, NA",
                        sg_dur = sg_dur.as_secs_f64(),
                        dur = dur.as_secs_f64()
                    );
                }
            }
        }
    }
}
