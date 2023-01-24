mod undirected_graph;
mod directed_graph;
mod processing;

pub use undirected_graph::UndirectedGraph;
pub use directed_graph::DirectedGraph;
pub use processing::{
    DepthFirstSearch,
    BreadthFirstSearch,
    ConnectedComponent,
    TopologicalSort,
    StrongConnectedComponent
};
pub use std::collections::{LinkedList, HashSet};

pub trait VertexInfo{
    fn vertex_edges<'a>(&'a self, v: &usize) -> &'a HashSet<usize>;
    fn nb_vertices(&self) -> usize;
}
