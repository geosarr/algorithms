#[cfg(test)]
mod unit_test;

use std::collections::LinkedList;
use std::mem::replace;

// Implementing stacks relatively "from scratch"
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    item: T,
    // Box helps avoid infinity memory allocation
    // in a recursive definition of a struct
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T>{
    pub fn get_item<'a>(&'a self) -> &'a T{
        &self.item
    }
    pub fn get_mut_item<'a>(&'a mut self) -> &'a mut T{
        &mut self.item
    }
    pub fn get_next<'a>(&'a self) -> &'a Option<Box<Node<T>>>{
        &self.next
    }
    pub fn get_mut_next<'a>(&'a mut self) -> &'a mut Option<Box<Node<T>>>{
        &mut self.next
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stack<T> {
    first: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Stack<T> { 
    pub fn new() -> Self {
        // run time complexity O(1)
        Self {
            first: None,
            len: 0
        }
    }
    pub fn init(s: T) -> Self {
        // run time complexity O(1)
        let node = Node {
            item: s,
            next: None,
        };
        Self {
            first: Some(Box::new(node)),
            len: 1,
        }
    }
    pub fn get_first<'a>(&'a self) -> &'a Option<Box<Node<T>>>{
        &self.first
    }
    pub fn get_mut_first<'a>(&'a mut self) -> &'a mut Option<Box<Node<T>>>{
        &mut self.first
    }
    pub fn is_empty(&self) -> bool {
        // run time complexity O(1)
        self.first.is_none()
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Clone> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        // run time complexity O(N) (due to cloning) ?
        // space complexity O(N) (due to cloning) ?
        match self.first {
            Some(ref node) => {
                let item = node.item.clone();
                self.first = node.next.clone();
                self.len -= 1;
                Some(item)
            }
            None => panic!("cannot pop, stack is empty"),
        }
    }

    pub fn push(&mut self, s: T) {
        // run time complexity O(1) (due to cloning) ?
        let oldfirst = self.first.clone();
        let new_node = Node {
            item: s,
            next: oldfirst,
        };
        self.first = Some(Box::new(new_node));
        self.len += 1;
    }
}

// Implementation of stacks using the LinkedList std struct
#[derive(Debug, Clone)]
pub struct LinkedListStack<T> {
    list: LinkedList<T>,
}

impl<T> LinkedListStack<T> {
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    pub fn init(s: T) -> Self {
        let mut res = Self {
            list: LinkedList::new(),
        };
        res.push(s);
        res
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_back()
    }

    pub fn push(&mut self, element: T) {
        self.list.push_back(element)
    }
}

// Implementation using a fixed size vec instead with doubling capacity when full
#[derive(Debug, Clone)]
pub struct VecStack<T> {
    // Number of not None values in vec
    n: usize,
    // Contains the objects
    vec: Vec<Option<T>>,
}

impl<T> VecStack<T> {
    pub fn new() -> Self {
        Self {
            n: 0,
            vec: vec![None],
        }
    }

    pub fn init(capacity: usize) -> Self {
        // run time complexity O(capacity)
        if capacity > 0 {
            let mut vector = Vec::new();
            for _ in 0..capacity {
                vector.push(None);
            }

            Self { n: 0, vec: vector }
        } else {
            panic!("capacity shoul be > 0");
        }
    }

    pub fn is_empty(&self) -> bool {
        // run time complexity O(1)
        self.n == 0
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn push(&mut self, obj: T) {
        // run time complexity O(1)
        if self.n < self.vec.len() {
            // The stack is not full yet
            // This works only if the stack has a capacity > 0
            // If the capacity is zero then panic
            let _ = replace(&mut self.vec[self.n], Some(obj));
            self.n += 1;
            if self.n == self.vec.len() {
                // resize the stack to allow more capacity
                self.resize();
            }
        } else {
            panic!("cannot push, stack is full or has capacity 0");
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        // run time complexity O(1)
        if self.n > 0 && self.n <= self.vec.len() {
            let elt = replace(&mut self.vec[self.n - 1], None);
            self.n -= 1;
            return elt;
        } else {
            panic!("cannot pop, stack is empty");
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