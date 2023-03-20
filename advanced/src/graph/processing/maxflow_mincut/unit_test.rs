#[cfg(test)]

mod tests {
    use super::super::{FlowNetwork, FordFulkerson};

    #[test]
    fn test_ford_fulkerson() {
        let mut network = FlowNetwork::init(6);
        // From CLRS book
        network.add_edge(0, 1, 11, 16);
        network.add_edge(0, 2, 8, 13);
        network.add_edge(1, 3, 12, 12);
        network.add_edge(2, 1, 1, 4);
        network.add_edge(2, 4, 11, 14);
        network.add_edge(3, 2, 4, 9);
        network.add_edge(3, 5, 15, 20);
        network.add_edge(4, 3, 7, 7);
        network.add_edge(4, 5, 4, 4);

        // println!("{:?}", network.nb_edges());
        // println!("{:?}", network.nb_vertices());

        let mut ford_fulkerson = FordFulkerson::<usize>::init(0, 5, network.nb_vertices());
        ford_fulkerson.find_flows(&network);
        assert_eq!(Some(23), ford_fulkerson.value());
    }
}
