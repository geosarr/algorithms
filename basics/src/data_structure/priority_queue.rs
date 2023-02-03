#[cfg(test)]
mod unit_test;
mod kind;
pub use kind::PriorityQueueKind;
use std::mem::replace;
use std::collections::BinaryHeap;

/// Implementation of priority queues with the standard library 
/// # Examples
/// ```
/// use basics::data_structure::priority_queue::BinaryHeapPriorityQueue;
/// let mut bhqueue = BinaryHeapPriorityQueue::with_capacity(3);
/// assert_eq!(bhqueue.len(), 0);
/// bhqueue.insert(0);
/// bhqueue.insert(1);
/// bhqueue.insert(2);
/// assert_eq!(bhqueue.len(), 3);
/// assert_eq!(bhqueue.delete(), Some(2));
/// assert_eq!(bhqueue.delete(), Some(1));
/// assert_eq!(bhqueue.len(), 1);
/// ```
/// By default the priority queue is **max oriented**, the user should adapt 
/// the `Ord` trait to implement a min oriented priority queues. 
#[derive(Debug)]
pub struct BinaryHeapPriorityQueue<T>{
    heap: BinaryHeap<T>,
}

impl<T: Ord> BinaryHeapPriorityQueue<T>{
    /// Creates an empty priority queue instance.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::BinaryHeapPriorityQueue;
    /// let bhqueue = BinaryHeapPriorityQueue::<usize>::new();
    /// assert_eq!(bhqueue.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self{
            heap: BinaryHeap::<T>::new(),
        }
    }

    /// Creates a new empty priority queue with an initial size.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::BinaryHeapPriorityQueue;
    /// let bhqueue = BinaryHeapPriorityQueue::<&str>::with_capacity(1);
    /// assert_eq!(bhqueue.len(), 0);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::<T>::with_capacity(capacity),
        }
    }

    /// Tests whether or not the priority queue is empty.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::BinaryHeapPriorityQueue;
    /// let mut bhqueue = BinaryHeapPriorityQueue::<usize>::new();
    /// bhqueue.insert(1);
    /// assert!(!bhqueue.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool{
        self.heap.is_empty()
    }

    /// Gives the number of objects in the priority queue.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::BinaryHeapPriorityQueue;
    /// let mut bhqueue = BinaryHeapPriorityQueue::<isize>::new();
    /// bhqueue.insert(-1);
    /// bhqueue.insert(-2);
    /// bhqueue.insert(-4);
    /// assert_eq!(bhqueue.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Inserts an object into the priority queue.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::BinaryHeapPriorityQueue;
    /// let mut bhqueue = BinaryHeapPriorityQueue::<isize>::new();
    /// bhqueue.insert(-1);
    /// bhqueue.insert(-2);
    /// assert_eq!(bhqueue.len(), 2);
    /// ```
    pub fn insert(&mut self, key: T) {
        self.heap.push(key)
    }

    /// Deletes and returns the extremal (smallest in min oriented heap 
    /// and largest in max oriented heap) object in the priority queue, if any.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::BinaryHeapPriorityQueue;
    /// let mut bhqueue = BinaryHeapPriorityQueue::new();
    /// bhqueue.insert(0);
    /// bhqueue.insert(1);
    /// assert_eq!(bhqueue.delete(), Some(1));
    /// ```
    pub fn delete(&mut self) -> Option<T> {
        self.heap.pop()
    }

    /// Returns the extremal (smallest in min oriented heap 
    /// and largest in max oriented heap) object in the priority queue, if any.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::BinaryHeapPriorityQueue;
    /// let mut bhqueue = BinaryHeapPriorityQueue::new();
    /// bhqueue.insert(0);
    /// bhqueue.insert(1);
    /// assert_eq!(bhqueue.extremum(), Some(&1));
    /// ```
    /// # Time complexity
    /// This is expected to run in O(1)
    pub fn extremum(&self) -> Option<&T> {
        self.heap.peek()
    }

}











/// Implementation of priority queues using a `Vec` structure
/// # Examples
/// ```
/// use basics::data_structure::priority_queue::{PriorityQueue, PriorityQueueKind};
/// let mut bhqueue = PriorityQueue::with_capacity(3, PriorityQueueKind::Max);
/// assert_eq!(bhqueue.len(), 0);
/// bhqueue.insert(0);
/// bhqueue.insert(1);
/// bhqueue.insert(2);
/// assert_eq!(bhqueue.len(), 3);
/// assert_eq!(bhqueue.delete(), Some(2));
/// assert_eq!(bhqueue.delete(), Some(1));
/// assert_eq!(bhqueue.len(), 1);
/// ```
#[derive(Debug, Default, Clone)]
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

impl<T> PriorityQueue<T> {
    /// Creates a new empty priority queue with an initial size.
    /// # Panics
    /// If `capacity = 0`, then it panics.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::{PriorityQueue, PriorityQueueKind};
    /// let bhqueue = PriorityQueue::<&str>::with_capacity(1, PriorityQueueKind::Min);
    /// assert_eq!(bhqueue.len(), 0);
    /// ```
    pub fn with_capacity(capacity: usize, k: PriorityQueueKind) -> Self {
        // running time complexity: O(N)
        if capacity > 0 {
            let mut vector = Vec::with_capacity(capacity+1);
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

    /// Tests whether or not the priority queue is empty.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::{PriorityQueue, PriorityQueueKind};
    /// let mut bhqueue = PriorityQueue::<usize>::with_capacity(1, PriorityQueueKind::Min);
    /// bhqueue.insert(1);
    /// assert!(!bhqueue.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.n == 1
    }

    /// Gives the number of objects in the priority queue.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::{PriorityQueue, PriorityQueueKind};
    /// let mut bhqueue = PriorityQueue::<isize>::with_capacity(3, PriorityQueueKind::Min);
    /// bhqueue.insert(-1);
    /// bhqueue.insert(-2);
    /// bhqueue.insert(-4);
    /// assert_eq!(bhqueue.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        // number of objects in the queue
        // run time complexity O(1)
        self.n - 1
    }

