use crate::graph::{processing::TopologicalSort, EdgeWeightedDigraph};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Eq, PartialEq)]
struct CurrentNode {
    vertex: usize,
    distance: usize,
}

// Taken and adapted from the standard library documentation
// for binary heap
impl Ord for CurrentNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on distances.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}
impl PartialOrd for CurrentNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(
    graph: &EdgeWeightedDigraph,
    source: usize,
    edge_to: &mut Vec<usize>,
    dist_to: &mut Vec<usize>,
) {
    let nb = graph.nb_vertices();
    assert_eq!(edge_to.len(), dist_to.len());
    assert_eq!(nb, edge_to.len());

    let mut priority_queue = BinaryHeap::new();
    dist_to[source] = 0;
    priority_queue.push(CurrentNode {
        vertex: source,
        distance: 0,
    });

    while let Some(CurrentNode { vertex, distance }) = priority_queue.pop() {
        let neighbors = graph.vertex_edges(&vertex);
        for (neighbor, dist) in neighbors {
            let node = CurrentNode {
                vertex: *neighbor,
                distance: distance + *dist,
            };
            if dist_to[*neighbor] > node.distance {
                relax(dist_to, edge_to, vertex, *neighbor, *dist);
                // Not optimal, should see first whether or not
                // the vertex in node is already in the heap
                // if it is the case then update its distance
                // otherwise push it into the heap.
                {
                    priority_queue.push(node);
                }
            }
        }
    }
}

pub fn relax(
    dist_to: &mut [usize],
    edge_to: &mut [usize],
    origin: usize,
    destination: usize,
    dist: usize,
) {
    dist_to[destination] = dist_to[origin] + dist;
    edge_to[destination] = origin;
}

/// Function that computes the shortest paths from a source
/// for edge weighted directed acyclic graph
pub fn shortest_path_ewdag(
    graph: &EdgeWeightedDigraph,
    source: usize,
    edge_to: &mut Vec<usize>,
    dist_to: &mut Vec<usize>,
) {
    let nb = graph.nb_vertices();
    assert_eq!(edge_to.len(), dist_to.len());
    assert_eq!(nb, edge_to.len());

    let mut topo = TopologicalSort::init(nb);
    topo.depth_first_order(graph);
    dist_to[source] = 0;

    // tells whether or not the source
    // vertex is processed in the topological
    // order
    let mut flag_source = false;
    for vertex in topo.order() {
        if *vertex == source {
            flag_source = true;
        }
        if flag_source {
            let neighbors = graph.vertex_edges(vertex);
            for (neighbor, dist) in neighbors {
                if dist_to[*neighbor] > dist_to[*vertex] + *dist {
                    relax(dist_to, edge_to, *vertex, *neighbor, *dist);
                }
            }
        }
    }
}
