#[cfg(test)]
mod tests{
    use crate::utils::{Wordnet};
    use crate::utils::wordnet::ShortestAncestralPath;
    use crate::DirectedGraph;
    
    #[test]
    fn test_wordnet(){
        let w = Wordnet::from_file(
            "/home/georges/github/algorithms/advanced/src/synsets.txt", 
            "/home/georges/github/algorithms/advanced/src/hypernyms.txt", ',', ',', 100000);

        assert!(w.is_noun("theory_of_probability"));
  
        println!("{:?}", ShortestAncestralPath::new().ancestor(&w.hypernym_graph, 9992,9993));
        println!("{:?}", w.sap_distance("Percival_Lowell", "Robert_Lowell"));
    }
}