#[cfg(test)]
mod tests{
    use super::super::{UndirectedGraph};

    #[test]
    fn test_undirected_graph(){
        let n: usize = 10;
        let mut graph = UndirectedGraph::init(n);
        assert_eq!(graph.nb_vertices(), n);
        graph.add_edge(0, 5);
        graph.add_edge(4, 8);
        graph.add_edge(7, 4);
        assert_eq!(graph.nb_edges(), 3);
        assert_eq!(graph.degree(&1), 0);
        assert_eq!(graph.degree(&4), 2);
        assert_eq!(graph.self_loop_number(), 0);
        graph.add_edge(0, 0);
        assert_eq!(graph.self_loop_number(), 1);
    }

    #[test]
    #[should_panic]
    fn test_undirected_graph_panic1(){
        let n: usize = 2;
        let mut graph = UndirectedGraph::init(n);
        graph.add_edge(4,1);
    }

    #[test]
    #[should_panic]
    fn test_undirected_graph_panic2(){
        let graph = UndirectedGraph::new();
        graph.average_degree();
    }

    #[test]
    fn test_dfs(){
        
    }
}