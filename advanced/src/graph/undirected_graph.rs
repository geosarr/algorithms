#[cfg(test)]
mod unit_test;
use std::collections::HashSet;
use std::collections::LinkedList;

pub struct UndirectedGraph{
    // implements an adjacency-list graph
    // where vertices have indices 0, ..., nb_objects
    // and each vertex is associated to its adjacent vertices
    data: Vec<HashSet<usize>>,
    nb_edges: usize,
    nb_vertices: usize,
}
impl UndirectedGraph{
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
        for _ in 0..nb_objects{
            graph.data.push(HashSet::<usize>::new());
        }
        graph
    }

    pub fn nb_edges(&self) -> usize{
        // run time complexity O(1)
        self.nb_edges
    }

    pub fn nb_vertices(&self) -> usize{
        // run time complexity O(1)
        self.nb_vertices
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        // adds an edge to the graph
        // run time complexity O(1)
        self.data[v].insert(w);
        self.data[w].insert(v);
        self.nb_edges += 1;
    }

    pub fn vertex_edges<'a>(&'a self, v: &usize) -> &'a HashSet<usize>{
        // gets all the vertices linked to a given vertex v, 
        // that is the adjacent vertices of v 
        // run time complexity O(1)
        &self.data[*v]
    }

    pub fn degree(&self, v: &usize) -> usize{
        self.vertex_edges(v).len()
    }

    pub fn average_degree(&self) -> f32{
        // gets the average number of degree of the graph
        // each edge is counted only once (by the self.add_edge() method) 
        if self.nb_vertices > 0 {
            (self.nb_edges as f32) / (self.nb_vertices as f32)
        } else {
            panic!("No vertex in the graph");
        }
    }

    pub fn self_loop_number(&self) -> usize{
        self.data.iter().enumerate().map(|(v,e)| if e.contains(&v){1} else{0}).sum()
    }
}





pub struct DepthFirstSearch{
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Indicates what is the previous vertex leading to the current vertex 
    // when edge_to[w]=w, then no path is found yet from v to w
    edge_to: Vec<usize>,
    // vertex for which paths are computed
    v: usize
}
impl DepthFirstSearch{
    pub fn init(nb_vertices: usize, origin: usize) -> Self {
        Self {
            marked: vec![false;nb_vertices],
            edge_to: (0..nb_vertices).collect::<Vec<usize>>(), 
            v: origin
        }
    }

    pub fn find_paths(&mut self, graph: &UndirectedGraph){
        // finds all paths from self.v in self.graph
        Self::dfs(graph, &mut self.marked, &mut self.edge_to, &self.v);
    }

    fn dfs(graph: &UndirectedGraph, marked: &mut Vec<bool>, edge_to: &mut Vec<usize>, w: &usize){
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



pub struct BreadthFirstSearch{
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Indicates what is the previous vertex leading to the current vertex 
    // when edge_to[w]=w, then no path is found yet from v to w
    edge_to: Vec<usize>,
    // Vertex for which paths are computed
    v: usize,
}
impl BreadthFirstSearch{
    pub fn init(nb_vertices: usize, origin: usize) -> Self {
        Self {
            marked: vec![false;nb_vertices],
            edge_to: (0..nb_vertices).collect::<Vec<usize>>(), 
            v: origin,
        }
    }

    pub fn find_paths(&mut self, graph: &UndirectedGraph){
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



pub struct ConnectedComponent{
    // Aims at answering the question are two vertives v and w connected in contant time
    // after preprocessing the graph
    // Identifier of the connected commponent vertices belong to
    id: Vec<usize>,
    // Indicates wether or not a vertex w in the graph is visited
    marked: Vec<bool>,
    // Number of connected components
    nb_cc: usize,
    // Whether or not the algorithm has run
    ran: bool
}
impl ConnectedComponent{
    pub fn init(nb_vertices: usize) -> Self {
        Self {
            marked: vec![false;nb_vertices],
            id: (0..nb_vertices).collect::<Vec<usize>>(), 
            nb_cc: 0,
            ran: false
        }
    }
    pub fn find_cc(&mut self, graph: &UndirectedGraph){
        // builds all the connected components from a graph
        let nb = graph.nb_vertices;
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

    fn dfs_cc(graph: &UndirectedGraph, marked: &mut Vec<bool>, id: &mut Vec<usize>, w: usize, v: usize){
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
