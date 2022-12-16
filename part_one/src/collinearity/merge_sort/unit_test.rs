#[cfg(test)]
mod tests {
    use super::super::super::*;
    use rand::Rng;
    use std::thread;

    fn gen_vec_rand_int(n: usize) -> Vec<isize> {
        // Generate random integers
        let mut rng = rand::thread_rng();
        let mut vec: Vec<isize> = Vec::new();
        for _ in 0..n {
            vec.push(rng.gen::<isize>());
        }
        vec
    }

    #[test]
    fn test_merge_sort() {
        let mut v = gen_vec_rand_int(10000);
        let mut mbup = MergeSort {
            vec: v.clone(),
            algo: MergeSortAlgorithm::BottomUp,
        };
        let mut mrec = MergeSort {
            vec: v.clone(),
            algo: MergeSortAlgorithm::Recursive,
        };
        mbup.sort();
        mrec.sort();
        v.sort(); // std sort of a vec
        assert_eq!(mbup.vec, v);
        assert_eq!(mrec.vec, v);
    }
}
