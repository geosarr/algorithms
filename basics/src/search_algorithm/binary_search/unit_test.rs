#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::utils::{gen_vec_rand_int, RandKind};

    #[test]
    fn test_binary_search() {
        let mut vec = gen_vec_rand_int(1000000, RandKind::Full);
        vec.sort_unstable();
        for i in 0..vec.len() {
            let ind = binary_search(vec[i], &vec);
            assert_eq!(ind.unwrap(), i);
        }
        assert!(binary_search(vec[0] - 1, &vec).is_err());
    }
}
