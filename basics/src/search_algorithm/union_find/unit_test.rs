#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_new() {
        let uf = UnionFind::new();
        assert_eq!(uf.ids.len(), uf.nb_objects);
    }

    #[test]
    fn test_init() {
        let n = 100;
        let qf = UnionFind::init(n, UnionFindAlgorithm::QuickFind);
        assert_eq!(qf.size.len(), 0);
        assert_eq!(qf.nb_objects, n);
        assert_eq!(qf.ids.len(), n);
        let qu = UnionFind::init(n, UnionFindAlgorithm::QuickUnion);
        assert_eq!(qu.size.len(), n);
        assert_eq!(qu.nb_objects, n);
        assert_eq!(qu.ids.len(), n);
    }

    #[test]
    fn test_root_connected_union() {
        let mut uf = UnionFind::init(6, UnionFindAlgorithm::QuickUnion);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(1, 3);
        uf.union(4, 5);
        assert_eq!(uf.connected(0, 3), true);
        assert_eq!(uf.connected(1, 2), true);
        assert_eq!(uf.connected(0, 4), false);
    }
}
