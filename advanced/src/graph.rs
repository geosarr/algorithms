mod directed_graph;
mod processing;
mod undirected_graph;

pub use directed_graph::DirectedGraph;
pub use processing::{
    BreadthFirstSearch, ConnectedComponent, DepthFirstSearch, StrongConnectedComponent,
    TopologicalSort,
};
pub use std::collections::{HashSet, LinkedList};
pub use undirected_graph::UndirectedGraph;

pub trait VertexInfo {
    fn vertex_edges<'a>(&'a self, v: &usize) -> &'a HashSet<usize>;
    fn nb_vertices(&self) -> usize;
}
