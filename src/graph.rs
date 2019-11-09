use crate::prelude::Matrix;
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};
use std::hash::{Hash, Hasher};
use std::io::Write;

/// Represents a molecular graph.
/// Graphs are not resizeable.
#[derive(Debug, Clone, Serialize)]
pub struct Graph {
    // How many atoms has this molecule?
    n: usize,

    /// What atoms are in this molecule?
    atoms: Vec<usize>,

    /// How are these atoms bonded?
    bonds: Matrix<u8>,
}

impl Graph {
    /// Creates an empty graph with the specified size.
    pub fn with_size(n: usize) -> Graph {
        let atoms = vec![0usize; n];
        let bonds: Matrix<u8> = Matrix::new(n);
        Graph { n, atoms, bonds }
    }

    /// Parses the given JSON and transforms it into a graph object.
    /// The JSON format is documented in JSON_FORMAT.MD
    pub fn from_json(json: impl AsRef<str>) -> Graph {
        use std::convert::TryInto;

        #[derive(serde::Deserialize, serde::Serialize)]
        struct SqliteGraph {
            atoms: Vec<(u16, usize, i8)>,
            bonds: Vec<(u16, u16, u8)>,
        };

        let graph: SqliteGraph = serde_json::from_str(json.as_ref()).unwrap();
        let n = graph.atoms.len().try_into().unwrap();
        let mut atoms = vec![0usize; n];
        let mut bonds: Matrix<u8> = Matrix::new(n);

        for (i, a, charge) in graph.atoms {
            let j = i as usize - 1;
            atoms[j] = crate::Atoms::encode(a as u8, charge);
        }

        for (i, j, k) in graph.bonds {
            let i = i as usize - 1;
            let j = j as usize - 1;
            let k = k as u8;
            *bonds.get_mut(i, j) = k;
            *bonds.get_mut(j, i) = k;
        }

        Graph { n, atoms, bonds }
    }

    /// Parses the given JSON and transforms it into a graph object.
    /// The JSON format is documented in JSON_FORMAT.MD
    pub fn from_old_json(json: impl AsRef<str>) -> Graph {
        use std::convert::TryInto;

        #[derive(serde::Deserialize, serde::Serialize)]
        struct SqliteGraph {
            atoms: Vec<(u16, usize)>,
            bonds: Vec<(u16, u16, u8)>,
        };

        let graph: SqliteGraph = serde_json::from_str(json.as_ref()).unwrap();
        let n = graph.atoms.len().try_into().unwrap();
        let mut atoms = vec![0usize; n];
        let mut bonds: Matrix<u8> = Matrix::new(n);

        for (i, a) in graph.atoms {
            let j = i as usize - 1;
            atoms[j] = crate::Atoms::encode(a as u8, 0);
        }

        for (i, j, k) in graph.bonds {
            let i = i as usize - 1;
            let j = j as usize - 1;
            let k = k as u8;
            *bonds.get_mut(i, j) = k;
            *bonds.get_mut(j, i) = k;
        }

        Graph { n, atoms, bonds }
    }

    #[cfg(test)]
    pub fn debug_print(&self) {
        for (i, l) in self.atoms.iter().enumerate() {
            println!("Node {}: {:?}", i, l);
        }

        for i in 0..self.size() {
            for n in self.neighbors(i) {
                println!("{} -> {}", i, n);
            }
        }
    }

    /// Clones this graph while adding `n` new nodes/atoms with element number 0.
    pub fn clone_with_extraspace(&self, n: usize) -> Graph {
        use itertools::Itertools;

        let mut g = Graph::with_size(self.atoms.len() + n);
        for i in 0..self.size() {
            g.atoms_mut()[i] = self.atoms()[i];
        }

        for (i, j) in (0..self.size()).tuple_combinations() {
            let v = *self.bonds.get(i, j);
            *g.bonds.get_mut(i, j) = v;
            *g.bonds.get_mut(j, i) = v;
        }

        g
    }

    /// Returns the number of atoms in this graph.
    pub fn size(&self) -> usize {
        self.n
    }

