use crate::prelude::Matrix;
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};
use std::hash::{Hash, Hasher};

/// Represents a molecular graph.
/// size_of = 16 + 8 * n + 16 * 4 * n bit
/// That means that 50 subgraphs of size 5 use less than 2 KiB of space.
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
    pub fn with_size(n: usize) -> Graph {
        let atoms = vec![0usize; n];
        let bonds: Matrix<u8> = Matrix::new(n);
        Graph { n: n, atoms, bonds }
    }

    pub fn new(json: impl AsRef<str>) -> Graph {
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
            assert_eq!(atoms[j], 0);
            atoms[j] = a;
        }

        for (i, j, k) in graph.bonds {
            let i = i as usize - 1;
            let j = j as usize - 1;
            let k = k as u8;
            *bonds.get_mut(i, j) = k;
            *bonds.get_mut(j, i) = k;
        }

        Graph { n: n, atoms, bonds }
    }

    pub fn debug_print(&self) {
        for (i, l) in self.atoms.iter().enumerate() {
            println!("Node {}: {}", i, l);
        }

        for i in 0..self.size() {
            for n in self.neighbors(i) {
                println!("{} -> {}", i, n);
            }
        }
    }

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

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn bonds(&self) -> &Matrix<u8> {
        &self.bonds
    }

    pub fn bonds_mut(&mut self) -> &mut Matrix<u8> {
        &mut self.bonds
    }

    pub fn atoms(&self) -> &Vec<usize> {
        &self.atoms
    }

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
    pub fn dump(&self, offset: usize, use_element_names: bool) {
        for (i, j) in self.atoms.iter().enumerate() {
            if use_element_names {
                let label = get_element_label_from_element_id(j);
                println!(r#"  {} [shape=circle, label="{}"];"#, i + offset, label);
            } else {
                println!(r#"  {} [shape=circle, label="{}"];"#, i + offset, j);
            }
        }

        for j in 0..self.size() as usize {
            for i in 0..j {
                for _ in 0..*self.bonds.get(i as usize, j as usize) {
                    println!(
                        r#"  {} -- {} [type=s, splines=none];"#,
                        i + offset,
                        j + offset
                    );
                }
            }
        }
    }

    /// Provides an iterator over the elements of this graph that are adjacent to the given node.
    pub fn neighbors<'a>(&'a self, i: usize) -> impl 'a + Iterator<Item = usize> {
        (0..self.size() as usize).filter(move |j| self.bonds().get(i, *j) > &0)
    }

    /// Returns a Dictionary with counts for all the node labels in this graph.
    pub fn label_counts(&self) -> BTreeMap<usize, usize> {
        let mut hm = BTreeMap::new();
        for (_, label) in self.atoms.iter().enumerate() {
            *hm.entry(*label).or_insert(0) += 1;
        }
        hm
    }

    /// Changes the internal order of the nodes in this graph.
    /// This function is for testing only.
    #[must_use]
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
}

impl Hash for Graph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // This function must result in the same hash for equal (isomorph) graphs.

        let element_id = |i| match i {
            1 => 0,
            6 => 1,
            7 => 2,
            8 => 3,
            15 => 4,
            16 => 5,
            _ => 6,
        };

        // Sum up the element ids (associative, so the ordering of the atoms doesnt matter).
        {
            let label_counts = self.label_counts();
            for p in label_counts {
                p.hash(state);
            }
        }

        // Sum up the element ids (associative, so the ordering of the atoms doesnt matter).
        {
            use itertools::Itertools;

            let mut counts: Matrix<usize> = Matrix::new(7);

            for (i, j) in (0..self.size()).tuple_combinations().filter(|(i, j)| i < j) {
                let element1 = self.atoms()[i];
                let element2 = self.atoms()[j];
                *counts.get_mut(element_id(element1), element_id(element2)) +=
                    *self.bonds.get(i, j) as usize;
            }

            counts.hash(state);
        }
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Graph) -> bool {
        crate::are_isomorphic(self, other)
    }
}

