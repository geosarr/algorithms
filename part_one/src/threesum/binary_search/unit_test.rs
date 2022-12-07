#[cfg(test)]
mod tests {
    use super::super::super::*;
    use rand::Rng;
    use std::thread;
    use std::thread::JoinHandle;

    enum RandKind {
        Range,
        Full
    }

    fn gen_vec_rand_int(n: usize, kind: RandKind) -> Vec<isize>{
        // Generate random integers within a range or not
        let mut rng = rand::thread_rng();
        let mut vec: Vec<isize> = Vec::new();
        if let RandKind::Range = kind {
            for i in 0..n{
                vec.push(rng.gen_range(-10000000isize..10000000isize));
        }} else {
            for i in 0..n{
                vec.push(rng.gen::<isize>());
        }} 
        vec
    }

    #[test]
    fn test_binary_search(){
        let mut vec = gen_vec_rand_int(1000000, RandKind::Full);
        vec.sort_unstable();
        for i in 0..vec.len(){
            let ind = binary_search(vec[i], &vec);
            assert_eq!(ind, i as isize);
        }
        assert_eq!(-1, binary_search(vec[0]-1, &vec)); // may fail if vec[0] = min isize
    }

    #[test]
    fn test_threesum(){
        let mut vec = gen_vec_rand_int(10, RandKind::Range);
        let n = vec.len();
        vec.sort_unstable();
        let mut threesum = ThreeSum{target: 0, vec: vec};
        let mut handles: Vec<JoinHandle<isize>> = Vec::new();
        for i in 0..n {
            let a = threesum.vec[i];
            for j in (i+1)..n {
                let b = threesum.vec[j];
                for k in (j+1).. n{
                    let c = threesum.vec[k];
                    let handle = thread::spawn(move || {
                        a+b+c
                    });
                handles.push(handle);
        }}}
        let sums = handles.into_iter()
                          .map(|h| h.join().unwrap())
                          .collect::<Vec<isize>>();
        for s in &sums {
            threesum.target = *s;
            assert!(1 <= threesum.run());
        }
    }
}