#[cfg(test)]
mod unit_test;

use std::collections::LinkedList;

#[derive(Default, Clone, Debug)]
pub struct LinkedListQueue<T> {
    list: LinkedList<T>,
}

impl<T> LinkedListQueue<T> {
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    pub fn init(s: T) -> Self {
        let mut res = Self {
            list: LinkedList::new(),
        };
        res.enqueue(s);
        res
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn enqueue(&mut self, element: T) {
        self.list.push_back(element)
    }
}
