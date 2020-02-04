use super::attach;
use crate::Graph;

#[test]
fn simple() {
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,6],[5,6],[6,6]], "bonds": [[1,2,1],[2,3,2],[3,4,1],[4,5,2],[5,6,1],[6,1,2]]}"#;
    let g = Graph::from_old_json(j);
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,1]], "bonds": [[1,2,1],[2,3,2],[2,4,1]]}"#;
    let sg = Graph::from_old_json(j);

    let r = attach(&g, &sg);
    println!("{:?}", r);
}

#[test]
fn only_choice() {
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,5],[5,6],[6,6]], "bonds": [[1,2,1],[2,3,2],[3,4,1],[4,5,1],[5,6,1],[6,1,1]]}"#;
    let g = Graph::from_old_json(j);
    let j = r#"{"atoms": [[1,6],[2,6],[3,5],[4,1]], "bonds": [[1,2,2],[2,3,1],[2,4,1]]}"#;
    let sg = Graph::from_old_json(j);

    let r = attach(&g, &sg);
    println!("{:?}", r);
}

#[test]
fn k4() {
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,6],[5,6],[6,6]], "bonds": [[1,2,1],[2,3,2],[3,4,1],[4,5,2],[5,6,1],[6,1,3]]}"#;
    let g = Graph::from_old_json(j);
    let j =
        r#"{"atoms": [[1,6],[2,6],[3,6],[4,1],[5,6]], "bonds": [[1,2,1],[2,3,3],[2,4,1],[3,5,1]]}"#;
    let sg = Graph::from_old_json(j);

    let r = attach(&g, &sg);
    println!("{:?}", r);
}

#[test]
fn benzol() {
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,6],[5,6],[6,6]], "bonds": [[1,2,1],[2,3,1],[3,4,1],[4,5,1],[5,6,1],[6,1,1]]}"#;
    let g = Graph::from_old_json(j);
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,1]], "bonds": [[1,2,1],[2,3,1],[2,4,1]]}"#;
    let sg = Graph::from_old_json(j);

    let r = attach(&g, &sg);
    println!("{:?}", r);
}

#[test]
fn benzol_and_cube() {
    let j = r#"{"atoms": [[1,7],[2,6],[3,7],[4,7],[5,7],[6,6]], "bonds": [[1,2,1],[2,3,1],[3,4,1],[4,5,1],[5,6,1],[6,1,1]]}"#;
    let g = Graph::from_old_json(j);
    let j = r#"{"atoms": [[1,6],[2,7],[3,6],[4,7]], "bonds": [[1,2,1],[1,4,1],[2,3,1],[3,4,1]]}"#;
    let sg = Graph::from_old_json(j);

    for _ in 0..1000 {
        use rand;
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        let mut order: Vec<_> = (1..6).collect();
        order.shuffle(&mut rng);
        let mut order2 = vec![0];
        order2.extend(order);
        let g = g.permutate(&order2);

        let mut order: Vec<_> = (0..4).collect();
        order.shuffle(&mut rng);
        let sg = sg.permutate(&order);

        let r = attach(&g, &sg);
        assert!(r.len() > 0);

        let b = r
            .into_iter()
            .filter_map(|mapping| {
                let new_node = mapping.new_node;
                let mapping = mapping.into();
                super::perform(&g, &sg, mapping, new_node)
            })
            .any(|g| g.neighbors(g.size() - 1).count() > 1);

        assert!(b);
    }
}
