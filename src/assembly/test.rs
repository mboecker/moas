use super::assemble;
use crate::subgraphs;
use crate::subgraphs::Subgraphs;
use crate::Graph;

fn test_assembly(structure: &str, _: usize) {
    let g = Graph::new(structure);

    {
        let filename = "trace/original.dot";
        let f = std::fs::File::create(filename).unwrap();
        g.dump(f, 0, true).unwrap();
    }

    let sg = subgraphs::variants::Only4::new(&g);
    let gs = assemble(sg);
    assert!(gs.contains(&g));
    {
        let filename = "trace/result.dot";
        let f = std::fs::File::create(filename).unwrap();
        crate::prelude::dump_set(f, gs.iter()).unwrap();
    }
}

#[test]
#[ignore]
fn dioxaziridine() {
    let j = r#"{"atoms": [[1, 2], [2,2], [3,3], [4,1]],
                "bonds": [[1,2,1], [2,3,1], [1,3,1], [3,4,1]] }"#;
    test_assembly(j, 3);
}

#[test]
#[ignore]
fn benzol() {
    let j = r#"{"atoms": [[1, 6], [2, 6], [3, 6], [4, 6], [5, 6], [6, 6], [7, 1], [8, 1], [9, 1], [10, 1], [11, 1], [12, 1]], "bonds": [[1, 2, 2], [1, 3, 1], [1, 12, 1], [2, 4, 1], [2, 7, 1], [3, 5, 2], [3, 8, 1], [4, 6, 2], [4, 9, 1], [5, 6, 1], [5, 10, 1], [6, 11, 1]]}"#;
    for k in &[3, 6usize] {
        test_assembly(j, *k);
    }
}

#[test]
#[ignore]
fn ethanol() {
    let j = r#"{"atoms": [[1, 8], [2, 6], [3, 6], [4, 1], [5, 1], [6, 1], [7, 1], [8, 1], [9, 1]], "bonds": [[1, 2, 1], [1, 9, 1], [2, 3, 1], [2, 4, 1], [2, 5, 1], [3, 6, 1], [3, 7, 1], [3, 8, 1]]}"#;
    for k in 3..5 {
        test_assembly(j, k);
    }
}

#[test]
#[ignore]
fn caffeine() {
    test_assembly(r#"{"atoms": [[1, 8], [2, 8], [3, 7], [4, 7], [5, 7], [6, 7], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1]], "bonds": [[1, 9, 2], [2, 10, 2], [3, 8, 1], [3, 10, 1], [3, 12, 1], [4, 7, 1], [4, 11, 1], [4, 13, 1], [5, 9, 1], [5, 10, 1], [5, 14, 1], [6, 8, 1], [6, 11, 2], [7, 8, 2], [7, 9, 1], [11, 15, 1], [12, 16, 1], [12, 17, 1], [12, 18, 1], [13, 19, 1], [13, 20, 1], [13, 21, 1], [14, 22, 1], [14, 23, 1], [14, 24, 1]]}
"#, 6);
}