    /// Returns the extremal (smallest in min oriented heap 
    /// and largest in max oriented heap) object in the priority queue, if any.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::{PriorityQueue, PriorityQueueKind};
    /// let mut bhqueue = PriorityQueue::<isize>::with_capacity(3, PriorityQueueKind::Min);
    /// bhqueue.insert(0);
    /// bhqueue.insert(1);
    /// assert_eq!(bhqueue.extremum(), Some(&0));
    /// ```
    /// # Time complexity
    /// This is expected to run in O(1)
    pub fn extremum(&self) -> Option<&T>{
        // run time complexity O(1)
        self.vec[1].as_ref()
    }

    fn double(&mut self) {
        // run time complexity O(N)
        // doubling the size of the priority queue
        let mut vector = Vec::with_capacity(self.vec.len());
        for _ in 0..self.vec.len() {
            vector.push(None);
        }
        self.vec.append(&mut vector);
    }

    fn halve(&mut self){
        // run time complexity O(N)
        // halving the size of the priority queue
        self.vec.truncate(self.vec.len()/2);
    }
}

impl<T: Ord + Clone> PriorityQueue<T>{
    fn swim(&mut self, mut k: usize){
        // moves data at position k up in the "tree" following the
        // Peter principle: Nodes are promoted to their level of incompetence
        // run time complexity O(log(N))
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

    /// Inserts an object into the priority queue.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::{PriorityQueue, PriorityQueueKind};
    /// let mut bhqueue = PriorityQueue::<isize>::with_capacity(3, PriorityQueueKind::Min);
    /// bhqueue.insert(-1);
    /// bhqueue.insert(-2);
    /// assert_eq!(bhqueue.len(), 2);
    /// ```
    /// # Time complexity
    /// This is expected to run in O(log(N)) on average
    pub fn insert(&mut self, key: T){
        // run time complexity O(log(N)) (without resizing)
        // and O(N) with resizing
        if self.n < self.vec.len() {
            self.vec[self.n] = Some(key);
            self.swim(self.n);
            self.n += 1;
            if self.n == self.vec.len() {
                // resize the stack to allow more capacity
                self.double();
            }
        } else {
            panic!("cannot push, stack is full or has capacity 0");
        }
    }

    fn sink(&mut self, mut k: usize, n: usize){
        // moves data at position k down in the "tree" following the
        // Power struggle principle: Better nodes are promoted
        // Nodes beyond node n are untouched.
        // run time complexity O(log(N))
        if self.is_empty(){
            panic!("cannot sink data, queue is empty.")
        } else {
            match self.kind {
                PriorityQueueKind::Max => {
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
                },
                PriorityQueueKind::Min => {
                    while 2*k < n {
                        let mut j = 2*k;
                        // find the smallest child of node k
                        if j < n - 1 && self.vec[j] > self.vec[j+1]{ j += 1;}
                        // compare it to node k
                        if self.vec[k] <= self.vec[j] {break;}
                        // exchange them if it is smaller than node k
                        let val = self.vec[k].clone();
                        self.vec[k] = replace(&mut self.vec[j], val);
                        k = j;
                    }
                },
            }
        }
    }
    
    /// Deletes and returns the extremal (smallest in min oriented heap 
    /// and largest in max oriented heap) object in the priority queue, if any.
    /// Returns `None` otherwise.
    /// # Example
    /// ```
    /// use basics::data_structure::priority_queue::{PriorityQueue, PriorityQueueKind};
    /// let mut bhqueue = PriorityQueue::<isize>::with_capacity(3, PriorityQueueKind::Min);
    /// bhqueue.insert(0);
    /// bhqueue.insert(1);
    /// assert_eq!(bhqueue.delete(), Some(0));
    /// ```
    /// # Time complexity
    /// This is expected to run in O(log(N)) on average
    pub fn delete(&mut self) -> Option<T>{
        // delete the extremal value and returns it
        // run time complexity O(log(N))
        if self.is_empty() {
            panic!("cannot delete, queue is empty");
        } else {
            let res = self.vec[1].clone();
            // Put the last object at the beginning of the root of the tree
            self.vec[1] = replace(&mut self.vec[self.n-1], None);
            // sink the root object
            self.sink(1, self.n);
            self.n -= 1;
            if self.n <= self.vec.len()/4{
                self.halve();
            } 
            res
        }
    }
}











// Implementation of priority queue with unordered vec
#[derive(Debug, Default)]
struct UnorderedVecPriorityQueue<T>{
    // vector of objects
    vec: Vec<Option<T>>,
    // type of priority queue
    kind: PriorityQueueKind,
    // number of objects in the priority queue
    n: usize,
    // the maximum or minimum value object
    extremum: Option<T>
}

impl<T> UnorderedVecPriorityQueue<T> {
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

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn len(&self) -> usize {
        // number of objects in the queue
        // run time complexity O(1)
        self.n 
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


impl<T: Ord + Clone> UnorderedVecPriorityQueue<T>{ 
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
}