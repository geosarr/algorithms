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
    fn test_heap_sort(){
        let mut vec = gen_vec_rand_int(100000);
        let h = HeapSort::init(vec.clone());
        let v = h.into_sorted_vec();
        vec.sort();
        assert_eq!(vec, v);
    }

    #[test]
    fn test_binary_heap_sort(){
        let mut vec = gen_vec_rand_int(100000);
        let h = BinaryHeapSort::init(vec.clone());
        let v = h.into_sorted_vec();
        vec.sort();
        assert_eq!(vec, v);
    }
}