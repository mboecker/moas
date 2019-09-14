use crate::prelude::Matrix;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

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

fn brute_force_isomorphie_check(g1: &Graph, g2: &Graph) -> bool {
    let n = g1.atoms().len();
    // Space for the isomorphic mapping.
    // partial_mapping[x] = y
    let mut partial_mapping: Vec<usize> = vec![0; n];
    let mut undecided_nodes: HashSet<_> = g1
        .label_counts()
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(i, _)| i)
        .collect();
    let mut taken_g2_nodes: HashSet<usize> = HashSet::new();

    // inner recursive function
    fn inner(
        g1: &Graph,
        g2: &Graph,
        undecided_nodes: &mut HashSet<usize>,
        taken_g2_nodes: &mut HashSet<usize>,
        partial_mapping: &mut Vec<usize>,
    ) -> bool {
        if undecided_nodes.len() == 0 {
            return true;
        }

        // try every undecided node in g1
        for current in undecided_nodes.clone().iter() {
            let label = g1.atoms()[*current];

            // nodes with the same label
            let similar_nodes = g2
                .atoms()
                .iter()
                .enumerate()
                .filter(|(_, c_label)| c_label == &&label)
                .map(|(i, _)| i);

            // select possible candidates from g2
            for similar in similar_nodes {
                partial_mapping[*current] = similar;
                undecided_nodes.remove(current);
                taken_g2_nodes.insert(similar);

                // check if the rest is ok
                if inner(g1, g2, undecided_nodes, taken_g2_nodes, partial_mapping) {
                    return true;
                }

                undecided_nodes.insert(*current);
                taken_g2_nodes.remove(&similar);
            }
        }

        // No isomorphism found
        false
    }

    inner(
        g1,
        g2,
        &mut undecided_nodes,
        &mut taken_g2_nodes,
        &mut partial_mapping,
    )
}

fn random_graph(n: usize) -> Graph {
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
        for j in 0..n {
            *g.bonds_mut().get_mut(i, j) = if rng.gen_ratio(p, 100) { 1 } else { 0 };
        }
    }

    g
}

/// Represents a molecular graph.
/// size_of = 16 + 8 * n + 16 * 4 * n bit
/// That means that 50 subgraphs of size 5 use less than 2 KiB of space.
#[derive(Debug, Clone)]
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

        g
    }

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

    pub fn is_isomorphic(&self, other: &Graph) -> bool {
        // create two copies of the graphs so that we can alter the node names.
        let mut g1 = self.clone();
        let mut g2 = other.clone();

        // how many iterations of the relabeling-algorithm to do.
        const N_ITERS: usize = 5;

        for _ in 0..N_ITERS {
            // do a relabelling iteration
            // rename each node by appending their direct neighbors to themselves.
            let r_g1 = crate::weisfeiler_lehman::relabel(&g1);
            let r_g2 = crate::weisfeiler_lehman::relabel(&g2);

            if r_g1.1 != r_g2.1 {
                return false;
            }

            g1 = r_g1.0;
            g2 = r_g2.0;
        }

        // we now have sufficiently fine node labels
        brute_force_isomorphie_check(&g1, &g2)
    }

    pub fn neighbors<'a>(&'a self, i: usize) -> impl 'a + Iterator<Item = usize> {
        (0..self.size() as usize).filter(move |j| self.bonds().get(i, *j) > &0)
    }

    pub fn label_counts(&self) -> HashMap<usize, usize> {
        let mut hm = HashMap::new();
        for (_, label) in self.atoms.iter().enumerate() {
            *hm.entry(*label).or_insert(0) += 1;
        }
        hm
    }

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

        // Sum up the element ids (associative, so the ordering of the atoms doesnt matter).
        let atoms_hash: usize = self.atoms.iter().map(|x| *x as usize).product();
        atoms_hash.hash(state);
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Graph) -> bool {
        self.is_isomorphic(other)
    }
}

impl Eq for Graph {}

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
fn test_isomorphism() {
    use rand;
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    let n = 3;
    let g1 = random_graph(n);
    let g2 = g1.clone();

    println!("g1 = {:?}", g1);
    println!("g2 = {:?}", g2);

    let mut order: Vec<_> = (0..n).collect();
    order.shuffle(&mut rng);
    g2.permutate(&order);

    assert!(g1.is_isomorphic(&g2));
}

#[test]
#[ignore]
fn test_big_isomorphism() {
    use rand;
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    for n in 3..30 {
        for _ in 0..10 {
            let g1 = random_graph(n);
            for _ in 0..10 {
                let g2 = g1.clone();

                println!("g1 = {:?}", g1);
                println!("g2 = {:?}", g2);

                let mut order: Vec<_> = (0..n).collect();
                order.shuffle(&mut rng);
                g2.permutate(&order);

                assert!(g1.is_isomorphic(&g2));
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
                let g2 = random_graph(n);

                println!("g1 = {:?}", g1);
                println!("g2 = {:?}", g2);

                assert!(!g1.is_isomorphic(&g2));
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
        let g2 = g1.clone();

        let mut order: Vec<_> = (0..n).collect();
        order.shuffle(&mut rng);
        g2.permutate(&order);

        b.iter(|| g1.is_isomorphic(&g2));
    }
}

#[bench]
#[ignore]
fn bench_big_non_isomorphism(b: &mut test::Bencher) {
    for n in 20..50 {
        let g1 = random_graph(n);
        let g2 = random_graph(n);
        b.iter(|| g1.is_isomorphic(&g2));
    }
}
