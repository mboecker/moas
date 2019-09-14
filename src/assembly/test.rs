use super::assemble;
use crate::subgraphs;
use crate::Graph;

#[test]
fn dioxaziridine() {
    let j = r#"{"atoms": [[1, 2], [2,2], [3,3], [4,1]],
                "bonds": [[1,2,1], [2,3,1], [1,3,1], [3,4,1]] }"#;
    let g = Graph::new(j);
    let sg = subgraphs::subgraphs(&g, 3);
    let sg = subgraphs::count_subgraphs(&g, &sg, 3);

    //println!("Subgraphs: {:?}", sg);

    let g = assemble(sg);

    println!("possible graphs: {:?}", g);

    // println!("graph possibles {{");
    // let mut i = 0;
    // for g in g {
    //     g.dump(i, false);
    //     i += g.size();
    // }
    // println!("}}");
}

#[test]
fn fake_benzol4() {
    let j = r#"{"atoms": [[1, 1], [2, 2], [3, 3], [4,4], [5,5], [6,6]],
                "bonds": [[1,2,1], [2,3,1], [3,4,1], [4,5,1], [5,6,1], [6,1,1]] }"#;
    let g = Graph::new(j);
    let sg = subgraphs::subgraphs(&g, 4);
    let sg = subgraphs::count_subgraphs(&g, &sg, 4);

    println!("Subgraphs: {:?}", sg);

    let g = assemble(sg);

    println!("possible graphs: {:?}", g);

    use std::fs::File;
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();

    // println!("graph possibles {{");
    // let mut i = 0;
    // for g in g {
    //     if g.size() == 8 {
    //         // g.dump(i, false);
    //         // i += g.size();
    //         for i in 0..g.size() {
    //             for n in g.neighbors(i) {
    //                 println!("{} -> {}", i, n);
    //             }
    //         }
    //         return;
    //     }
    // }
    // println!("}}");
}

#[test]
fn fake_benzol5() {
    let j = r#"{"atoms": [[1, 1], [2, 2], [3, 3], [4,4], [5,5], [6,6]],
                "bonds": [[1,2,1], [2,3,1], [3,4,1], [4,5,1], [5,6,1], [6,1,1]] }"#;
    let g = Graph::new(j);
    let sg = subgraphs::subgraphs(&g, 5);
    let sg = subgraphs::count_subgraphs(&g, &sg, 5);

    println!("Subgraphs: {:?}", sg);

    let g = assemble(sg);

    println!("possible graphs: {:?}", g);

    // println!("graph possibles {{");
    // let mut i = 0;
    // for g in g {
    //     if g.size() == 8 {
    //         // g.dump(i, false);
    //         // i += g.size();
    //         for i in 0..g.size() {
    //             for n in g.neighbors(i) {
    //                 println!("{} -> {}", i, n);
    //             }
    //         }
    //         return;
    //     }
    // }
    // println!("}}");
}

#[test]
fn slightly_less_fake_benzol() {
    let j = r#"{"atoms": [[1, 1], [2, 1], [3, 1], [4,1]],
                "bonds": [[1,2,1], [2,3,1], [3,4,1], [4,1,1]]}"#;
    let g = Graph::new(j);
    let sg = subgraphs::subgraphs(&g, 3);
    let sg = subgraphs::count_subgraphs(&g, &sg, 3);

    println!("Subgraphs: {:?}", sg);

    // for (g, _) in sg.iter() {
    //     use std::collections::hash_map::DefaultHasher;
    //     use std::hash::Hash;
    //     use std::hash::Hasher;
    //     let mut hasher = DefaultHasher::new();
    //     g.hash(&mut hasher);
    //     println!("{:?} -> {}", g, hasher.finish());
    // }

    // use itertools::Itertools;
    // for (i,j) in sg.iter().map(|(g,c)| g).tuple_combinations() {
    //     println!("eq? {} {}", i == j, i.is_isomorphic(j));
    // }

    // panic!();

    let g = assemble(sg);

    println!("possible graphs: {:?}", g);

    // println!("graph possibles {{");
    // let mut i = 0;
    // for g in g {
    //     g.dump(i, false);
    //     i += g.size();
    // }
    // println!("}}");
}

#[test]
fn test_benzol() {
    use std::fs::File;

    let j = r#"{"atoms": [[1, 6], [2, 6], [3, 6], [4,6], [5,6], [6,6]],
                "bonds": [[1,2,1], [2,3,1], [3,4,1], [4,5,1], [5,6,1], [6,1,1]] }"#;
    let g = Graph::new(j);
    let sg = subgraphs::subgraphs(&g, 4);
    let sg = subgraphs::count_subgraphs(&g, &sg, 4);

    println!("Subgraphs: {:?}", sg);

    // for (g, _) in sg.iter() {
    //     use std::collections::hash_map::DefaultHasher;
    //     use std::hash::Hash;
    //     use std::hash::Hasher;
    //     let mut hasher = DefaultHasher::new();
    //     g.hash(&mut hasher);
    //     println!("{:?} -> {}", g, hasher.finish());
    // }

    // use itertools::Itertools;
    // for (i,j) in sg.iter().map(|(g,c)| g).tuple_combinations() {
    //     println!("eq? {} {}", i == j, i.is_isomorphic(j));
    // }

    // panic!();

    let g = assemble(sg);

    println!("possible graphs: {:?}", g);

    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();

    println!("graph possibles {{");
    let mut i = 0;
    for g in g {
        g.dump(i, true);
        i += g.size();
    }
    println!("}}");
}
