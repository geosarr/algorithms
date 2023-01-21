#[cfg(test)]
mod unit_test;
pub struct DirectedGraph{
    // 
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
}