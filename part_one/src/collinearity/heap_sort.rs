#[cfg(test)]
mod unit_test;
use std::mem::replace;
use std::collections::BinaryHeap;

// Implementation with std BinaryHeap
#[derive(Debug)]
pub struct BinaryHeapSort<T> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> BinaryHeapSort<T>{
    pub fn init(v: Vec<T>) -> Self {
        Self {
            heap: BinaryHeap::from(v),
        }
    }

    pub fn into_sorted_vec(mut self) -> Vec<T>{
        self.heap.into_sorted_vec()
    } 
}

// Implementation "from scratch" using a max oriented binary heap
#[derive(Debug, Default)]
pub struct HeapSort<T> {
    vec: Vec<T>,
}

impl<T: PartialOrd + Default + Clone> HeapSort<T> {
    pub fn init(mut v: Vec<T>) -> Self {
        let mut new_vec = vec![T::default()]; // index 0 is not used
        new_vec.append(&mut v);
        Self {
            vec: new_vec,
        }
    }

    fn sink(&mut self, mut k: usize, n: usize){
        // moves data at position k down in the "tree" following the
        // Power struggle principle: Better nodes are promoted
        // Nodes beyond node n are untouched.
        // run time complexity O(log(N))
        while 2*k < n {
            let mut j = 2*k;
            // find the largest child of node k
            if j < n - 1 && self.vec[j] < self.vec[j+1]{ j += 1;}
            // compare it to node k
            if self.vec[k] >= self.vec[j] {break;}
            // exchange them if it is larger than node k
            let val = self.vec[k].clone();
            self.vec[k] = replace(&mut self.vec[j], val);
            k = j;
        }
    }

    fn sink_all(&mut self) {
        // design a Binary Heap from self.vec
        // run time complexity O(N log(N))
        let n = self.vec.len();
        for j in (1..n/2).rev(){
            let new_j = j;
            self.sink(new_j, n);
        }
    }

    fn sort_down(mut self) -> Vec<T>{
        let mut n = self.vec.len();
        // run time complexity O(N log(N))
        while n > 1 {
            let val = self.vec[1].clone();
            self.vec[1] = replace(&mut self.vec[n-1], val);
            n -= 1;
            self.sink(1, n);
        }
        self.vec[1..].into()
    }

    pub fn into_sorted_vec(mut self) -> Vec<T>{
        // run time complexity O(N log(N))
        self.sink_all();
        self.sort_down()    
    }
}