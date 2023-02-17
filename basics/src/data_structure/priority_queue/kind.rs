/// Defines the orientation of a priority queue (min oriented or max oriented)
#[derive(Debug, Clone)]
pub enum PriorityQueueKind {
    /// Max-oriented priority queue
    Max,
    /// Min-oriented priority queue
    Min,
}

impl Default for PriorityQueueKind {
    fn default() -> Self {
        Self::Max
    }
}
