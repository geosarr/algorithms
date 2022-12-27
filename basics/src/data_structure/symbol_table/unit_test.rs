#[cfg(test)]
mod tests{
    use super::super::{OrderedVecSymbolTable, BinarySearchTree, BTreeSymbolTable};

    #[test]
    fn test_ordered_vec_symbol_table(){
        let mut st = OrderedVecSymbolTable::<usize, String>::init(1,"test".to_owned());
        assert_eq!(st.len(), 1);
        assert!(st.contains(1));
        st.insert(0, "test2".to_owned());
        assert_eq!(st.vec[0].get_first(), Some(&0));
        assert_eq!(st.vec[0].get_second(), Some(&Some("test2".to_owned())));
        st.delete(0);
        assert_eq!(st.vec[0].get_first(), Some(&0));
        assert_eq!(st.vec[0].get_second(), Some(&None));
    }

    #[test]
    fn test_binary_search_tree(){
        let mut st = BinarySearchTree::<usize, String>::init(1,"test".to_owned());
        assert_eq!(st.len(), 1);
        assert_eq!(st.get(&1), Some(&"test".to_owned()));
        assert_eq!(st.get(&0), None);
    
    }

    #[test]
    fn test_b_tree_symbol_table(){
        assert!(BTreeSymbolTable::<usize, String>::new().is_empty());
        let mut st = BTreeSymbolTable::<usize, &str>::init(1,"test");
        assert_eq!(st.len(), 1);
        assert!(!st.is_empty());
        assert_eq!(st.get(&1), Some(&"test"));
        assert_eq!(st.get(&0), None);
    }
}
