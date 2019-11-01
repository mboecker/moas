use crate::Graph;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Mutex;

pub const fn trace_enabled() -> bool {
    false
}

lazy_static! {
    pub static ref STATISTICS: Mutex<Statistics> = Mutex::new(Statistics::default());
}

#[derive(Default)]
pub struct Statistics {
    proposed_graphs: BTreeMap<usize, usize>,
    valid_proposed_graphs: BTreeMap<usize, usize>,
    invalid_graph: HashMap<Graph, usize>,
}

impl Statistics {
    pub fn graph_proposed(&mut self, iter: usize) {
        *self.proposed_graphs.entry(iter).or_default() += 1;
    }

    pub fn valid_graph_proposed(&mut self, iter: usize) {
        *self.valid_proposed_graphs.entry(iter).or_default() += 1;
    }

    pub fn dump(&mut self) {
        use std::io::Write;
        {
            let mut f = std::fs::File::create("statistics.txt").unwrap();
            for iter in self.proposed_graphs.keys() {
                writeln!(
                    &mut f,
                    "proposed graphs in iter {}: {:?}",
                    iter,
                    self.proposed_graphs.get(iter)
                )
                .unwrap();
                writeln!(
                    &mut f,
                    " -> amount of valid graphs: {:?}",
                    self.valid_proposed_graphs.get(iter)
                )
                .unwrap();
            }
        }

        {
            let f = std::fs::File::create("invalid_subgraphs.dot").unwrap();
            crate::prelude::dump_set(f, self.invalid_graph.keys()).unwrap();
        }
    }

    pub fn report_invalid_graph(&mut self, g: Graph) {
        self.invalid_graph.entry(g).or_insert(1);
    }
}
