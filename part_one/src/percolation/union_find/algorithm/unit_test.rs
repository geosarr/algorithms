#[cfg(test)]
mod tests {
    use super::super::*; 

    #[test]
    fn test_algo_format(){
        let algo = Algorithm::QuickFind;
        assert_eq!(algo.to_string(), "QuickFind");
        let algo = Algorithm::QuickUnion;
        assert_eq!(algo.to_string(), "QuickUnion");
        let algo = Algorithm::WeightedQuickUnion;
        assert_eq!(algo.to_string(), "WeightedQuickUnion");
        let algo = Algorithm::WeightedQuickUnionPathComp;
        assert_eq!(algo.to_string(), "WeightedQuickUnionPathComp");
    }

    #[test]
    fn test_parse_str_to_algo(){
        let bad_algo = Algorithm::from_str("NotImplementedAlgo");
        let error =  Err(ParseAlgorithmError);
        assert_eq!(bad_algo, error);
    }
}
