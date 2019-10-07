use crate::graph::Graph;
use crate::subgraphs::get_all;
use crate::subgraphs::subgraphs3;

#[test]
fn test_subgraphs3_smol() {
    use itertools::Itertools;

    let j = r#"{"atoms": [[1, 1], [2,2], [3,3], [4,4], [5,5], [6,6]],
                "bonds": [[1,2,1], [1,3,1], [2,3,1], [2,5,1], [3,4,1], [4,5,1], [5,6,1]] }"#;
    let g = Graph::new(j);
    let sg = subgraphs3(&g);
    println!("{:?}", sg);

    let k = 4;
    let sg4 = get_all(&g, k);
    for chunk in &sg4.into_iter().chunks(k) {
        println!("{:?}", chunk.collect::<Vec<usize>>());
    }
}

#[test]
fn test_subgraphs3_large_graph() {
    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 8], [4, 8], [5, 7], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1]],
                "bonds": [[1, 7, 1], [1, 13, 1], [2, 12, 1], [3, 12, 2], [4, 13, 2], [5, 6, 1], [5, 8, 1], [5, 9, 1], [5, 10, 1], [6, 7, 1], [6, 15, 1], [6, 16, 1], [7, 11, 1], [7, 17, 1], [8, 18, 1], [8, 19, 1], [8, 20, 1], [9, 21, 1], [9, 22, 1], [9, 23, 1], [10, 24, 1], [10, 25, 1], [10, 26, 1], [11, 12, 1], [11, 27, 1], [11, 28, 1], [13, 14, 1], [14, 29, 1], [14, 30, 1], [14, 31, 1]]}"#;
    let g = Graph::new(j);
    let sg = subgraphs3(&g);
    println!("{:?}", sg);
}

#[test]
fn test_subgraphs4() {
    use itertools::Itertools;

    let j = r#"{"atoms": [[1, 1], [2,2], [3,3], [4,4], [5,5]],
                "bonds": [[1,2,1], [1,3,1], [2,3,1], [2,5,1], [3,4,1], [4,5,1]] }"#;
    let g = Graph::new(j);
    let sg = get_all(&g, 4);

    for chunk in &sg.into_iter().chunks(4) {
        println!("{:?}", chunk.collect::<Vec<usize>>());
    }
}

#[test]
fn test_subgraphs4_2() {
    use itertools::Itertools;

    let j = r#"{"atoms": [[1, 1], [2,2], [3,3], [4,4], [5,5]],
                "bonds": [[1,2,1], [1,3,1], [3,4,1], [4,5,1]] }"#;
    let g = Graph::new(j);
    let sg = get_all(&g, 4);

    for chunk in &sg.into_iter().chunks(4) {
        println!("{:?}", chunk.collect::<Vec<usize>>());
    }
}

#[test]
fn test_subgraphs5() {
    use itertools::Itertools;

    let j = r#"{"atoms": [[1, 1], [2,2], [3,3], [4,4], [5,5]],
                "bonds": [[1,2,1], [1,3,1], [2,3,1], [2,5,1], [3,4,1], [4,5,1]] }"#;
    let g = Graph::new(j);
    let sg = get_all(&g, 4);

    for chunk in &sg.into_iter().chunks(4) {
        println!("{:?}", chunk.collect::<Vec<usize>>());
    }
}

#[test]
fn test_subgraphs4_large_graph() {
    use itertools::Itertools;

    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 8], [4, 8], [5, 7], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1]],
                "bonds": [[1, 7, 1], [1, 13, 1], [2, 12, 1], [3, 12, 2], [4, 13, 2], [5, 6, 1], [5, 8, 1], [5, 9, 1], [5, 10, 1], [6, 7, 1], [6, 15, 1], [6, 16, 1], [7, 11, 1], [7, 17, 1], [8, 18, 1], [8, 19, 1], [8, 20, 1], [9, 21, 1], [9, 22, 1], [9, 23, 1], [10, 24, 1], [10, 25, 1], [10, 26, 1], [11, 12, 1], [11, 27, 1], [11, 28, 1], [13, 14, 1], [14, 29, 1], [14, 30, 1], [14, 31, 1]]}"#;
    let g = Graph::new(j);
    let sg = get_all(&g, 4);

    for chunk in &sg.into_iter().chunks(4) {
        println!("{:?}", chunk.collect::<Vec<usize>>());
    }
}

#[test]
fn test_subgraphs5_large_graph() {
    use itertools::Itertools;

    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 8], [4, 8], [5, 7], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1]],
                "bonds": [[1, 7, 1], [1, 13, 1], [2, 12, 1], [3, 12, 2], [4, 13, 2], [5, 6, 1], [5, 8, 1], [5, 9, 1], [5, 10, 1], [6, 7, 1], [6, 15, 1], [6, 16, 1], [7, 11, 1], [7, 17, 1], [8, 18, 1], [8, 19, 1], [8, 20, 1], [9, 21, 1], [9, 22, 1], [9, 23, 1], [10, 24, 1], [10, 25, 1], [10, 26, 1], [11, 12, 1], [11, 27, 1], [11, 28, 1], [13, 14, 1], [14, 29, 1], [14, 30, 1], [14, 31, 1]]}"#;
    let g = Graph::new(j);
    let sg = get_all(&g, 5);

    for chunk in &sg.into_iter().chunks(5) {
        println!("{:?}", chunk.collect::<Vec<usize>>());
    }
}

