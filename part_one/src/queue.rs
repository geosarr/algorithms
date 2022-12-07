#[cfg(test)]
mod unit_test;
// use std::mem::replace;


#[derive(Debug, Clone)]
struct Node {
    item: String,
    // Box helps avoid infinity memory allocation
    // in a recursive definition of a struct
    next: Option<Box<Node>>,
}

#[derive(Default, Clone, Debug)]
pub struct LinkedListQueue {
    first: Option<Box<Node>>,
    last: Option<Box<Node>>,
}


impl LinkedListQueue {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(s: String) -> Self {
        let node = Node {item: s, next: None};
        Self {
            first: Some(Box::new(node.clone())),
            last: Some(Box::new(node.clone())),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.first {
            Some(_) => false,
            None => true
        }
    }

    pub fn dequeue(&mut self) -> Option<String> {
        match self.first{
            Some(ref node) => {
                let item = node.item.clone();
                self.first = node.next.clone();
                if let None = self.first {
                    // When the first element in the queue is None
                    // there is no element in the queue, so last
                    // should be None as well
                    self.last = None;
                }
                Some(item)
            },
            None => panic!("cannot dequeue, queue is empty"), 
        }
    }

    pub fn enqueue(&mut self, s: String) {
        match self.first {
            None => {
                // The queue is empty last = None = first
                // So they should be both equal to node defined as follows
                let new_node =  Node {item: s, next: None};
                self.first = Some(Box::new(new_node.clone()));
                self.last = Some(Box::new(new_node.clone()));
            },
            Some(ref mut node) => {
                if let None = node.next {
                    let new_node = Node {item: s, next: self.last.clone()};
                    node.next = Some(Box::new(new_node));
                } else {
                    let mut oldlast = self.last.clone();
                    let newlast = Node {item: s, next: None};
                    self.last = Some(Box::new(newlast.clone()));
                    if let Some(ref mut node) = oldlast {
                        node.next = self.last.clone();
                    }
                    self.first = oldlast;                }
              
            }
        } 
        // else {
        //     if let Some(ref node) = self.first.next {
                
        //     }
        //     let mut oldlast = self.last.clone();
        //     let newlast = Node {item: s, next: None};
        //     self.last = Some(Box::new(newlast.clone()));
        //     if let Some(ref mut node) = oldlast {
        //         node.next = self.last.clone();
        // }}
        
    }
}