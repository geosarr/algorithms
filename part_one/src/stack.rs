// use std::collections::LinkedList;
use std::mem::replace;

// fn unbox(value: Box) -> T {
//     *value
// }

#[derive(Clone)]
struct Node {
    item: String,
    // Box helps avoid infinity memory allocation
    // in a recursive definition of a struct
    next: Option<Box<Node>>,
}

#[derive(Default, Clone)]
pub struct LinkedListStack {
    first: Option<Box<Node>>,
}


impl LinkedListStack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(s: String) -> Self {
        let node = Node {item: s, next: None};
        Self {
            first: Some(Box::new(node)),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.first {
            Some(_) => false,
            None => true
        }
    }

    pub fn pop(&mut self) -> String {
        match self.first{
            Some(ref node) => {
                let item = node.item.clone();
                self.first = node.next.clone();
                item
            },
            None => panic!("cannot pop, stack is empty"),
        }
    }

    pub fn push(&mut self, s: String) {
        let oldfirst = self.first.clone();
        let new_node = Node {item: s, next: oldfirst};
        self.first = Some(Box::new(new_node));
    }
}


pub struct VecStack<T>{
    // Number of not None values in vec
    n: isize,
    // Contains the objects
    pub vec: Vec<Option<T>>, 
}

impl<T> VecStack<T> {
    pub fn new() -> Self {
        Self {
            n: 0,
            vec: Vec::new()
        }
    }

    pub fn from(capacity: usize) -> Self {
        let mut vector = Vec::new();
        for _ in 0..capacity {
            vector.push(None);
        }

        Self {
            n: 0,
            vec: vector,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn push(&mut self, obj: T) {
        let n = self.n;
        if n < self.vec.len() {
            // The stack is not full yet
            if n > 0 {
            let _ = replace(&mut self.vec[self.n - 1], Some(obj));}
            self.n += 1;
        } else {
            panic!("cannot push, stack is full");
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.n > 0 && self.n < self.vec.len() {
            let elt = replace(&mut self.vec[self.n - 1], None);
            self.n -= 1;
            elt
        } else {panic!("cannot pop, stack has no object.");}
    }
}