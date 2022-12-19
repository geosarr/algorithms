#[derive(Debug)]
pub enum PriorityQueueKind {
    Max, // Max priority queue
    Min, // Min priority queue
}


impl Default for PriorityQueueKind{
    fn default() -> Self{
        Self::Max
    }
}