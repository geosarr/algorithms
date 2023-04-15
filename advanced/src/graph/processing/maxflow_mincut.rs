#[cfg(test)]
mod unit_test;
// use crate::graph::{FlowEdge, FlowNetwork, VertexInfo, Weight};
use crate::graph::Weight;
///
// pub struct FordFulkerson<T>
// where
//     T: Weight,
// {
//     source: usize,
//     destination: usize,
//     marked: Vec<bool>,
//     edge_to: Vec<FlowEdge<T>>,
//     value: Option<T>,
// }

// impl<T: Weight> FordFulkerson<T> {
//     pub fn init(origin: usize, end: usize, nb_vertices: usize) -> Self {
//         let zero = Weight::zero();
//         Self {
//             source: origin,
//             destination: end,
//             marked: vec![false; nb_vertices],
//             edge_to: (0..nb_vertices)
//                 .map(|v| FlowEdge::init(v, v, zero, zero))
//                 .collect(),
//             value: None,
//         }
//     }
// }

// impl<T: Weight> FordFulkerson<T> {
//     /// Solves he Maxflow-Mincut problem
//     /// with the Ford-Fulkerson algorithm
//     pub fn find_flows(&mut self, network: &FlowNetwork<T>) {
//         let mut value: T = Weight::zero();

//         // Time complexity is <= to this value,
//         let nb_max_iter = network.nb_vertices().pow(5); //TODO: find a better way to handle the while loop
//         let mut nb_iter = 0;
//         // loop {
//         //     if !self.has_augmenting_path(network){
//         //         break;
//         //     }
//         while self.has_augmenting_path(network) && nb_iter < nb_max_iter {
//             // let mut has_aug_path = self.has_augmenting_path(network);

//             // if self.has_augmenting_path(network) {
//             let mut bottle = Weight::max();
//             let mut vertex = self.destination;
//             // println!("destination {}", self.destination);

//             while vertex != self.source {
//                 let residual_cap = self.edge_to[vertex].residual_capacity_to(&vertex);
//                 bottle = std::cmp::min(bottle, residual_cap);
//                 //     vertex = *(self.edge_to[vertex].other(&vertex));
//                 // }
//                 // // println!("OK");

//                 // let mut vertex = self.destination;
//                 // while vertex != self.source {
//                 self.edge_to[vertex].add_residual_flow_to(&vertex, bottle);
//                 vertex = *(self.edge_to[vertex].other(&vertex));
//             }

//             value = value + bottle;
//             nb_iter += 1;
//         }
//         // else {
//         //     break;
//         // };
//         // println!("yes");
//         // break;

//         self.value = Some(value);
//     }
//     pub fn value(&self) -> Option<T> {
//         self.value
//     }
//     pub fn in_cut(&self, vertex: &usize) -> bool {
//         self.marked[*vertex]
//     }

//     fn has_augmenting_path(&mut self, network: &FlowNetwork<T>) -> bool {
//         // It is a variant of breadth first search
//         let zero = Weight::zero();
//         let nb_vertices = network.nb_vertices();
//         self.marked = vec![false; nb_vertices];

//         self.marked[self.source] = true;
//         let mut queue = LinkedList::new();
//         queue.push_back(self.source);

//         while let Some(vertex) = queue.pop_front() {
//             let adj = network.vertex_edges(&vertex);
//             for flow_edge in adj {
//                 // let other = *(flow_edge.other(&vertex));
//                 let other = *(flow_edge.to());
//                 if flow_edge.residual_capacity_to(&other) > zero && !self.marked[other] {
//                     self.edge_to[other] = *flow_edge;
//                     if other == self.destination {
//                         return true;
//                     }
//                     self.marked[other] = true;
//                     queue.push_back(other);
//                 }
//             }
//         }
//         false
//     }
// }
use std::cmp::{max, min};
use std::collections::LinkedList;
use std::collections::VecDeque;

// Define a struct for a graph edge
// #[derive(Debug, Copy, Clone)]
// struct Edge {
//     from: usize,
//     to: usize,
//     capacity: i32,
//     flow: i32,
// }