#[test]
fn test_subgraphs10_large_graph() {
    use itertools::Itertools;

    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 8], [4, 8], [5, 7], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1]],
                "bonds": [[1, 7, 1], [1, 13, 1], [2, 12, 1], [3, 12, 2], [4, 13, 2], [5, 6, 1], [5, 8, 1], [5, 9, 1], [5, 10, 1], [6, 7, 1], [6, 15, 1], [6, 16, 1], [7, 11, 1], [7, 17, 1], [8, 18, 1], [8, 19, 1], [8, 20, 1], [9, 21, 1], [9, 22, 1], [9, 23, 1], [10, 24, 1], [10, 25, 1], [10, 26, 1], [11, 12, 1], [11, 27, 1], [11, 28, 1], [13, 14, 1], [14, 29, 1], [14, 30, 1], [14, 31, 1]]}"#;
    let g = Graph::new(j);
    let sg = get_all(&g, 6);

    for chunk in &sg.into_iter().chunks(6) {
        println!("{:?}", chunk.collect::<Vec<usize>>());
    }
}

#[test]
fn test_subgraphs5_big_testgraph() {
    use itertools::Itertools;

    let j = r#"{"atoms": [[1, 3], [2,3], [3,3], [4,3], [5,3], [6,2], [7,2], [8,2], [9,1], [10,1], [11,1]],
                "bonds": [[1,2,1], [1,5,1], [1,8,1], [2,3,1], [2,11,1], [3,4,1], [3,9,1], [4,5,1], [4,10,1], [5,6,1], [6,7,1], [7,8,1]] }"#;
    let g = Graph::new(j);
    for k in 3..=11 {
        let sg = get_all(&g, k);
        for chunk in (&sg.into_iter().chunks(k)).into_iter() {
            println!("{:?}", chunk.collect::<Vec<usize>>());
        }
    }
}

#[test]
fn test_actual_subgraph() {
    use itertools::Itertools;

    let j = r#"{"atoms": [[1, 3], [2,3], [3,3], [4,3], [5,3], [6,2], [7,2], [8,2], [9,1], [10,1], [11,1]],
                "bonds": [[1,2,1], [1,5,1], [1,8,1], [2,3,1], [2,11,1], [3,4,1], [3,9,1], [4,5,1], [4,10,1], [5,6,1], [6,7,1], [7,8,1]] }"#;
    let g = Graph::new(j);
    let k = 7;
    let sg = get_all(&g, k);
    for subgraph in sg
        .into_iter()
        .chunks(k)
        .into_iter()
        .map(|x| g.subgraph(&x.collect::<Vec<usize>>()))
    {
        println!("{:?}", subgraph);
    }
}

#[test]
fn test_fake_benzol() {
    use crate::subgraphs;

    let j = r#"{"atoms": [[1, 1], [2, 2], [3, 3], [4,4], [5,5], [6,6]],
                "bonds": [[1,2,1], [2,3,1], [3,4,1], [4,5,1], [5,6,1], [6,1,1]] }"#;
    let g = Graph::new(j);
    let sg = subgraphs::get_all(&g, 3);

    let data: Vec<_> = sg
        .chunks(3)
        .map(|chunk| chunk.iter().cloned().collect::<Vec<_>>())
        .collect();

    let g1 = g.subgraph(&data[1]);
    let g2 = g.subgraph(&data[2]);

    assert!(g1 != g2);

    let sg = subgraphs::count_subgraphs(&g, &sg, 3);

    println!("{:?}", sg);

    assert!(sg.iter().any(|(g, c)| g.atoms() == &[1, 2, 3] && c == &1));
    assert!(sg.iter().any(|(g, c)| g.atoms() == &[1, 2, 6] && c == &1));
    assert!(sg.iter().any(|(g, c)| g.atoms() == &[1, 5, 6] && c == &1));
    assert!(sg.iter().any(|(g, c)| g.atoms() == &[2, 3, 4] && c == &1));
    assert!(sg.iter().any(|(g, c)| g.atoms() == &[3, 4, 5] && c == &1));
    assert!(sg.iter().any(|(g, c)| g.atoms() == &[4, 5, 6] && c == &1));
}

#[test]
fn test_fake_benzol4() {
    use crate::subgraphs;

    let j = r#"{"atoms": [[1, 1], [2, 2], [3, 3], [4,4], [5,5]],
                "bonds": [[1,2,1], [2,3,1], [3,4,1], [4,5,1], [5,1,1]] }"#;
    let g = Graph::new(j);
    let sg = subgraphs::get_all(&g, 4);

    let data: Vec<_> = sg
        .chunks(4)
        .map(|chunk| chunk.iter().cloned().collect::<Vec<_>>())
        .collect();

    for sg in data {
        g.subgraph(&sg).debug_print();
    }

    //panic!();

    let sg = subgraphs::count_subgraphs(&g, &sg, 4);

    println!("{:?}", sg);
}
