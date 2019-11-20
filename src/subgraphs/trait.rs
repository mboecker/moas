use crate::Graph;

pub trait Subgraphs {
    fn new(g: &Graph) -> Self;
    fn select_starting_graph(&self) -> Graph;
    fn is_subset_of(&self, other: &Self) -> bool;
    fn all_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a Graph>>;
    fn with_counts<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&'a Graph, &'a usize)>>;
    fn attachable_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = &'a Graph>>;
    fn score(&self) -> usize;
    fn amount_of(&self, g: &Graph) -> usize;
    fn molecule_size(&self) -> usize;
    fn hydrogen_count(&self) -> usize;
}
