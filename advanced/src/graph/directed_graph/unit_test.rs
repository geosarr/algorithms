#[cfg(test)]
mod tests{
    use super::super::DirectedGraph;
    use crate::graph::{
        DepthFirstSearch,
        BreadthFirstSearch, 
        LinkedList,
        TopologicalSort
    };

    #[test]
    fn test_directed_graph(){
        let n: usize = 10;
        let mut graph = DirectedGraph::init(n);
        assert_eq!(graph.nb_vertices(), n);
        graph.add_edge(0, 5);
        graph.add_edge(4, 8);
        graph.add_edge(7, 4);
        assert_eq!(graph.nb_edges(), 3);
        assert_eq!(graph.out_degree(&2), 0);
        assert_eq!(graph.out_degree(&4), 1);
        assert_eq!(graph.self_loop_number(), 0);
        graph.add_edge(0, 0);
        assert_eq!(graph.self_loop_number(), 1);
    }

    #[test]
    #[should_panic]
    fn test_directed_graph_panic1(){
        let n: usize = 2;
        let mut graph = DirectedGraph::init(n);
        graph.add_edge(4,1);
    }

    #[test]
    #[should_panic]
    fn test_directed_graph_panic2(){
        let graph = DirectedGraph::new();
        graph.average_degree();
    }

    #[test]
    fn test_dfs(){
        let mut graph = DirectedGraph::init(9);
        graph.add_edge(0,1);
        graph.add_edge(0,2);
        graph.add_edge(0,6);
        graph.add_edge(0,5);
        graph.add_edge(6,4);
        graph.add_edge(4,3);
        graph.add_edge(5,4);
        graph.add_edge(5,3);

        let mut dfs = DepthFirstSearch::init(graph.nb_vertices,0);
        dfs.find_paths(&graph);
        assert_eq!(dfs.path_to(2), Some(LinkedList::from([2,0])));
        assert!(dfs.path_to(4) == Some(LinkedList::from([4,6,0])) ||
            dfs.path_to(4) == Some(LinkedList::from([4,5,0])) ||
            dfs.path_to(4) == Some(LinkedList::from([4,3,5,0]))
        );
    }

    #[test]
    fn test_bfs(){
        let mut graph = DirectedGraph::init(9);
        graph.add_edge(0,1);
        graph.add_edge(0,2);
        graph.add_edge(0,6);
        graph.add_edge(0,5);
        graph.add_edge(6,4);
        graph.add_edge(4,3);
        graph.add_edge(5,4);
        graph.add_edge(5,3);

        let mut bfs = BreadthFirstSearch::init(graph.nb_vertices, 0);
        bfs.find_paths(&graph);
        assert_eq!(bfs.path_to(2), Some(LinkedList::from([2,0])));
        assert!(bfs.path_to(4) == Some(LinkedList::from([4,6,0])) ||
            bfs.path_to(4) == Some(LinkedList::from([4,5,0]))
        );
        assert_eq!(bfs.path_to(5), Some(LinkedList::from([5,0])));
    }


    #[test]
    fn test_topological_sort(){
        let mut graph = DirectedGraph::init(7);
        graph.add_edge(0,1);
        graph.add_edge(0,2);
        graph.add_edge(0,5);
        graph.add_edge(1,4);
        graph.add_edge(3,2);
        graph.add_edge(3,4);
        graph.add_edge(3,5);
        graph.add_edge(3,6);
        graph.add_edge(5,2);
        graph.add_edge(6,4);
        graph.add_edge(6,0);

        let mut topo = TopologicalSort::init(graph.nb_vertices);
        topo.depth_first_order(&graph);
        // println!("{:?}", topo.reverse_postorder());
        assert!(
            topo.reverse_postorder() == &LinkedList::from([4,1,2,5,0,6,3]) ||
            topo.reverse_postorder() == &LinkedList::from([2,5,4,1,0,6,3]) ||
            topo.reverse_postorder() == &LinkedList::from([2,4,1,5,0,6,3])                  
        );
    }
}