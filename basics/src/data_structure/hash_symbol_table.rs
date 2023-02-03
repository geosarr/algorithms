#[cfg(test)]
mod unit_test;
use crate::data_structure::stack::Stack;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};




#[derive(Debug)]
pub struct SeparateChainingSymbolTable<T,U>{
    // number of chains in the hash table
    chains: usize, 
    // list of chains
    vec: Vec<Stack<(T,U)>>,
    // number of elements in the table
    len: usize, 

    // Remarks:
    // - since collision is almost unavoidable (birthday "paradox"), the idea is to put keys that have the same hash
    // in the same linked list (Stack here) and for searching it suffices to go through the stack
    // to find the key and its value
    // - to ensure constant time for search (one should update the length of the list of chains (chains field))
    //   so that N/chains ~ constant (where N is the number of keys in the table) with resizing strategies (e.g doubling and halving)
}
impl<T,U> SeparateChainingSymbolTable<T,U>{
    pub fn new() -> Self {
        Self {
            chains: 97, 
            vec: Vec::new(),
            len: 0
        }
    }
}
impl<T: Clone, U: Clone> SeparateChainingSymbolTable<T,U>{
    fn double(&mut self){
        // doubles the number of chains
        self.vec.resize(2*self.chains, Stack::new());
        self.chains *= 2;
    }
    fn halve(&mut self){
        // reduce the size of the chain to free space
        // TODO 
    }
}
impl<T: Hash,U> SeparateChainingSymbolTable<T,U>{
    fn calculate_hash(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
    fn hash(&self, key: &T) -> usize{
        // makes the default hash within range [0, chains-1]
        let h = Self::calculate_hash(&key) as usize;
        h.rem_euclid(self.chains) 
        // h % self.chains
    }
}
impl<T: Hash + Clone, U: Clone> SeparateChainingSymbolTable<T,U>{
    pub fn init(_chains: usize, key: T, value: U) -> Self{
        assert!(_chains > 0);
        let mut symbol_table = Self {
            chains: _chains, 
            vec: vec![Stack::new();_chains],
            len: 0,
        };
        let index = symbol_table.hash(&key); 
        symbol_table.vec[index] = Stack::init((key, value));
        symbol_table.len = 1;
        symbol_table
    }
}
impl<T: Hash + Clone + Eq, U: Clone + Eq> SeparateChainingSymbolTable<T,U>{
        pub fn insert(&mut self, key: T, value: U){
        // works if chains is small enough so that all index in u64 
        // can be converted to usize (see self.hash() function)
        let index = self.hash(&key); 
        let stack_len = self.vec[index].len();
        let mut stack = self.vec[index].get_mut_first();
        let mut c = 0; 
        while c < stack_len {
            let temp_stack = stack.as_mut().unwrap(); 
            if key == temp_stack.get_item().0 {
                // replace its value
                (*temp_stack.get_mut_item()).1 = value.clone();
                break;
            }
            stack = temp_stack.get_mut_next(); 
            c += 1;
        }
        if c >= stack_len {self.vec[index].push((key, value))};
        self.len += 1;
        }
}
impl<T: Eq + Hash, U: Eq> SeparateChainingSymbolTable<T,U>{
    pub fn contains(&self, key: &T) -> bool {
        self.get(key).is_some()
    }
    pub fn get(&self, key: &T) -> Option<&U>{
        // run time complexity on average O(N/chains)
        // because on average the stacks are equally likely to
        // have the same number of keys due to the coupon collector and load balancing properties
        let index = self.hash(&key);
        let mut stack = self.vec[index].get_first();
        while stack != &None{
            let temp_stack = stack.as_ref().unwrap(); 
            if key == &temp_stack.get_item().0 {
                return Some(&temp_stack.get_item().1);
            }
            stack = &temp_stack.get_next(); 
        }
        None
    }
}