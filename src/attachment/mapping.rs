#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct Mapping {
    pub mapping: Vec<(usize, usize)>,
}

impl Default for Mapping {
    fn default() -> Mapping {
        Mapping {
            mapping: Vec::new(),
        }
    }
}

impl Mapping {
    pub fn new(i: usize, j: usize) -> Mapping {
        Mapping {
            mapping: vec![(i, j)],
        }
    }

    pub fn add(&mut self, i: usize, j: usize) {
        self.mapping.push((i, j));
        self.mapping.sort_unstable();
    }
}