impl Eq for Graph {}

fn get_element_label_from_element_id(id: &usize) -> &'static str {
    match id {
        1 => "H",
        2 => "He",
        3 => "Li",
        4 => "Be",
        5 => "B",
        6 => "C",
        7 => "N",
        8 => "O",
        9 => "F",
        10 => "Ne",
        11 => "Na",
        12 => "Mg",
        13 => "Al",
        14 => "Si",
        15 => "P",
        16 => "S",
        17 => "Cl",
        _ => "??",
    }
}

pub fn random_graph(n: usize) -> Graph {
    use rand::Rng;

    // get random number generator
    let mut rng = rand::thread_rng();

    // [2, n]
    let n_diff_labels = rng.gen_range(2, n + 1);

    // [1, 99]
    let p = rng.gen_range(1, 100);

    let mut g = Graph::with_size(n);

    for i in 0..n {
        g.atoms_mut()[i] = rng.gen_range(0, n_diff_labels);
    }

    for i in 0..n {
        for j in 0..i {
            let v = if rng.gen_ratio(p, 100) {
                rng.gen_range(1, 4)
            } else {
                0
            };
            *g.bonds_mut().get_mut(i, j) = v;
            *g.bonds_mut().get_mut(j, i) = v;
        }
    }

    if g.is_contiguous() {
        g
    } else {
        random_graph(n)
    }
}

#[test]
fn test_new_graph() {
    let j = r#"{"atoms": [[1, 8], [2, 8], [3, 8], [4, 8], [5, 7], [6, 6], [7, 6], [8, 6], [9, 6], [10, 6], [11, 6], [12, 6], [13, 6], [14, 6], [15, 1], [16, 1], [17, 1], [18, 1], [19, 1], [20, 1], [21, 1], [22, 1], [23, 1], [24, 1], [25, 1], [26, 1], [27, 1], [28, 1], [29, 1], [30, 1], [31, 1]],
                "bonds": [[1, 7, 1], [1, 13, 1], [2, 12, 1], [3, 12, 2], [4, 13, 2], [5, 6, 1], [5, 8, 1], [5, 9, 1], [5, 10, 1], [6, 7, 1], [6, 15, 1], [6, 16, 1], [7, 11, 1], [7, 17, 1], [8, 18, 1], [8, 19, 1], [8, 20, 1], [9, 21, 1], [9, 22, 1], [9, 23, 1], [10, 24, 1], [10, 25, 1], [10, 26, 1], [11, 12, 1], [11, 27, 1], [11, 28, 1], [13, 14, 1], [14, 29, 1], [14, 30, 1], [14, 31, 1]]}"#;
    let g = Graph::new(j);
    println!("{:?}", g);
}

#[test]
fn test_error1() {
    let j = r#"{"atoms": [[1, 15], [2, 8], [3, 8], [4, 8], [5, 8], [6, 8], [7, 7], [8, 6], [9, 6], [10, 6], [11, 1], [12, 1], [13, 1], [14, 1], [15, 1], [16, 1], [17, 1], [18, 1]], "bonds": [[1, 2, 1], [1, 3, 1], [1, 4, 1], [1, 6, 2], [2, 8, 1], [3, 17, 1], [4, 18, 1], [5, 9, 2], [7, 10, 1], [7, 15, 1], [7, 16, 1], [8, 9, 1], [8, 11, 1], [8, 12, 1], [9, 10, 1], [10, 13, 1], [10, 14, 1]]}"#;
    let g = Graph::new(j);
    println!("{:?}", g);
}

#[test]
fn test_random_graph() {
    for i in 3..100 {
        random_graph(i);
    }
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
    let n = 10;
    let g1 = random_graph(n);

    let mut order: Vec<_> = (0..n).collect();
    order.shuffle(&mut rng);
    let g2 = g1.permutate(&order);

    // println!("g1 = {:?}", g1);
    // println!("g2 = {:?}", g2);

    assert!(g1 == g2);
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