    /// Returns the bond matrix of this graph.
    pub fn bonds(&self) -> &Matrix<u8> {
        &self.bonds
    }

    /// Returns the bond matrix of this graph, as a mutable reference.
    pub fn bonds_mut(&mut self) -> &mut Matrix<u8> {
        &mut self.bonds
    }

    /// Returns the elements of the atoms in this graph.
    pub fn atoms(&self) -> &Vec<usize> {
        &self.atoms
    }

    /// Returns the elements of the atoms in this graph, as a mutable reference.
    pub fn atoms_mut(&mut self) -> &mut Vec<usize> {
        &mut self.atoms
    }

    /// Creates a new graph from the given node ids.
    /// Atom labels and edges carry over.
    pub fn subgraph(&self, nodes: &[usize]) -> Graph {
        use itertools::Itertools;

        let mut g = Graph::with_size(nodes.len());
        g.atoms = nodes
            .iter()
            .map(|&i| *self.atoms.get(i as usize).unwrap())
            .collect();

        for (i, j) in (0..nodes.len()).tuple_combinations() {
            let v = *self.bonds.get(nodes[i] as usize, nodes[j] as usize);
            *g.bonds.get_mut(i, j) = v;
            *g.bonds.get_mut(j, i) = v;
        }

        assert!(g.is_contiguous());

        g
    }

    /// Returns true if this graph is contiguous.
    pub fn is_contiguous(&self) -> bool {
        let mut h = HashSet::new();
        self.is_contiguous_helper(0, &mut h);
        h.len() == self.size()
    }

    fn is_contiguous_helper(&self, i: usize, h: &mut HashSet<usize>) {
        h.insert(i);
        for n in self.neighbors(i) {
            if !h.contains(&n) {
                self.is_contiguous_helper(n, h);
            }
        }
    }

    /// Prints the graph using dot format.
    /// Misses the first and last line (graph { and }).
    /// You can specify if you want the node ids to start from a different number than 0
    /// and if you want to replace element numbers with their abbreviated element name.
    pub fn dump(
        &self,
        mut f: impl Write,
        offset: usize,
        use_element_names: bool,
    ) -> std::io::Result<()> {
        for (i, j) in self.atoms.iter().enumerate() {
            if use_element_names {
                let label = crate::Atoms::label(*j);
                writeln!(f, r#"  {} [shape=circle, label="{}"];"#, i + offset, label)?;
            } else {
                writeln!(f, r#"  {} [shape=circle, label="{}"];"#, i + offset, j)?;
            }
        }

        for j in 0..self.size() as usize {
            for i in 0..j {
                for _ in 0..*self.bonds.get(i as usize, j as usize) {
                    writeln!(
                        f,
                        r#"  {} -- {} [type=s, splines=none];"#,
                        i + offset,
                        j + offset
                    )?;
                }
            }
        }

        std::io::Result::Ok(())
    }

    /// Provides an iterator over the elements of this graph that are adjacent to the given node.
    pub fn neighbors<'a>(&'a self, i: usize) -> impl 'a + Iterator<Item = usize> {
        (0..self.size() as usize).filter(move |j| self.bonds().get(i, *j) > &0)
    }

    /// Returns a Dictionary with counts for all the node labels in this graph.
    pub fn label_counts(&self) -> BTreeMap<usize, usize> {
        let mut hm = BTreeMap::new();
        for label in self.atoms.iter() {
            *hm.entry(*label).or_insert(0) += 1;
        }
        hm
    }

    /// Changes the internal order of the nodes in this graph.
    /// This function is for testing only.
    #[must_use]
    #[cfg(test)]
    pub fn permutate(&self, order: &[usize]) -> Graph {
        let mut other = self.clone();

        // copy node labels but in order
        for i in 0..self.n {
            other.atoms_mut()[i] = self.atoms()[order[i]];
        }

        // copy graph edges but in order
        for i in 0..self.n {
            for j in 0..self.n {
                *other.bonds_mut().get_mut(i, j) = *self.bonds().get(order[i], order[j]);
            }
        }

        other
    }

