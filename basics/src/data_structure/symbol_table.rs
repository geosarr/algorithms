#[cfg(test)]
mod unit_test;
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct BTreeSymbolTable<T,U>{
    tree: BTreeMap<T,U>,
}
impl<T: Ord,U> BTreeSymbolTable<T,U>{
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::<T,U>::new(),
        }
    }
    pub fn init(_key: T, _value: U) -> Self {
        let mut tree = Self::new();
        tree.insert(_key, _value);
        tree 
    }
    pub fn contains(&self, key: &T) -> bool{
        self.tree.get(key).is_some()
    }
    pub fn get(&self, key: &T) -> Option<&U>{
        self.tree.get(key)
    }
    pub fn insert(&mut self, key: T, value: U){
        self.tree.insert(key, value);
    }
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
    pub fn len(&self) -> usize{
        self.tree.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Node<T,U>{
    key: T,
    value: U,
    left: Option<Box<Node<T,U>>>,
    right: Option<Box<Node<T,U>>>,
}
impl<T,U> Node<T,U> {
    pub fn init(_key: T, _value: U) -> Self {
        Self {
            key: _key,
            value: _value,
            left: None,
            right: None,
        }
    }
}
pub struct BinarySearchTree<T,U> {
    root: Option<Box<Node<T,U>>>,
    len: usize,
}
impl<T, U> BinarySearchTree<T,U>{
    pub fn init(key: T, value: U) -> Self {
        Self { 
            root: Some(Box::new(Node::init(key, value))),
            len: 1,
        }
    }
    pub fn len(&self) -> usize{
        self.len
    }
}
impl<T: Eq + Ord + Clone, U: Eq+ Clone> BinarySearchTree<T,U>{
    pub fn cointains(&self, key: &T) -> bool{
        self.get(key).is_some()
    }
    pub fn get(&self, key: &T) -> Option<&U>{
        // gets the value associated to key if key is in 
        // the tree, otherwise returns None,
        // run time complexity on average O(log(N)), O(N) guaranteed (unbalanced tree) 
        let mut node = &self.root;
        while node != &None {
            if key < &node.as_ref().unwrap().key {
                node = &node.as_ref().unwrap().left;
            } else if key > &node.as_ref().unwrap().key {
                node = &node.as_ref().unwrap().right;
            } else { return Some(&node.as_ref().unwrap().value); }
        }
        return None;
    }
    fn put(&mut self, node: &mut Option<Box<Node<T,U>>>, key: T, value: U)
    -> Option<Box<Node<T,U>>>
    {
        match node {
            None => Some(Box::new(Node::init(key, value))),
            Some(ref mut nod) => {
                if key < nod.key {
                    nod.left = self.put(&mut nod.left, key, value);
                } else if key > nod.key{
                    nod.right = self.put(&mut nod.right, key, value);
                } else { nod.value = value; }
                node.clone()
            }
        }
    }
    pub fn insert(&mut self, key: T, value: U)
    {
        // self.root = self.put(&mut self.root, key, value);
        self.len += 1;
    }
}




// ###########################################
#[derive(Default, Clone, Debug)]
pub struct OrderedVecSymbolTable<T, U> {
    // collection of key-value pair (no duplicate keys)
    vec: Vec<Pair<T,Option<U>>>
}
impl<T, U> OrderedVecSymbolTable<T, U> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new()
        }
    }
    pub fn init(key: T, value: U) -> Self {
        let mut symbol_table = Self::new();
        symbol_table.vec.push(Pair::init(key, Some(value)));
        symbol_table
    }
}
impl<T: Eq + Clone + Ord, U: Eq + Clone> OrderedVecSymbolTable<T, U> {
    pub fn contains(&self, key: T) -> bool {
        // run time complexity O(log(N))
        self.get(key) != None
    }

    pub fn get(&self, key: T) -> Option<&U> {
        // run time complexity O(log(N))
        if let Ok(index) = self.vec.binary_search(&Pair::init(key, None)){
            if let Some(ref option) = self.vec[index].get_second(){
                return option.as_ref();
            } else {panic!("problem in binary search or get_second()")}
        } else {None}
    }

    fn put(&mut self, key: T, value: Option<U>) {
        // run time complexity O(N) due to insertion
        let index = self.vec.binary_search(&Pair::init(key.clone(), None));
        match index {
            // key is found
            Ok(ind) => self.vec.insert(ind, Pair::init(key, value)),
            Err(index) => { 
                // index where to insert key to keep self.vec sorted 
                if index < self.vec.len(){
                    self.vec.insert(index, Pair::init(key, value));
                } else { self.vec.push(Pair::init(key, value)) }
            }
        }
    } 

