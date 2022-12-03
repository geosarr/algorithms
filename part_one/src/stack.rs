// use std::collections::LinkedList;

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
pub struct Stack {
    first: Option<Box<Node>>,
}


impl Stack {
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