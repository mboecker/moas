#[derive(Debug, Hash, PartialEq, Eq)]
struct Attachment {
    mapping: Vec<(usize, usize)>,
}

pub fn attach(g: &Graph, sg: &Graph) -> HashSet<Attachment> {
    let mut r = Root::new(g, sg);
    r.collect();
}

struct Root {
    states: HashMap<(usize, usize), State>
}

impl Root {

    /// Create the root of an attachment tree by supplying the big graph and a small graph to be attached.
    pub fn new(g: &Graph, sg: &Graph) -> Root {
        let mut states = HashMap::new();

        // Try out every starting node.
        for i in sg.atoms().iter() {
            let similar = unimplemented!();
            for j in similar {

                // Initial starting node mapping.
                let m = (i,j);
                states.insert(m, State::first_level(m));
            }
        }

        Root {
            states
        }
    }

    /// Collect found possibilities of attachments.
    pub fn collect(mut self) -> HashSet<Attachment> {
        unimplemented!()
    }
}

/// A (partial) mapping of nodes.
struct State {
    mapping: Vec<(usize, usize)>,
    unmapped_sg_nodes: HashSet<usize>,
    free_g_nodes: HashSet<usize>,
    childs: HashMap<(usize, usize), State>
}

impl State {
    pub fn first_level(n: usize, m: (usize, usize)) -> State {
        let unmapped_sg_nodes = (0..m.0).chain((m.0..n)).collect();
        let free_g_nodes = (0..m.1).chain((m.1..n)).collect();
        State {
            mapping: vec![m],
            unmapped_nodes,
            free_nodes
        }
    }

    pub fn extend(&mut self, g: &Graph, sg: &Graph) {
        // iterate over edges in sg that are unmatched to g.
        unimplemented!()

        for i in self.unmapped_sg_nodes {
            for j in sg.neighbors(i) {
                let n = sg.bonds().get(i,j);

                // Try different matching nodes in the big graph g.
                // This means that the new node in the newly added edge is mapped to `candidate` in g.
                for candidate in self.try_edge(g) {
                    
                }
            }
        }
    }

    /// Searches for matching nodes in the big graph. Considered are adjacent nodes to `anchor` which are connected by a `n`-ary edge and have the correct label.
    fn try_edge<'a>(&'a self, g: &Graph, anchor: usize, n: u8, label: usize) -> impl 'a + Iterator<Item = usize> {
        g.neighbors(anchor).filter(|candidate| g.atoms()[candidate] == label && g.bonds().get(anchor, candidate) == n)
    }
}