    /// Returns the number of edges this graph has.
    pub fn number_of_edges(&self) -> usize {
        use itertools::Itertools;
        (0..self.size())
            .tuple_combinations::<(_, _)>()
            .filter(|(i, j)| i < j)
            .map(|(i, j)| self.bonds().get(i, j))
            .fold(0usize, |a, b| a + *b as usize)
    }

    /// Determines if this graph is one big circle.
    pub fn is_circular(&self) -> bool {
        (0..self.size()).all(|i| self.neighbors(i).count() == 2)
    }

    /// Returns the node id of the first atom, that has free bonds.
    pub fn first_unfull_node_id(&self) -> Option<usize> {
        (0..self.size())
            .filter(|&i| {
                let e = self.atoms[i];
                let n: u8 = self.neighbors(i).map(|j| self.bonds.get(i, j)).sum();
                n < crate::Atoms::max_bonds(e)
            })
            .next()
    }
}

impl Hash for Graph {
    /// This function must result in the same hash for equal (isomorph) graphs.
    fn hash<H: Hasher>(&self, state: &mut H) {
        // For each combination of node labels, count the edges between these.
        let mut edge_counts: BTreeMap<(usize, usize, u8), usize> = BTreeMap::new();

        // Hash the node labels and their occurance count in sorted order,
        // so that their ordering is consistent in different graphs.
        let mut label_counts = BTreeMap::new();

        // Count, how many edges each node has.
        // let mut degree_counts: BTreeMap<[usize; 3], usize> = BTreeMap::new();

        for i in 0..self.size() {
            // Increase label count of current node.
            let element1 = self.atoms()[i];
            *label_counts.entry(element1).or_insert(0) += 1;

            // let mut triple = [0,0,0];

            for j in 0..i {
                let v = *self.bonds().get(i, j);

                if v > 0 {
                    // Count a v-degree bond between these two elements.
                    // triple[*self.bonds().get(i,j) as usize - 1] += 1;

                    // normalize element tuple
                    let element2 = self.atoms()[j];
                    let tuple = if element1 > element2 {
                        (element2, element1, v)
                    } else {
                        (element1, element2, v)
                    };
                    *edge_counts.entry(tuple).or_default() += 1;
                }
            }
            // *degree_counts.entry(triple).or_default() += 1;
        }

        // Hash some values that are equal on isomorphic graphs.
        label_counts.hash(state);
        edge_counts.hash(state);
        // degree_counts.hash(state);
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Graph) -> bool {
        crate::are_isomorphic(self, other)
    }
}

impl Eq for Graph {}

#[cfg(test)]
pub fn random_graph(n: usize) -> Graph {
    use rand::Rng;

    // get random number generator
    let mut rng = rand::thread_rng();

    loop {
        // [2, n]
        let n_diff_labels = rng.gen_range(2, n + 1);

        // [1, 99]
        let p = rng.gen_range(1, 100);

        let mut g = Graph::with_size(n);

        for i in 0..n {
            g.atoms_mut()[i] = rng.gen_range(0, n_diff_labels);
        }

        let mut counts: Vec<_> = (0..n).map(|_| 0u8).collect();

        for _ in 0..10 {
            for i in 0..n {
                for j in 0..i {
                    let v = if rng.gen_ratio(p, 100) {
                        rng.gen_range(1, 4)
                    } else {
                        0
                    };

                    if counts[i] <= 4 - v && counts[j] <= 4 - v {
                        counts[i] += v;
                        counts[j] += v;
                        *g.bonds_mut().get_mut(i, j) = v;
                        *g.bonds_mut().get_mut(j, i) = v;
                    }
                }
            }

            if g.is_contiguous() {
                return g;
            }
        }

        if g.is_contiguous() {
            return g;
        }
    }
}

