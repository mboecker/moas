use crate::Graph;

mod dna_esque;
pub use dna_esque::*;

pub trait MoleculeGenerator {
    fn generate(size: usize) -> Graph;
}
