#[cfg(test)]
mod unit_test;
use crate::graph::{FlowEdge, FlowNetwork, VertexInfo, Weight};
use std::collections::LinkedList;
///
pub struct FordFulkerson<T>
where
    T: Weight,
{
    source: usize,
    destination: usize,
    marked: Vec<bool>,
    edge_to: Vec<FlowEdge<T>>,
    value: Option<T>,
}

impl<T: Weight> FordFulkerson<T> {
    pub fn init(origin: usize, end: usize, nb_vertices: usize) -> Self {
        let zero = Weight::zero();
        Self {
            source: origin,
            destination: end,
            marked: vec![false; nb_vertices],
            edge_to: (0..nb_vertices)
                .map(|v| FlowEdge::init(v, v, zero, zero))
                .collect(),
            value: None,
        }
    }
}

impl<T: Weight> FordFulkerson<T> {
    /// Solves he Maxflow-Mincut problem
    /// with the Ford-Fulkerson algorithm
    pub fn find_flows(&mut self, network: &FlowNetwork<T>) {
        let mut value: T = Weight::zero();

        // Time complexity is <= to this value,
        let nb_max_iter = network.nb_vertices().pow(5); //TODO: find a better way to handle the while loop
        let mut nb_iter = 0;
        while self.has_augmenting_path(network) && nb_iter < nb_max_iter {
            // let mut has_aug_path = self.has_augmenting_path(network);

            // if self.has_augmenting_path(network) {
            let mut bottle = Weight::max();
            let mut vertex = self.destination;
            // println!("destination {}", self.destination);

            while vertex != self.source {
                let residual_cap = self.edge_to[vertex].residual_capacity_to(&vertex);
                bottle = std::cmp::min(bottle, residual_cap);
                vertex = *(self.edge_to[vertex].other(&vertex));
            }
            // println!("OK");

            let mut vertex = self.destination;
            while vertex != self.source {
                self.edge_to[vertex].add_residual_flow_to(&vertex, bottle);
                vertex = *(self.edge_to[vertex].other(&vertex));
            }

            value = value + bottle;
            nb_iter += 1;
        }
        // else {
        //     break;
        // };
        // println!("yes");
        // break;

        self.value = Some(value);
    }
    pub fn value(&self) -> Option<T> {
        self.value
    }
    pub fn in_cut(&self, vertex: &usize) -> bool {
        self.marked[*vertex]
    }
    fn has_augmenting_path(&mut self, network: &FlowNetwork<T>) -> bool {
        // It is a variant of breadth first search
        let zero = Weight::zero();
        let nb_vertices = network.nb_vertices();
        self.marked = vec![false; nb_vertices];
        // self.edge_to = (0..nb_vertices)
        //     .map(|v| FlowEdge::init(v, v, zero, zero))
        //     .collect();

        self.marked[self.source] = true;
        let mut queue = LinkedList::new();
        queue.push_back(self.source);

        while let Some(vertex) = queue.pop_front() {
            let adj = network.vertex_edges(&vertex);
            for flow_edge in adj {
                let other = *(flow_edge.other(&vertex));
                if flow_edge.residual_capacity_to(&other) > zero && !self.marked[other] {
                    self.edge_to[other] = *flow_edge;
                    self.marked[other] = true;
                    queue.push_back(other);
                }
            }
        }
        self.marked[self.destination]
    }
}
