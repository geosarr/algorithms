#[cfg(test)]
mod unit_test;

use std::collections::LinkedList;


#[derive(Default, Clone, Debug)]
pub struct LinkedListDeque<T> {
    list: LinkedList<T>,
}

impl<T> LinkedListDeque<T>{
    pub fn new() -> Self {
        Self{
            list: LinkedList::new(),
        }
    }
    
    pub fn init(s: T) -> Self {
        let mut res = Self{
            list: LinkedList::new(),
        };
        res.add_first(s);
        res
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }

    pub fn add_first(&mut self, elt: T) {
        self.list.push_front(elt)
    }

    pub fn add_last(&mut self, elt: T) {
        self.list.push_back(elt)
    }

    pub fn remove_first(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn remove_last(&mut self) -> Option<T> {
        self.list.pop_back()
    }
    
}