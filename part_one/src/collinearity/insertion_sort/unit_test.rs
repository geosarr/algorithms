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
    fn test_insertion_sort(){
        let mut v = gen_vec_rand_int(1000);
        let mut insert = InsertionSort {
            vec: v.clone(),
        };
        insert.sort();
        v.sort(); // std sort of a vec
        assert_eq!(insert.vec, v);
    }
}