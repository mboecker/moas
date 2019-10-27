use crate::Graph;
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Result {
    pub mapping: Vec<(usize, usize)>,
    pub new_node: Option<usize>,
}

impl Result {
    pub fn new(sg: &Graph, mapping: Vec<(usize, usize)>) -> Result {
        let new_node = (0..sg.size())
            .filter(|i| mapping.iter().all(|(j, _)| j != i))
            .next();
        Result { mapping, new_node }
    }
}

impl Into<BTreeMap<usize, usize>> for Result {
    fn into(self) -> BTreeMap<usize, usize> {
        self.mapping.into_iter().collect()
    }
}
