use crate::graph::Graph;
use crate::subgraphs;

use std::collections::{HashMap, HashSet};

// Provides a common interface and enumeration for the subgraphs,
// so that ints (usizes) can be used in the other functions.
struct Context {
    subgraphs: Vec<Graph>,
}

// The brute-force-tree created by trying to reconstruct a molecule graph.
#[derive(Debug)]
struct Tree {
    // The top-level children, in subgraph-order.
    center_nodes: Vec<Node>,
}

/// A decision in the tree, representing the attachment of a new subgrah to the constructed graph.
/// This includes which subgraph is attached as well as the exact position and overlapping nodes.
#[derive(Debug, PartialEq, Eq)]
struct Decision {
    // Index of the newly added subgraph.
    newest_sg: usize,

    // mapping of nodes in the new subgraph to existing nodes in the graph.
    attachment_points: Vec<(usize, usize)>,

    // the tree node with the resulting graph
    resulting_graph: Node,
}

// A node of the tree. Contains a partially constructed graph and information to keep the tree growing,
// such as subgraphs that are allowed to be used.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    // the currently constructed graph
    graph: Graph,

    // used subgraphs: subgraph id and amount used
    used_subgraphs: HashMap<usize, usize>,

    // Children of this node: current graph with every possible attached subgraph
    children: Vec<Decision>,
}

impl Context {
    pub fn avail<'a>(
        &'a self,
        used: &'a HashMap<usize, usize>,
    ) -> impl 'a + Iterator<Item = (usize, usize)> {
        self.subgraphs.iter().enumerate().filter_map(move |(i, _)| {
            let p = *used.get(&i).unwrap_or(&0);
            if p > 0 {
                Some((i, p))
            } else {
                None
            }
        })
    }

    pub fn len(&self) -> usize {
        self.subgraphs.len()
    }

    pub fn get<'a>(&'a self, idx: usize) -> &'a Graph {
        &self.subgraphs[idx]
    }
}

impl Tree {
    // Initially fill the tree root with all possible "middle" subgraphs.
    pub fn new(ctx: &Context) -> Tree {
        let n = ctx.len();
        Tree {
            center_nodes: (0..n)
                .map(|i| {
                    let g = ctx.get(i).clone();

                    Node {
                        graph: g,
                        used_subgraphs: std::iter::once((i, 1)).collect(),
                        children: Vec::new(),
                    }
                })
                .collect(),
        }
    }

    pub fn fill(&mut self, ctx: &Context) {
        for node in &mut self.center_nodes {
            node.expand(ctx);
        }
    }

    pub fn collect(self, ctx: &Context, hs: &mut HashSet<Graph>) {
        for node in self.center_nodes {
            node.collect(ctx, hs);
        }
    }
}

impl Node {
    pub fn expand(&mut self, ctx: &Context) {
        for sg in ctx.avail(&self.used_subgraphs) {
            let decisions = Decision::new(&self.graph, ctx, &self.used_subgraphs, sg.0);
            self.children.extend(decisions);
        }
    }

    pub fn collect(self, ctx: &Context, graphs: &mut HashSet<Graph>) {
        // check if there are any unused subgraphs.
        if ctx.avail(&self.used_subgraphs).peekable().peek().is_none() {
            // all subgraphs are used, so this is a completely constructed graph.
            graphs.insert(self.graph);
        } else {
            // propagate collection call down the tree.
            for decision in self.children.into_iter() {
                decision.collect(ctx, graphs);
            }
        }
    }
}

impl Decision {
    pub fn new(
        g: &Graph,
        ctx: &Context,
        used_subgraphs: &HashMap<usize, usize>,
        new_subgraph: usize,
    ) -> impl Iterator<Item = Decision> {
        // Try to attach new_subgraph to g.
        // Try every possible combination of attachment points,
        // including attaching the "new graph node" to any existing graph node.
        let sg = ctx.get(new_subgraph);

        // if false {
        //     let g = g.clone();

        //     let u = used_subgraphs.clone();
        //     *u.entry(new_subgraph).or_insert(0) += 1;

        //     let attachment_points = HashSet::new();
        //     let resulting_graph = Node {
        //         graph: g,
        //         used_subgraphs: u,
        //         fitted_subgraphs: HashMap::new(),
        //     };

        //     std::iter::once(Decision {
        //         attachment_points,
        //         resulting_graph
        //     })
        // } else {

        // The subgraph is impossible to attach.
        std::iter::empty()
        // }
    }

    pub fn collect(self, ctx: &Context, graphs: &mut HashSet<Graph>) {
        self.resulting_graph.collect(ctx, graphs);
    }
}

pub fn assemble(mut subgraphs: HashMap<Graph, usize>) -> HashSet<Graph> {
    let compressed = subgraphs.into_iter().map(|(a, _)| a).collect();
    let context = Context {
        subgraphs: compressed,
    };

    // Populate tree root with possible subgraphs.
    let mut tree = Tree::new(&context);

    // Populate and fill tree.
    tree.fill(&context);

    // Allocate space and return found graphs.
    let mut hs = HashSet::new();
    tree.collect(&context, &mut hs);
    hs
}

#[test]
fn test_simple_assembly() {
    let j = r#"{"atoms": [[1, 1], [2,2], [3,3], [4,4], [5,5]],
                "bonds": [[1,2,1], [1,3,1], [3,4,1], [4,5,1]] }"#;
    let g = Graph::new(j);
    let sg = subgraphs::subgraphs(&g, 3);
    let sg = subgraphs::count_subgraphs(&g, &sg, 3);

    println!("Subgraphs: {:?}", sg);

    let g = assemble(sg);

    println!("possible graphs: {:?}", g);
}
