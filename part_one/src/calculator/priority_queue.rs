#[cfg(test)]
mod unit_test;
pub mod kind;
pub use kind::PriorityQueueKind;
use std::mem::replace;


// Implementation using Binary Heap data structure
#[derive(Debug, Default)]
pub struct PriorityQueue<T>{
    // vector of objects
    vec: Vec<Option<T>>,
    // type of priority queue
    kind: PriorityQueueKind,
    // position of the next object in the queue (or 1 + number of objects)
    n: usize,

    // Remarks:
    // - objects are nodes of the tree
    // - in the implementation objects are stored in self.vec from index = 1 to index = capacity 
    //   so that index = 0 is always None object and:
    //     - each node k's parent is at position k/2
    //     - each node k's children are at positions 2k and 2k+1
    // - in the max oriented binary heap (with kind = PriorityQueueKind::Max), 
    //   parents are larger than their children (smaller for min oriented heap) 
}

impl<T: PartialOrd + Clone> PriorityQueue<T>{
    pub fn init(capacity: usize, k: PriorityQueueKind) -> Self {
        // running time complexity: O(N)
        if capacity > 0 {
            let mut vector = Vec::new();
            for _ in 0..capacity+1 {
                vector.push(None);
            }

            Self {
                vec: vector, 
                kind: k,
                n: 1,
            }
        } else {
            panic!("capacity shoul be > 0");
        }
    }

    pub fn is_empty(&self) -> bool{
        self.n == 1
    }

    pub fn len(&self) -> usize {
        // number of objects in the queue
        // run time complexity O(1)
        self.n - 1
    }

    fn swim(&mut self, mut k: usize){
        // moves data at position k up in the "tree" following the
        // Peter principle: Nodes are promoted to their level of incompetence
        match self.kind {
            PriorityQueueKind::Max => {
                while k > 1 && self.vec[k] > self.vec[k/2]{
                    let val = self.vec[k].clone();
                    self.vec[k] = replace(&mut self.vec[k/2], val);
                    k = k/2;
                }
            },
            PriorityQueueKind::Min => { 
                while k > 1 && self.vec[k] < self.vec[k/2]{
                    let val = self.vec[k].clone();
                    self.vec[k] = replace(&mut self.vec[k/2], val);
                    k = k/2;
                }
            },
        }
    }

    pub fn insert(&mut self, key: T){
        if self.n < self.vec.len() {
            self.vec[self.n] = Some(key);
            self.swim(self.n);
            self.n += 1;
            if self.n == self.vec.len() {
                // resize the stack to allow more capacity
                self.resize();
            }
        } else {
            panic!("cannot push, stack is full or has capacity 0");
        }
    }

    fn sink(&mut self, mut k: usize){
        // moves data at position k down in the "tree" following the
        // Power struggle principle: Better nodes are promoted
        if self.is_empty(){
            panic!("cannot sink data, queue is empty.")
        } else {
            match self.kind {
                PriorityQueueKind::Max => {
                    while 2*k < self.n {
                        let mut j = 2*k;
                        if j < self.n - 1 && self.vec[j] < self.vec[j+1]{ j += 1;}
                        if k >= j {break;}
                        let val = self.vec[k].clone();
                        self.vec[k] = replace(&mut self.vec[j], val);
                        k = j;
                    }
                },
                PriorityQueueKind::Min => {
                    while 2*k < self.n {
                        let mut j = 2*k;
                        if j < self.n - 1 && self.vec[j] > self.vec[j+1]{ j += 1;}
                        if k >= j {break;}
                        let val = self.vec[k].clone();
                        self.vec[k] = replace(&mut self.vec[j], val);
                        k = j;
                    }
                },
            }
        }
    }
    pub fn delete(&mut self){

    }

    pub fn extremum(&self) -> Option<T>{
        // run time complexity O(1)
        self.vec[1].clone()

    }

    fn resize(&mut self) {
        // run time complexity O(N)
        // doubling the size of the stack when it is full
        let mut vector = Vec::new();
        for _ in 0..self.vec.len() {
            vector.push(None);
        }
        self.vec.append(&mut vector);
    }


}





