use crate::Graph;

pub trait Subgraphs {
    fn new(g: &Graph) -> Self;
    fn select_starting_graph(&self) -> Graph;
    fn basic_subgraphs<'a>(&'a self) -> Box<dyn 'a + Iterator<Item=&'a Graph>>;
}