#[cfg(test)]
mod unit_test;

use std::collections::LinkedList;
use std::mem::replace;


#[derive(Debug, Clone)]
struct Node {
    item: String,
    // Box helps avoid infinity memory allocation
    // in a recursive definition of a struct
    next: Option<Box<Node>>,
}

#[derive(Default, Clone)]
pub struct LinkedListStackOfString {
    first: Option<Box<Node>>,
    len: usize,
}


impl LinkedListStackOfString {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(s: String) -> Self {
        let node = Node {item: s, next: None};
        Self {
            first: Some(Box::new(node)),
            len: 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    pub fn pop(&mut self) -> Option<String> {
        match self.first{
            Some(ref node) => {
                let item = node.item.clone();
                self.first = node.next.clone();
                self.len -= 1;
                Some(item)
            },
            None => panic!("cannot pop, stack is empty"),
        }
    }

    pub fn push(&mut self, s: String) {
        let oldfirst = self.first.clone();
        let new_node = Node {item: s, next: oldfirst};
        self.first = Some(Box::new(new_node));
        self.len += 1;
    }
}


#[derive(Default, Clone, Debug)]
pub struct LinkedListStack<T> {
    list: LinkedList<T>,
}

impl<T> LinkedListStack<T>{
    pub fn new() -> Self {
        Self{
            list: LinkedList::new(),
        }
    }
    
    pub fn init(s: T) -> Self {
        let mut res = Self{
            list: LinkedList::new(),
        };
        res.push(s);
        res
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_back()
    }

    pub fn push(&mut self, element: T) {
        self.list.push_back(element)
    }
    
}

#[derive(Debug, Clone)]
pub struct VecStack<T>{
    // Number of not None values in vec
    n: usize,
    // Contains the objects
    pub vec: Vec<Option<T>>, 
}

impl<T> VecStack<T> {
    pub fn new() -> Self {
        Self {
            n: 0,
            vec: vec![None],
        }
    }

    pub fn init(capacity: usize) -> Self {
        if capacity > 0 {
            let mut vector = Vec::new();
            for _ in 0..capacity {
                vector.push(None);
            }

            Self {
                n: 0,
                vec: vector,
            }
        } else {panic!("capacity shoul be > 0");}
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn push(&mut self, obj: T) {
        // let n = self.n;
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
        } else {panic!("cannot push, stack is full or has capacity 0");}
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.n > 0 && self.n <= self.vec.len() {
            let elt = replace(&mut self.vec[self.n - 1], None);
            self.n -= 1;
            return elt;
        } else {panic!("cannot pop, stack is empty");}
    }

    fn resize(&mut self) {
        // doubling the size of the stack when it is full
        let mut vector = Vec::new();
        for _ in 0..self.vec.len() {
            vector.push(None);
        } 
        self.vec.append(&mut vector);
    }
}
