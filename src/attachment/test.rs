use super::attach;
use crate::Graph;

#[test]
fn simple() {
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,6],[5,6],[6,6]], "bonds": [[1,2,1],[2,3,2],[3,4,1],[4,5,2],[5,6,1],[6,1,2]]}"#;
    let g = Graph::new(j);
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,1]], "bonds": [[1,2,1],[2,3,2],[2,4,1]]}"#;
    let sg = Graph::new(j);

    let r = attach(&g, &sg);
    println!("{:?}", r);
}

#[test]
fn only_choice() {
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,5],[5,6],[6,6]], "bonds": [[1,2,1],[2,3,2],[3,4,1],[4,5,1],[5,6,1],[6,1,1]]}"#;
    let g = Graph::new(j);
    let j = r#"{"atoms": [[1,6],[2,6],[3,5],[4,1]], "bonds": [[1,2,2],[2,3,1],[2,4,1]]}"#;
    let sg = Graph::new(j);

    let r = attach(&g, &sg);
    println!("{:?}", r);
}

#[test]
fn k4() {
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,6],[5,6],[6,6]], "bonds": [[1,2,1],[2,3,2],[3,4,1],[4,5,2],[5,6,1],[6,1,3]]}"#;
    let g = Graph::new(j);
    let j =
        r#"{"atoms": [[1,6],[2,6],[3,6],[4,1],[5,6]], "bonds": [[1,2,1],[2,3,3],[2,4,1],[3,5,1]]}"#;
    let sg = Graph::new(j);

    let r = attach(&g, &sg);
    println!("{:?}", r);
}

#[test]
fn benzol() {
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,6],[5,6],[6,6]], "bonds": [[1,2,1],[2,3,1],[3,4,1],[4,5,1],[5,6,1],[6,1,1]]}"#;
    let g = Graph::new(j);
    let j = r#"{"atoms": [[1,6],[2,6],[3,6],[4,1]], "bonds": [[1,2,1],[2,3,1],[2,4,1]]}"#;
    let sg = Graph::new(j);

    let r = attach(&g, &sg);
    println!("{:?}", r);
}
