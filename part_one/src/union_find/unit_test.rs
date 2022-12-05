#[cfg(test)]
mod tests {
    use super::super::{*};

    #[test]
    fn test_new() {
        let uf = UnionFind::new();
        assert_eq!(uf.ids.len(), uf.nb_objects);
    }

    #[test]
    fn test_init() {
        let n = 100;
        let qf = UnionFind::init(n, Algorithm::QuickFind);
        assert_eq!(qf.size.len(), 0);
        assert_eq!(qf.nb_objects, n);
        assert_eq!(qf.ids.len(), n);
        let qu = UnionFind::init(n, Algorithm::QuickUnion);
        assert_eq!(qu.size.len(), n);
        assert_eq!(qu.nb_objects, n);
        assert_eq!(qu.ids.len(), n);
    }

    // #[test]
    // fn 

    

    
}