mod directed_graph;
pub mod processing;
mod undirected_graph;

pub use directed_graph::DirectedGraph;
use std::collections::HashSet;
pub use undirected_graph::UndirectedGraph;

/// This trait gives some basic information on vertices
pub trait VertexInfo {
    fn vertex_edges<'a>(&'a self, v: &usize) -> &'a HashSet<usize>;
    fn nb_vertices(&self) -> usize;
}
