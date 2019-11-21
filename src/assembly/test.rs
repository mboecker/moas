use super::assemble;
use crate::subgraphs;
use crate::subgraphs::Subgraphs;
use crate::Graph;

use std::collections::HashSet;

fn test_rings(structure: &str) -> HashSet<Graph> {
    let g = Graph::from_json(structure);

    if crate::statistics::trace_enabled() {
        use std::io::Write;
        let filename = "trace/original.dot";
        let mut f = std::fs::File::create(filename).unwrap();
        writeln!(&mut f, "graph g {{").unwrap();
        g.dump(&mut f, 0, true).unwrap();
        writeln!(&mut f, "}}").unwrap();
    }

    let sg = subgraphs::variants::SubgraphsAndRings::new(&g);

    if crate::statistics::trace_enabled() {
        let filename = "trace/subgraphs.dot";
        let f = std::fs::File::create(filename).unwrap();
        crate::prelude::dump_set(f, sg.all_subgraphs()).unwrap();
    }

    let gs = assemble(sg, None).unwrap();
    assert!(gs.contains(&g));

    if crate::statistics::trace_enabled() {
        let filename = "trace/result.dot";
        let f = std::fs::File::create(filename).unwrap();
        crate::prelude::dump_set(f, gs.iter()).unwrap();
    }

    gs
}

fn test_assembly(structure: &str) -> HashSet<Graph> {
    test_rings(structure)
}

#[test]
fn benzol() {
    let j = r#"{"atoms": [[1, 6, 0], [2, 6, 0], [3, 6, 0], [4, 6, 0], [5, 6, 0], [6, 6, 0], [7, 1, 0], [8, 1, 0], [9, 1, 0], [10, 1, 0], [11, 1, 0], [12, 1, 0]],
                "bonds": [[1, 2, 2], [1, 3, 1], [1, 12, 1], [2, 4, 1], [2, 7, 1], [3, 5, 2], [3, 8, 1], [4, 6, 2], [4, 9, 1], [5, 6, 1], [5, 10, 1], [6, 11, 1]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 1);
}

#[test]
fn ethanol() {
    let j = r#"{"atoms": [[1, 8, 0], [2, 6, 0], [3, 6, 0], [4, 1, 0], [5, 1, 0], [6, 1, 0], [7, 1, 0], [8, 1, 0], [9, 1, 0]],
                "bonds": [[1, 2, 1], [1, 9, 1], [2, 3, 1], [2, 4, 1], [2, 5, 1], [3, 6, 1], [3, 7, 1], [3, 8, 1]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 1);
}

#[test]
fn cid_226() {
    let j = r#"{"atoms": [[1, 15, 0], [2, 8, 0], [3, 7, 0], [4, 6, 0], [5, 1, 0], [6, 1, 0], [7, 1, 0], [8, 1, 0], [9, 1, 0], [10, 1, 0]],
                "bonds": [[1, 4, 1], [1, 6, 1], [1, 7, 1], [2, 4, 1], [2, 10, 1], [3, 4, 1], [3, 8, 1], [3, 9, 1], [4, 5, 1]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 1);
}

#[test]
#[ignore]
fn caffeine() {
    let j = r#"{"atoms": [[1, 8, 0], [2, 8, 0], [3, 7, 0], [4, 7, 0], [5, 7, 0], [6, 7, 0], [7, 6, 0], [8, 6, 0], [9, 6, 0], [10, 6, 0], [11, 6, 0], [12, 6, 0], [13, 6, 0], [14, 6, 0], [15, 1, 0], [16, 1, 0], [17, 1, 0], [18, 1, 0], [19, 1, 0], [20, 1, 0], [21, 1, 0], [22, 1, 0], [23, 1, 0], [24, 1, 0]],
                "bonds": [[1, 9, 2], [2, 10, 2], [3, 8, 1], [3, 10, 1], [3, 12, 1], [4, 7, 1], [4, 11, 1], [4, 13, 1], [5, 9, 1], [5, 10, 1], [5, 14, 1], [6, 8, 1], [6, 11, 2], [7, 8, 2], [7, 9, 1], [11, 15, 1], [12, 16, 1], [12, 17, 1], [12, 18, 1], [13, 19, 1], [13, 20, 1], [13, 21, 1], [14, 22, 1], [14, 23, 1], [14, 24, 1]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 1);
}

#[test]
#[ignore]
fn prophin() {
    let j = r#"{"atoms": [[1, 7], [2, 7], [3, 7], [4, 7], [5, 6], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 6], [16, 6], [17, 6], [18, 6], [19, 6], [20, 6], [21, 6], [22, 6], [23, 6], [24, 6], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1], [32, 1], [33, 1], [34, 1], [35, 1], [36, 1], [37, 1], [38, 1]],
                "bonds": [[1, 5, 1], [1, 7, 1], [1, 26, 1], [2, 6, 1], [2, 8, 1], [2, 27, 1], [3, 16, 2], [3, 18, 1], [4, 17, 1], [4, 19, 2], [5, 9, 2], [5, 10, 1], [6, 9, 1], [6, 11, 2], [7, 12, 1], [7, 14, 2], [8, 13, 2], [8, 15, 1], [9, 25, 1], [10, 12, 2], [10, 28, 1], [11, 13, 1], [11, 29, 1], [12, 30, 1], [13, 31, 1], [14, 16, 1], [14, 32, 1], [15, 17, 2], [15, 33, 1], [16, 21, 1], [17, 22, 1], [18, 20, 2], [18, 23, 1], [19, 20, 1], [19, 24, 1], [20, 34, 1], [21, 23, 2], [21, 35, 1], [22, 24, 2], [22, 36, 1], [23, 37, 1], [24, 38, 1]]}"#;
    test_assembly(j);
}

