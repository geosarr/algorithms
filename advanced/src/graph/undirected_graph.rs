#[cfg(test)]
mod unit_test;
use crate::graph::{HashSet, VertexInfo};

pub struct UndirectedGraph {
    // implements an adjacency-list graph
    // where vertices have indices 0, ..., nb_objects
    // and each vertex is associated to its adjacent vertices
    data: Vec<HashSet<usize>>,
    nb_edges: usize,
    nb_vertices: usize,
}
impl Default for UndirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}
impl UndirectedGraph {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            nb_edges: 0,
            nb_vertices: 0,
        }
    }

    pub fn init(nb_objects: usize) -> Self {
        let mut graph = Self::new();
        graph.nb_vertices = nb_objects;
        graph.data = Vec::new();
        for _ in 0..nb_objects {
            graph.data.push(HashSet::<usize>::new());
        }
        graph
    }

    pub fn nb_edges(&self) -> usize {
        // run time complexity O(1)
        self.nb_edges
    }
    pub fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
    fn vertex_edges<'a>(&'a self, v: &usize) -> &'a HashSet<usize> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        &self.data[*v]
    }
    pub fn add_edge(&mut self, v: usize, w: usize) {
        // adds an edge to the graph
        // run time complexity O(1)
        self.data[v].insert(w);
        self.data[w].insert(v);
        self.nb_edges += 1;
    }
    pub fn add_vertex(&mut self) {
        self.data.push(HashSet::<usize>::new());
        self.nb_vertices += 1;
    }

    pub fn degree(&self, v: &usize) -> usize {
        self.vertex_edges(v).len()
    }

    pub fn average_degree(&self) -> f32 {
        // gets the average number of degree of the graph
        // each edge is counted only once (by the self.add_edge() method)
        if self.nb_vertices > 0 {
            (self.nb_edges as f32) / (self.nb_vertices as f32)
        } else {
            panic!("No vertex in the graph");
        }
    }

    pub fn self_loop_number(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(v, e)| if e.contains(&v) { 1 } else { 0 })
            .sum()
    }
}
impl VertexInfo for UndirectedGraph {
    fn vertex_edges<'a>(&'a self, v: &usize) -> &'a HashSet<usize> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        &self.data[*v]
    }
    fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }
}