#[derive(Debug, Default)]
pub struct UnorderedVecPriorityQueue<T>
where T: PartialOrd
{
    // vector of objects
    vec: Vec<Option<T>>,
    // type of priority queue
    kind: PriorityQueueKind,
    // number of objects in the priority queue
    n: usize,
    // the maximum or minimum value object
    extremum: Option<T>
}

impl<T: PartialOrd + Clone> UnorderedVecPriorityQueue<T>{ 
    pub fn init(capacity: usize, k: PriorityQueueKind) -> Self {
        // running time complexity: O(N)
        if capacity > 0 {
            let mut vector = Vec::new();
            for _ in 0..capacity {
                vector.push(None);
            }

            Self {
                vec: vector, 
                kind: k,
                n: 0,
                extremum: None
            }
        } else {
            panic!("capacity shoul be > 0");
        }
    }
    pub fn is_empty(&self) -> bool{
        // run time complexity O(1)
        self.n == 0
    }

    pub fn insert(&mut self, key: T){
        // inserts a key 
        // run time complexity O(1) (without resizing)
        // with resizing O(N)
        if self.n < self.vec.len() {
            let _ = replace(&mut self.vec[self.n], Some(key));
            self.n += 1;
            if self.n == self.vec.len() {
                // resize the stack to allow more capacity
                self.resize();
            }
        } else {
            panic!("cannot push, stack is full or has capacity 0");
        }
    }

    pub fn len(&self) -> usize {
        // number of objects in the queue
        // run time complexity O(1)
        self.n
    }

    pub fn delete(&mut self) -> Option<T>{
        // deletes the max if kind = Max or the minimum if kind = min
        // run time complexity O(N)
        if self.is_empty() {
            panic!("cannot delete, queue is empty");
        } else {
            let mut m = 0; // extremum (largest or lowest)
            let mut prec_m = 0; // second extremum (second largest or second lowest)
            match self.kind {
                // this procedure is not in-place, meaning that equal extreme value objects
                // will imply value modification (with the replace function). This is due to
                // the comparisons <= or >=, they could be swapped with their strict counterparts
                // < or > respectively, but this procedure makes easy to find the extrema in the
                // self.extremum() function   
                PriorityQueueKind::Max => for i in 1..self.n{
                    if self.vec[i] >= self.vec[m]{
                        prec_m = replace(&mut m, i);
                    }
                    else if self.vec[i] >= self.vec[prec_m]{
                        prec_m = i;
                    }
                },
                PriorityQueueKind::Min => for i in 1..self.n{
                    if self.vec[i] <= self.vec[m]{
                        prec_m = replace(&mut m, i);
                    }
                    else if self.vec[i] <= self.vec[prec_m]{
                        prec_m = i;
                    }
                },
            }
            
            // helps speed up extremum finding
            self.extremum = self.vec[prec_m].clone();

            let val = self.vec[m].clone(); 
            if m < self.n - 1{
                self.vec[m] = replace(&mut self.vec[self.n-1], None);
            } else {
                self.vec[m] = None;
            }
            self.n -= 1;
            val
    }}

    pub fn extremum(&mut self) -> Option<T>{
        // finds the maximum/minimum object
        // run time complexity O(N) (in general), 
        // when speed up (e.g. when self.delete() is called), its O(1)
        if !self.extremum.is_none() {
            self.extremum.clone()
        } else if !self.is_empty(){
            let mut m = 0;
            match self.kind{
                PriorityQueueKind::Max => for i in 1..self.n{
                    if self.vec[i] > self.vec[m]{
                        m = i;
                }},
                PriorityQueueKind::Min => for i in 1..self.n{
                    if self.vec[i] < self.vec[m]{
                        m = i;
                }},
            }
            self.extremum = self.vec[m].clone();
            self.extremum.clone()
        } else {
            None
        }
    }

    fn resize(&mut self) {
        // run time complexity O(N)
        // doubling the size of the stack when it is full
        let mut vector = Vec::new();
        for _ in 0..self.vec.len() {
            vector.push(None);
        }
        self.vec.append(&mut vector);
    }
}