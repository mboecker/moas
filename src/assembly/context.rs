use crate::Graph;
use std::collections::HashMap;

struct Context {
    index: HashMap<Graph, usize>,
    amounts: Vec<usize>,
}