#[test]
#[ignore]
fn cid_5055() {
    let j = r#"{"atoms": [[1, 8, 0], [2, 8, 0], [3, 8, 0], [4, 6, 0], [5, 6, 0], [6, 6, 0], [7, 6, 0], [8, 6, 0], [9, 6, 0], [10, 6, 0], [11, 6, 0], [12, 1, 0], [13, 1, 0], [14, 1, 0], [15, 1, 0], [16, 1, 0], [17, 1, 0], [18, 1, 0], [19, 1, 0]],
                "bonds": [[1, 4, 1], [1, 10, 1], [2, 7, 1], [2, 19, 1], [3, 10, 2], [4, 5, 1], [4, 6, 2], [5, 7, 2], [5, 12, 1], [6, 8, 1], [6, 13, 1], [7, 9, 1], [8, 9, 2], [8, 14, 1], [9, 15, 1], [10, 11, 1], [11, 16, 1], [11, 17, 1], [11, 18, 1]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 3);
}

#[test]
fn cid_67055201() {
    let j = r#"{"atoms": [[1, 8, 0], [2, 7, 0], [3, 7, 0], [4, 6, 0], [5, 6, 0], [6, 6, 0], [7, 6, 0], [8, 6, 0], [9, 6, 0], [10, 1, 0], [11, 1, 0], [12, 1, 0], [13, 1, 0], [14, 1, 0], [15, 1, 0]],
                    "bonds": [[1, 2, 1], [1, 6, 1], [2, 4, 1], [2, 10, 1], [3, 6, 2], [3, 9, 1], [4, 5, 2], [4, 6, 1], [5, 7, 1], [5, 8, 1], [7, 9, 2], [7, 11, 1], [8, 12, 1], [8, 13, 1], [8, 14, 1], [9, 15, 1]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 1);
}

#[test]
fn cid_13643966() {
    let j = r#"{"atoms": [[1, 16, 0], [2, 16, 0], [3, 8, 0], [4, 6, 0], [5, 1, 0], [6, 1, 0]],
                "bonds": [[1, 3, 1], [1, 4, 1], [2, 3, 1], [2, 4, 1], [4, 5, 1], [4, 6, 1]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 1);
}

#[test]
#[ignore]
fn cid_5462805() {
    let j = r#"{"atoms": [[1, 16, 0], [2, 16, 0], [3, 15, 0], [4, 15, 0], [5, 15, 0], [6, 15, 0]],
                "bonds": [[1, 4, 1], [1, 6, 1], [2, 5, 1], [2, 6, 1], [3, 4, 1], [3, 5, 1], [3, 6, 1], [4, 5, 1]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 1);
}

#[test]
fn cid_223() {
    // this is an ionized molecule.
    let j = r#"{"atoms": [[1, 7, 1], [2, 1, 0], [3, 1, 0], [4, 1, 0], [5, 1, 0]],
                "bonds": [[1, 2, 1], [1, 3, 1], [1, 4, 1], [1, 5, 1]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 1);
}

#[test]
fn cid_101203302() {
    let j = r#"{"atoms": [[1, 8, 0], [2, 6, 0], [3, 6, 0], [4, 6, 0], [5, 6, 0], [6, 6, 0], [7, 6, 0], [8, 6, -1], [9, 6, -1]],
                "bonds": [[1, 2, 1], [1, 5, 1], [2, 3, 3], [3, 4, 1], [4, 6, 3], [5, 8, 3], [6, 7, 1], [7, 9, 3]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 1);
}

#[test]
fn cid_16692396() {
    let j = r#"{"atoms": [[1, 15, 0], [2, 15, 0], [3, 15, 0], [4, 15, 0], [5, 6, 0], [6, 1, 0], [7, 1, 0]],
                "bonds": [[1, 2, 1], [1, 3, 1], [1, 5, 1], [2, 4, 1], [2, 5, 1], [3, 5, 1], [3, 6, 1], [4, 5, 1], [4, 7, 1]]}"#;
    let gs = test_assembly(j);
    assert_eq!(gs.len(), 1);
}

#[test]
fn cid_22116718() {
    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 6], [4, 6], [5, 1], [6, 1]], "bonds": [[1, 2, 1], [1, 3, 1], [2, 3, 1], [3, 4, 2], [4, 5, 1], [4, 6, 1]]}"#;

    let g = Graph::from_old_json(j);

    if crate::statistics::trace_enabled() {
        use std::io::Write;
        let filename = "trace/original.dot";
        let mut f = std::fs::File::create(filename).unwrap();
        writeln!(&mut f, "graph g {{").unwrap();
        g.dump(&mut f, 0, true).unwrap();
        writeln!(&mut f, "}}").unwrap();
    }

    let sg = subgraphs::variants::SubgraphsAndRings::new(&g);

    if crate::statistics::trace_enabled() {
        let filename = "trace/subgraphs.dot";
        let f = std::fs::File::create(filename).unwrap();
        crate::prelude::dump_set(f, sg.all_subgraphs()).unwrap();
    }

    let starting_node = sg
        .attachable_subgraphs()
        .filter(|g| {
            (0..g.size()).filter(|&i| g.atoms()[i] == 6).count() == 2
                && (0..g.size()).filter(|&i| g.atoms()[i] == 1).count() == 2
        })
        .next()
        .unwrap()
        .clone();

    let run = crate::assembly::run::Run::with_starting_graph(sg, starting_node);
    let gs = run.assemble().unwrap();
    assert!(gs.contains(&g));

    if crate::statistics::trace_enabled() {
        let filename = "trace/result.dot";
        let f = std::fs::File::create(filename).unwrap();
        crate::prelude::dump_set(f, gs.iter()).unwrap();
    }

    assert_eq!(gs.len(), 1);
}
