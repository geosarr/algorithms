use std::marker::PhantomData;
use crate::graph::{VertexInfo, LinkedList, HashSet};

pub struct DepthFirstSearch<G>{
    // Indicates whether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Indicates what is the previous vertex leading to the current vertex 
    // when edge_to[w]=w, then no path is found yet from v to w
    edge_to: Vec<usize>,
    // vertex for which paths are computed
    v: usize,
    // type of the graph
    graph_type: PhantomData<G>
}
impl<G: VertexInfo> DepthFirstSearch<G>{
    pub fn init(nb_vertices: usize, origin: usize) -> Self {
        Self {
            marked: vec![false;nb_vertices],
            edge_to: (0..nb_vertices).collect::<Vec<usize>>(), 
            v: origin,
            graph_type: PhantomData
        }
    }

    pub fn find_paths(&mut self, graph: &G){
        // finds all paths from self.v in self.graph
        Self::dfs(graph, &mut self.marked, &mut self.edge_to, &self.v);
    }

    fn dfs(graph: &G, marked: &mut Vec<bool>, edge_to: &mut Vec<usize>, w: &usize){
        // finds all reachable vertices from w
        // run time complexity O(sum of degrees of all reachable vertices from v)

        // mark vertex v as visited
        marked[*w] = true;

        // recursively visit all unmarked adjacent vertices to v
        let adjacent_vertices = graph.vertex_edges(w);
        for u in adjacent_vertices{
            if !marked[*u] {
                Self::dfs(graph, marked, edge_to, u);
                edge_to[*u] = *w;
            }
        }
    }

    pub fn path_to(&self, w: usize) -> Option<LinkedList<usize>> {
        // finds the path from v to w
        // run time complexity O(length of the path)
        // can be very time consuming for some applications
        
        if !self.marked[w] {return None}
        let mut path = LinkedList::<usize>::new();
        let mut x = w;
        while x != self.v {
            path.push_back(x);
            x = self.edge_to[x];
        }
        path.push_back(self.v);
        Some(path)
    }
}


pub struct BreadthFirstSearch<G>{
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Indicates what is the previous vertex leading to the current vertex 
    // when edge_to[w]=w, then no path is found yet from v to w
    edge_to: Vec<usize>,
    // Vertex for which paths are computed
    v: usize,
    // type of the graph
    graph_type: PhantomData<G>
}
impl<G: VertexInfo> BreadthFirstSearch<G>{
    pub fn init(nb_vertices: usize, origin: usize) -> Self {
        Self {
            marked: vec![false;nb_vertices],
            edge_to: (0..nb_vertices).collect::<Vec<usize>>(), 
            v: origin,
            graph_type: PhantomData
        }
    }

    pub fn find_paths(&mut self, graph: &G){
        // finds all reachable vertices from w

        let mut queue = LinkedList::<usize>::new();
        // mark the vertex w as visited and add it to the queue
        queue.push_back(self.v);
        self.marked[self.v] = true;

        while !queue.is_empty(){
            // remove the first vertex in the queue
            // add to the queue all unmarked vertices adjacent to v and mark them
            // println!("{:?}", queue);
            let x = queue.pop_front().unwrap();
            let adj_x = graph.vertex_edges(&x);
            for u in adj_x{
                if !self.marked[*u]{
                    queue.push_back(*u);
                    self.marked[*u] = true;
                    self.edge_to[*u] = x;
                }
            } 
        }
        // println!("{:?}\n{:?}", self.edge_to, self.marked);
    }

    pub fn path_to(&self, w: usize) -> Option<LinkedList<usize>> {
        // finds the path from v to w
        // run time complexity O(length of the path)
        // computes shortest paths

        if !self.marked[w] {return None}
        let mut path = LinkedList::<usize>::new();
        let mut x = w;
        while x != self.v {
            path.push_back(x);
            x = self.edge_to[x];
        }
        path.push_back(self.v);
        Some(path)
    }
}


pub struct ConnectedComponent<G>{
    // Aims at answering the question are two vertives v and w connected in contant time
    // after preprocessing the graph
    // Identifier of the connected commponent vertices belong to
    id: Vec<usize>,
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Number of connected components
    nb_cc: usize,
    // Whether or not the algorithm has run
    ran: bool,
    // type of the graph
    graph_type: PhantomData<G>
}
impl<G: VertexInfo> ConnectedComponent<G>{
    pub fn init(nb_vertices: usize) -> Self {
        Self {
            marked: vec![false;nb_vertices],
            id: (0..nb_vertices).collect::<Vec<usize>>(), 
            nb_cc: 0,
            ran: false,
            graph_type: PhantomData
        }
    }
    pub fn find_cc(&mut self, graph: &G){
        // builds all the connected components from a graph
        let nb = graph.nb_vertices();
        for v in 0..nb{
            if !self.marked[v]{
                // run DFS for each vertex in each component
                Self::dfs_cc(graph, &mut self.marked, &mut self.id, v, v);
                self.nb_cc += 1;
            }
        }
        self.ran = true;
    }
    pub fn connected(&self, v: usize, w: usize) -> Option<bool>{
        // finds out whether or not two vertices are connected 
        // run time complexity O(1)
        if !self.marked[v] || !self.marked[w] {return None;}
        Some(self.id[v] == self.id[w])
    }
    pub fn count(&self) -> usize{
        self.nb_cc
    }
    pub fn is_bipartite(&self) -> Option<bool> {
        if self.ran {
            Some(self.nb_cc == 1)
        } else{
            None
        }
    }

    fn dfs_cc(graph: &G, marked: &mut Vec<bool>, id: &mut Vec<usize>, w: usize, v: usize){
        // finds all reachable vertices from w and adds them to the connected component v
        // run time complexity O(sum of degrees of all reachable vertices from v)

        // mark vertex v as visited
        marked[w] = true;

        // recursively visit all unmarked adjacent vertices to v
        let adjacent_vertices = graph.vertex_edges(&w);
        for u in adjacent_vertices{
            if !marked[*u] {
                Self::dfs_cc(graph, marked, id, *u, v);
                id[*u] = v;
            }
        }
    }

}
