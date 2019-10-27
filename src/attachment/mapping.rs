#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct Mapping {
    pub mapping: Vec<(usize, usize)>,
    pub min: usize,
}

impl Default for Mapping {
    fn default() -> Mapping {
        Mapping {
            mapping: Vec::new(),
            min: std::usize::MAX,
        }
    }
}

impl Mapping {
    pub fn new(i: usize, j: usize) -> Mapping {
        Mapping {
            mapping: vec![(i, j)],
            min: j,
        }
    }

    pub fn add(&mut self, i: usize, j: usize) {
        self.mapping.push((i, j));
        self.mapping.sort_unstable();
        if j < self.min {
            self.min = j;
        }
    }
}
