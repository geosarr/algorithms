#[cfg(test)]
mod unit_test;
use crate::graph::{HashSet, VertexInfo};
use crate::utils::read_lines;
use std::path::Path;

pub struct DirectedGraph {
    // implements an adjacency-list graph
    // where vertices have indices 0, ..., nb_objects
    // and each vertex is associated to the vertices it points to
    data: Vec<HashSet<usize>>,
    nb_edges: usize,
    nb_vertices: usize,
}
impl DirectedGraph {
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
    pub fn reverse(&self) -> Self {
        // Gets the reverse graph
        let nb_vertices = self.nb_vertices;
        let mut rev_graph = Self::init(nb_vertices);
        for v in 0..nb_vertices {
            let adj_v = self.vertex_edges(&v);
            for w in adj_v {
                rev_graph.add_edge(*w, v);
            }
        }
        rev_graph
    }
    pub fn from_file<P>(filename: P, sep: char, nb_vertices: usize) -> Self
    where
        P: AsRef<Path>,
    {
        // Builds a directed graph from a file with edges.
        // All the elements of each row should be non
        // negative integers separated by the value of the sep
        // argument, each row represent one or many edges from the first vertex to
        // the other ones. If there is only one value, it will be skipped
        let mut nb_iter = 0;
        println!("Initializing the graph");
        let mut dg = DirectedGraph::init(nb_vertices);
        match read_lines(filename) {
            Ok(lines) => {
                for (pos, line) in lines.enumerate() {
                    if let Ok(row) = line {
                        let values = row.split(sep).collect::<Vec<&str>>();
                        for i in 1..values.len() {
                            dg.add_edge(
                                values[0].parse::<usize>().unwrap(),
                                values[i].parse::<usize>().unwrap(),
                            );
                        }
                        // println!("{:?}", dg.vertex_edges(&values[0].parse::<usize>().unwrap()));
                        println!("{}", nb_iter);
                        nb_iter += 1
                    }
                }
            }
            Err(error) => panic!("{error}"),
        }
        dg
    }

    pub fn nb_edges(&self) -> usize {
        // run time complexity O(1)
        self.nb_edges
    }

    pub fn nb_vertices(&self) -> usize {
        // run time complexity O(1)
        self.nb_vertices
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        // adds an edge from v to w to the graph
        // run time complexity O(1)
        self.data[v].insert(w);
        self.nb_edges += 1;
    }
    pub fn add_vertex(&mut self) {
        self.data.push(HashSet::<usize>::new());
        self.nb_vertices += 1;
    }
    pub fn vertex_edges<'a>(&'a self, v: &usize) -> &'a HashSet<usize> {
        // gets all the vertices linked to a given vertex v,
        // that is the adjacent vertices of v
        // run time complexity O(1)
        &self.data[*v]
    }

    pub fn out_degree(&self, v: &usize) -> usize {
        // the number of vertices the vertex v points to
        self.vertex_edges(v).len()
    }

    pub fn average_degree(&self) -> f32 {
        // gets the average number of degree of the graph
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
impl VertexInfo for DirectedGraph {
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