#[test]
fn test_new_graph() {
    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 8], [4, 8], [5, 7], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1]],
                "bonds": [[1, 7, 1], [1, 13, 1], [2, 12, 1], [3, 12, 2], [4, 13, 2], [5, 6, 1], [5, 8, 1], [5, 9, 1], [5, 10, 1], [6, 7, 1], [6, 15, 1], [6, 16, 1], [7, 11, 1], [7, 17, 1], [8, 18, 1], [8, 19, 1], [8, 20, 1], [9, 21, 1], [9, 22, 1], [9, 23, 1], [10, 24, 1], [10, 25, 1], [10, 26, 1], [11, 12, 1], [11, 27, 1], [11, 28, 1], [13, 14, 1], [14, 29, 1], [14, 30, 1], [14, 31, 1]]}"#;
    let g = Graph::from_old_json(j);
    println!("{:?}", g);
}

#[test]
fn test_error1() {
    let j = r#"{"atoms": [[1, 15], [2, 8], [3, 8], [4, 8], [5, 8], [6, 8], [7, 7], [8, 6], [9, 6], [10, 6], [11, 1], [12, 1], [13, 1], [14, 1], [15, 1], [16, 1], [17, 1], [18, 1]], "bonds": [[1, 2, 1], [1, 3, 1], [1, 4, 1], [1, 6, 2], [2, 8, 1], [3, 17, 1], [4, 18, 1], [5, 9, 2], [7, 10, 1], [7, 15, 1], [7, 16, 1], [8, 9, 1], [8, 11, 1], [8, 12, 1], [9, 10, 1], [10, 13, 1], [10, 14, 1]]}"#;
    let g = Graph::from_old_json(j);
    println!("{:?}", g);
}

#[test]
#[ignore]
fn test_big_hashing() {
    use rand;
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    for n in 3..30 {
        for _ in 0..10 {
            let g1 = random_graph(n);
            for _ in 0..10 {
                let mut order: Vec<_> = (0..n).collect();
                order.shuffle(&mut rng);
                let g2 = g1.permutate(&order);

                println!("g1 = {:?}", g1);
                println!("g2 = {:?}", g2);

                assert!(g1 == g2);
            }
        }
    }
}

#[test]
fn test_isomorphism() {
    use rand;
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    let n = 6;
    let g1 = random_graph(n);

    let mut order: Vec<_> = (0..n).collect();
    order.shuffle(&mut rng);
    let g2 = g1.permutate(&order);

    // println!("g1 = {:?}", g1);
    // println!("g2 = {:?}", g2);

    assert_eq!(g1, g2);
}

#[test]
#[ignore]
fn test_big_isomorphism() {
    use rand;
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    for n in 20..30 {
        for _ in 0..10 {
            let g1 = random_graph(n);
            for _ in 0..10 {
                let mut order: Vec<_> = (0..n).collect();
                order.shuffle(&mut rng);
                let g2 = g1.permutate(&order);

                println!("g1 = {:?}", g1);
                println!("g2 = {:?}", g2);

                assert!(g1 == g2);
            }
        }
    }
}

#[test]
#[ignore]
fn test_big_non_isomorphism() {
    for n in 20..50 {
        for _ in 0..10 {
            let g1 = random_graph(n);
            for _ in 0..10 {
                let mut g2 = random_graph(n);
                *g2.atoms_mut() = g1.atoms().clone();

                println!("g1 = {:?}", g1);
                println!("g2 = {:?}", g2);

                assert!(g1 != g2);
            }
        }
    }
}

#[bench]
#[ignore]
fn bench_big_isomorphism(b: &mut test::Bencher) {
    use rand;
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    for n in 3..30 {
        let g1 = random_graph(n);

        let mut order: Vec<_> = (0..n).collect();
        order.shuffle(&mut rng);
        let g2 = g1.permutate(&order);

        b.iter(|| g1 == g2);
    }
}

#[bench]
#[ignore]
fn bench_big_non_isomorphism(b: &mut test::Bencher) {
    for n in 20..50 {
        let g1 = random_graph(n);
        let g2 = random_graph(n);
        b.iter(|| g1 == g2);
    }
}
