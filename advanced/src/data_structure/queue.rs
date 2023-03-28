#[cfg(test)]
mod unit_test;

use std::collections::LinkedList;
// use std::mem::replace;

/// Implementation of the First In First Out concept (namely a queue),
/// with the standard library
/// # Examples
/// ```
/// use algods::data_structure::queue::LinkedListQueue;
/// let mut queue = LinkedListQueue::new();
/// assert_eq!(queue.len(), 0);
/// queue.enqueue(0);
/// queue.enqueue(1);
/// queue.enqueue(2);
/// assert_eq!(queue.len(), 3);
/// assert_eq!(queue.dequeue(), Some(0));
/// assert_eq!(queue.dequeue(), Some(1));
/// assert_eq!(queue.len(), 1);
/// ```
#[derive(Default, Clone, Debug)]
pub struct LinkedListQueue<T> {
    list: LinkedList<T>,
}

impl<T> LinkedListQueue<T> {
    /// Creates an empty queue instance.
    /// # Example
    /// ```
    /// use algods::data_structure::queue::LinkedListQueue;
    /// let queue = LinkedListQueue::<usize>::new();
    /// assert_eq!(queue.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    /// Creates a new queue with an initial object.
    /// # Example
    /// ```
    /// use algods::data_structure::queue::LinkedListQueue;
    /// let queue = LinkedListQueue::init("queue");
    /// assert_eq!(queue.len(), 1);
    /// ```
    pub fn init(s: T) -> Self {
        let mut res = Self {
            list: LinkedList::new(),
        };
        res.enqueue(s);
        res
    }

    /// Tests whether or not the queue is empty.
    /// # Example
    /// ```
    /// use algods::data_structure::queue::LinkedListQueue;
    /// let queue = LinkedListQueue::<usize>::new();
    /// assert!(queue.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Gives the number of objects in the queue.
    /// # Example
    /// ```
    /// use algods::data_structure::queue::LinkedListQueue;
    /// let queue = LinkedListQueue::<isize>::new();
    /// assert_eq!(queue.len(),0);
    /// ```
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Deletes and returns the first object in the queue, if any.
    /// Otherwise, it returns `None`.
    /// # Example
    /// ```
    /// use algods::data_structure::queue::LinkedListQueue;
    /// let mut queue = LinkedListQueue::init(1);
    /// assert_eq!(queue.dequeue(), Some(1));
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    /// Inserts an object at the end the queue.
    /// # Example
    /// ```
    /// use algods::data_structure::queue::LinkedListQueue;
    /// let mut queue = LinkedListQueue::<isize>::new();
    /// queue.enqueue(-1);
    /// queue.enqueue(-2);
    /// assert_eq!(queue.dequeue(), Some(-1));
    /// ```
    pub fn enqueue(&mut self, element: T) {
        self.list.push_back(element)
    }
}

// Implementing queues relatively "from scratch"
// #[derive(Debug, Clone)]
// struct Node<T> {
//     item: T,
//     // Box helps avoid infinity memory allocation
//     // in a recursive definition of a struct
//     next: Option<Box<Node<T>>>,
// }

// #[derive(Default, Debug, Clone)]
// struct Queue<T> {
//     first: Option<Box<Node<T>>>,
//     last: Option<Box<Node<T>>>,
//     len: usize,
// }

// impl<T: Clone + Default> Queue<T> {
//     pub fn new() -> Self {
//         // run time complexity O(1)
//         Default::default()
//     }

//     pub fn init(s: T) -> Self {
//         // run time complexity O(1)
//         let node = Node {
//             item: s,
//             next: None,
//         };
//         Self {
//             first: Some(Box::new(node)),
//             last: None,
//             len: 1,
//         }
//     }

//     pub fn is_empty(&self) -> bool {
//         // run time complexity O(1)
//         self.first.is_none()
//     }

//     pub fn len(&self) -> usize {
//         self.len
//     }

//     pub fn dequeue(&mut self) -> Option<T> {
//         // run time complexity O(N) (due to cloning) ?
//         // space complexity O(N) (due to cloning) ?
//         match self.first {
//             Some(ref node) => {
//                 let item = node.item.clone();
//                 self.first = node.next.clone();
//                 self.len -= 1;
//                 Some(item)
//             }
//             None => panic!("cannot pop, queue is empty"),
//         }
//     }

//    // Does not work yet !!!!
//     pub fn enqueue(&mut self, element: T){

//         let node = Node{item: element, next: None};
//         let node = Some(Box::new(node));
//         if self.is_empty(){
//             self.first = node;
//         } else {
//             let mut oldlast = replace(&mut self.last, node);
//             if let Some(ref mut old) = oldlast {
//                 old.next = self.last.clone();
//             }
//         }
//         self.len += 1;
//     }
// }
