#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub struct TreeStatistics {
    pub max_active_graphs: usize,
    pub total_active_graphs: usize,
}
