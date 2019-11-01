use super::assemble;
use crate::subgraphs;
use crate::subgraphs::Subgraphs;
use crate::Graph;

fn test_rings(structure: &str) {
    let g = Graph::new(structure);

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

    let gs = assemble(sg);
    assert!(gs.contains(&g));
    if crate::statistics::trace_enabled() {
        let filename = "trace/result.dot";
        let f = std::fs::File::create(filename).unwrap();
        crate::prelude::dump_set(f, gs.iter()).unwrap();
    }
}

fn test_assembly(structure: &str) {
    test_rings(structure);
}

// #[test]
// #[ignore]
// fn dioxaziridine() {
//     let j = r#"{"atoms": [[1, 8], [2,8], [3,6], [4,1]],
//                 "bonds": [[1,2,1], [2,3,1], [1,3,1], [3,4,1]] }"#;
//     test_assembly(j);
// }

#[test]
fn benzol() {
    let j = r#"{"atoms": [[1, 6], [2, 6], [3, 6], [4, 6], [5, 6], [6, 6], [7, 1], [8, 1], [9, 1], [10, 1], [11, 1], [12, 1]],
                "bonds": [[1, 2, 2], [1, 3, 1], [1, 12, 1], [2, 4, 1], [2, 7, 1], [3, 5, 2], [3, 8, 1], [4, 6, 2], [4, 9, 1], [5, 6, 1], [5, 10, 1], [6, 11, 1]]}"#;
    test_assembly(j);
    crate::STATISTICS.lock().unwrap().dump();
}

#[test]
fn ethanol() {
    let j = r#"{"atoms": [[1, 8], [2, 6], [3, 6], [4, 1], [5, 1], [6, 1], [7, 1], [8, 1], [9, 1]],
                "bonds": [[1, 2, 1], [1, 9, 1], [2, 3, 1], [2, 4, 1], [2, 5, 1], [3, 6, 1], [3, 7, 1], [3, 8, 1]]}"#;
    test_assembly(j);
}

#[test]
fn caffeine() {
    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 7], [4, 7], [5, 7], [6, 7], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1]],
                "bonds": [[1, 9, 2], [2, 10, 2], [3, 8, 1], [3, 10, 1], [3, 12, 1], [4, 7, 1], [4, 11, 1], [4, 13, 1], [5, 9, 1], [5, 10, 1], [5, 14, 1], [6, 8, 1], [6, 11, 2], [7, 8, 2], [7, 9, 1], [11, 15, 1], [12, 16, 1], [12, 17, 1], [12, 18, 1], [13, 19, 1], [13, 20, 1], [13, 21, 1], [14, 22, 1], [14, 23, 1], [14, 24, 1]]}"#;
    test_assembly(j);
}

#[test]
#[ignore]
fn prophin() {
    let j = r#"{"atoms": [[1, 7], [2, 7], [3, 7], [4, 7], [5, 6], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 6], [16, 6], [17, 6], [18, 6], [19, 6], [20, 6], [21, 6], [22, 6], [23, 6], [24, 6], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1], [32, 1], [33, 1], [34, 1], [35, 1], [36, 1], [37, 1], [38, 1]],
                "bonds": [[1, 5, 1], [1, 7, 1], [1, 26, 1], [2, 6, 1], [2, 8, 1], [2, 27, 1], [3, 16, 2], [3, 18, 1], [4, 17, 1], [4, 19, 2], [5, 9, 2], [5, 10, 1], [6, 9, 1], [6, 11, 2], [7, 12, 1], [7, 14, 2], [8, 13, 2], [8, 15, 1], [9, 25, 1], [10, 12, 2], [10, 28, 1], [11, 13, 1], [11, 29, 1], [12, 30, 1], [13, 31, 1], [14, 16, 1], [14, 32, 1], [15, 17, 2], [15, 33, 1], [16, 21, 1], [17, 22, 1], [18, 20, 2], [18, 23, 1], [19, 20, 1], [19, 24, 1], [20, 34, 1], [21, 23, 2], [21, 35, 1], [22, 24, 2], [22, 36, 1], [23, 37, 1], [24, 38, 1]]}"#;
    test_assembly(j);
}
