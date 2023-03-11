mod directed_graph;
pub mod processing;
mod undirected_graph;

pub use directed_graph::{DirectedGraph, EdgeWeightedDigraph};
pub use undirected_graph::UndirectedGraph;

/// This trait gives some basic information on vertices
pub trait VertexInfo {
    // fn vertex_edges(&self, v: &usize) -> &HashSet<usize>;
    fn vertex_edges(&self, v: &usize) -> Vec<&usize>;
    fn nb_vertices(&self) -> usize;
}
