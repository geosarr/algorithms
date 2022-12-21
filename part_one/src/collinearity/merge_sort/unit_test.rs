#[cfg(test)]
mod tests {
    use super::super::super::*;
    use rand::Rng;

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
        let vec1 = mbup.into_sorted_vec();
        let vec2 = mrec.into_sorted_vec();
        v.sort(); // std sort of a vec
        assert_eq!(vec1, v);
        assert_eq!(vec2, v);
    }
}
