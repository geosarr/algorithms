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

    // #[test]
    fn test_quick_sort() {
        let mut v = gen_vec_rand_int(10);
        let mut quick = QuickSort {
            vec: v.clone(),
            // algo: MergeSortAlgorithm::BottomUp,
        };
        quick.sort();
        v.sort(); // std sort of a vec
        assert_eq!(quick.vec, v);
    }

    // #[test]
    fn test_quick_select() {
        let mut v = gen_vec_rand_int(1000);
        let mut quick = QuickSort {
            vec: v.clone(),
            // algo: MergeSortAlgorithm::BottomUp,
        };
        let med = quick.select(500);
        v.sort(); // std sort of a vec
                  // assert!(med == v[499] || med == v[500]);
        assert_eq!(med, v[500]);
    }
}