    pub fn insert(&mut self, key: T, value: U) {
        // run time complexity O(N)
        self.put(key, Some(value))
    }

    pub fn delete(&mut self, key: T) -> Option<T>
    // run time complexity O(N)
    {
        self.put(key.clone(), None); // lazy implementation ?
        Some(key)
    }
    
    pub fn len(&self) -> usize{
        // number of keys O(1)
        self.vec.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn min(&self) -> Option<&T>{
        // smallest key O(1)
        if self.is_empty(){
            None
        }else {
            self.vec[0].get_first()
        }
    }
    
    pub fn max(&self) -> Option<&T>{
        // largest key O(1)
        if self.vec.is_empty(){
            None
        } else {
            self.vec[self.vec.len()-1].get_first()
        }
    }

    pub fn floor(&self, key: T) -> Option<&T>{
        // largest key smaller or equal to key O(log(N))
        if self.is_empty() {None}
        else {
            let index = self.vec.binary_search(&Pair::init(key.clone(), None));
            match index {
                Ok(ind) => self.vec[ind].get_first(),
                Err(ind) => {
                    if ind > 0 {
                        self.vec[ind-1].get_first()
                    } else {
                        self.vec[ind].get_first()
                    }
                }
            }
        }
    }

    pub fn ceil(&self, key: T) -> Option<&T>{
        // smallest key larger or equal to key , O(log(N))
        if self.is_empty() {None}
        else {
            let index = self.vec.binary_search(&Pair::init(key, None));
            match index {
                Ok(ind) => self.vec[ind].get_first(),
                Err(ind) => {
                    if ind < self.vec.len()-1 {
                        self.vec[ind+1].get_first()
                    } else {
                        self.vec[ind].get_first()
                    }
                }
            }
        }
    }
}
#[derive(Default, Clone, Debug)]
struct Pair<T,U>{
    tuple: (T,U)
}
impl<T,U> Pair<T,U>{
    pub fn init(key: T, value: U) -> Self{
        Self{tuple:(key, value)}
    }
}
impl<T: Clone, U: Clone> Pair<T,U>{
    pub fn get_first(&self) -> Option<&T> {
        Some(&self.tuple.0)
    }

    pub fn get_second(&self) -> Option<&U> {
        Some(&self.tuple.1)
    }
}
impl<T: Ord, U> Ord for Pair<T,U>{
    fn cmp(&self, other: &Self) -> Ordering {
        self.tuple.0.cmp(&other.tuple.0)
    }
}
impl<T: Ord, U> PartialOrd for Pair<T,U>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.tuple.0.cmp(&other.tuple.0))
    }
}
impl<T: Ord,U> Eq for Pair<T,U> {}
impl<T: Ord, U> PartialEq for Pair<T,U>{
    fn eq(&self, other: &Self) -> bool {
        self.tuple.0 == other.tuple.0
    }
}



// ###############################################
#[derive(Default, Clone, Debug)]
pub struct UnorderedVecSymbolTable<T, U> {
    // collection of key-value pair (no duplicate keys)
    vec: Vec<(T, Option<U>)>
}
impl<T, U> UnorderedVecSymbolTable<T, U> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new()
        }
    }
    pub fn init(key: T, value: U) -> Self{
        let mut symbol_table = Self::new();
        symbol_table.vec.push((key, Some(value)));
        symbol_table
    }
}
impl<T: Eq + Clone, U: Eq + Clone> UnorderedVecSymbolTable<T, U> {
    pub fn contains(&self, key: T) -> bool {
        // run time complexity O(N)
        self.get(key) != None
        // self.list.iter().any(|e| e.0 == key)
    }

    pub fn get(&self, key: T) -> Option<U> {
        // run time complexity O(N)
        for k in 0..self.vec.len(){
            if self.vec[k].0 == key {
                return self.vec[k].1.clone();
            }
        }
        None
    }

    fn put(&mut self, key: T, value: Option<U>)  {
        // run time complexity O(N)
        let mut k = 0;
        while k < self.vec.len(){
            if self.vec[k].0 == key {
                self.vec[k].1 = value.clone();
                break;
            }
            k +=1; 
        }
        // check wether or not key is in self.vec
        // <=> comparing to the (k-1)^th element in self.vec 
        if self.vec[k-1].0 != key {
            // key is not in self.vec
            self.vec.push((key, value));
        } 
    } 

    pub fn insert(&mut self, key: T, value: U) {
        // run time complexity O(N)
        self.put(key, Some(value))
    }

    pub fn delete(&mut self, key: T) -> Option<T>
    // run time complexity O(N)
    {
        self.put(key.clone(), None); // lazy implementation ?
        Some(key)
    }
}