// impl Edge {
//     fn residual_capacity(&self) -> i32 {
//         self.capacity - self.flow
//     }
// }

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct FlowEdge<T>
where
    T: Weight,
{
    from: usize,
    to: usize,
    flow: T,
    capacity: T,
}

impl<T: Weight> FlowEdge<T> {
    pub fn init(origin: usize, destination: usize, f: T, c: T) -> Self {
        Self {
            from: origin,
            to: destination,
            flow: f,
            capacity: c,
        }
    }

    pub fn from(&self) -> &usize {
        &self.from
    }

    pub fn to(&self) -> &usize {
        &self.to
    }

    pub fn flow(&self) -> &T {
        &self.flow
    }

    pub fn capacity(&self) -> &T {
        &self.capacity
    }

    pub fn other(&self, vertex: &usize) -> &usize {
        if vertex == self.from() {
            self.to()
        } else if vertex == self.to() {
            self.from()
        } else {
            panic!("Illegal endpoint {vertex}")
        }
    }
}

impl<T: Weight> FlowEdge<T> {
    pub fn residual_capacity(&self) -> T {
        self.capacity - self.flow
    }
    pub fn add_residual_flow_to(&mut self, vertex: &usize, delta: T) {
        if vertex == self.from() {
            self.flow = self.flow - delta;
        } else if vertex == self.to() {
            self.flow = self.flow + delta;
        } else {
            panic!("Illegal endpoint {vertex}")
        }
    }
}
// Define a struct for the graph
#[derive(Debug)]
struct Graph {
    nb_vertices: usize,
    edges: Vec<Vec<FlowEdge<i32>>>,
}

impl Graph {
    fn new(nb_vertices: usize) -> Graph {
        Graph {
            nb_vertices,
            edges: vec![Vec::new(); nb_vertices],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: i32) {
        let forward_edge = FlowEdge {
            from,
            to,
            capacity,
            flow: 0,
        };
        let backward_edge = FlowEdge {
            from: to,
            to: from,
            capacity: 0,
            flow: 0,
        };
        self.edges[from].push(forward_edge);
        self.edges[to].push(backward_edge);
    }

    fn has_augmenting_path(
        &self,
        source: usize,
        sink: usize,
        edge_to: &mut Vec<Option<usize>>,
    ) -> bool {
        let mut marked = vec![false; self.nb_vertices];
        let mut queue = VecDeque::new();

        marked[source] = true;
        queue.push_back(source);

        while let Some(node) = queue.pop_front() {
            for edge in self.edges[node].iter() {
                let next_node = edge.to();
                let next_node = *next_node;
                if !marked[next_node] && edge.residual_capacity() > 0 {
                    marked[next_node] = true;
                    edge_to[next_node] = Some(node);
                    if next_node == sink {
                        return true;
                    }
                    queue.push_back(next_node);
                }
            }
        }
        false
    }

    fn max_flow(&mut self, source: usize, sink: usize) -> i32 {
        let mut edge_to = vec![None; self.nb_vertices];
        let mut max_flow = 0;

        while self.has_augmenting_path(source, sink, &mut edge_to) {
            let mut path_flow = i32::MAX;

            // Find the bottleneck capacity of the path
            let mut node = sink;
            while let Some(parent_node) = edge_to[node] {
                let edge = self.edges[parent_node]
                    .iter()
                    .find(|e| e.to == node)
                    .unwrap();
                path_flow = min(path_flow, edge.residual_capacity());
                node = parent_node;
            }

            // Update the flow of each edge along the path
            node = sink;
            while let Some(parent_node) = edge_to[node] {
                let mut edge = self.edges[parent_node]
                    .iter_mut()
                    .find(|e| e.to == node)
                    .unwrap();
                edge.flow += path_flow;
                let backward_edge = self.edges[node]
                    .iter_mut()
                    .find(|e| e.to == parent_node)
                    .unwrap();
                backward_edge.flow -= path_flow;
                node = parent_node;
            }

            max_flow += path_flow;
        }

        max_flow
    }
}
