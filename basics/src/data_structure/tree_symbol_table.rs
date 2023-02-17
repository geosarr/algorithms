#[cfg(test)]
mod unit_test;
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct BTreeSymbolTable<T, U> {
    tree: BTreeMap<T, U>,
}
impl<T, U> BTreeSymbolTable<T, U> {
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
    pub fn len(&self) -> usize {
        self.tree.len()
    }
}
impl<T: Ord, U> BTreeSymbolTable<T, U> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::<T, U>::new(),
        }
    }
    pub fn init(_key: T, _value: U) -> Self {
        let mut tree = Self::new();
        tree.insert(_key, _value);
        tree
    }
    pub fn contains(&self, key: &T) -> bool {
        self.tree.get(key).is_some()
    }
    pub fn get(&self, key: &T) -> Option<&U> {
        self.tree.get(key)
    }
    pub fn insert(&mut self, key: T, value: U) {
        self.tree.insert(key, value);
    }
}
impl<T: Ord, U: Ord> BTreeSymbolTable<T, U> {
    pub fn strict_floor(&self, key: &T) -> Option<&T> {
        // the largest key in the tree map, strictly inferior to key
        let res = self.tree.range(..key).max();
        if let Some(item) = res {
            Some(item.0)
        } else {
            None
        }
    }
    pub fn ceil(&self, key: &T) -> Option<&T> {
        // the smallest key in the tree map larger ot equal to key
        let res = self.tree.range(key..).min();
        if let Some(item) = res {
            Some(item.0)
        } else {
            None
        }
    }
    pub fn range_search(&self, low: &T, high: &T) -> Vec<&T> {
        // returns the keys between low (included) and high (excluded)
        self.tree
            .range(low..high)
            .into_iter()
            .map(|(k, _)| k)
            .collect::<Vec<&T>>()
    }
    pub fn range_count(&self, low: &T, high: &T) -> usize {
        // counts the keys between low (included) and high (excluded)
        self.range_search(low, high).len()
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Node<T, U> {
    key: T,
    value: U,
    left: Option<Box<Node<T, U>>>,
    right: Option<Box<Node<T, U>>>,
}
impl<T, U> Node<T, U> {
    pub fn init(_key: T, _value: U) -> Self {
        Self {
            key: _key,
            value: _value,
            left: None,
            right: None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BinarySearchTree<T, U> {
    root: Option<Box<Node<T, U>>>,
    len: usize,
}
impl<T, U> Default for BinarySearchTree<T, U> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T, U> BinarySearchTree<T, U> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }
    pub fn init(key: T, value: U) -> Self {
        Self {
            root: Some(Box::new(Node::init(key, value))),
            len: 1,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<T: Eq + Ord, U: Eq> BinarySearchTree<T, U> {
    pub fn contains(&self, key: &T) -> bool {
        self.get(key).is_some()
    }
    pub fn get(&self, key: &T) -> Option<&U> {
        // gets the value associated to key if key is in
        // the tree, otherwise returns None,
        // run time complexity on average O(log(N)), O(N) guaranteed (unbalanced tree)
        let mut node = &self.root;
        while node.is_some() {
            let temp_node = node.as_ref().unwrap();
            match key.cmp(&temp_node.key) {
                Ordering::Less => node = &temp_node.left,
                Ordering::Greater => node = &temp_node.right,
                Ordering::Equal => return Some(&temp_node.value),
            }
        }
        None
    }
}
impl<T: Ord, U> BinarySearchTree<T, U> {
    fn put(node: &mut Option<Box<Node<T, U>>>, key: T, value: U) {
        match node {
            None => *node = Some(Box::new(Node::init(key, value))),
            Some(ref mut nod) => {
                match key.cmp(&nod.key) {
                    Ordering::Less => Self::put(&mut nod.left, key, value),
                    Ordering::Greater => Self::put(&mut nod.right, key, value),
                    Ordering::Equal => nod.value = value,
                }
                // if key < nod.key {
                //     Self::put(&mut nod.left, key, value);
                // } else if key > nod.key {
                //     Self::put(&mut nod.right, key, value);
                // } else {
                //     nod.value = value;
                // }
            }
        }
    }
    pub fn insert(&mut self, key: T, value: U) {
        Self::put(&mut self.root, key, value);
        self.len += 1;
    }
}
impl<T: Eq + Ord, U: Ord> BinarySearchTree<T, U> {
    pub fn min(&self) -> Option<&T> {
        // finds the minimum key
        let mut node = &self.root;
        let mut result = None;
        while node.is_some() {
            // go to the left as long as you do not encounter
            // a None Node
            let temp_node = node.as_ref().unwrap();
            result = Some(&temp_node.key);
            node = &temp_node.left;
        }
        result
    }
    pub fn max(&self) -> Option<&T> {
        // finds the maximum key
        let mut node = &self.root;
        let mut result = None;
        while node.is_some() {
            // go to the right as long as you do not encounter
            // a None Node
            let temp_node = node.as_ref().unwrap();
            result = Some(&temp_node.key);
            node = &temp_node.right;
        }
        result
    }
    fn recursive_floor<'a>(
        node: &'a Option<Box<Node<T, U>>>,
        key: &T,
    ) -> &'a Option<Box<Node<T, U>>> {
        if node.is_none() {
            return &None;
        }
        let current_node = node.as_ref().unwrap();
        if key == &current_node.key {
            return node;
        }
        if key < &current_node.key {
            return Self::recursive_floor(&current_node.left, key);
        }
        let temp_node = Self::recursive_floor(&current_node.right, key);
        if temp_node.is_some() {
            temp_node
        } else {
            node
        }
    }
    pub fn floor(&self, key: &T) -> Option<&T> {
        // the largest key in the tree smaller or equal to key
        // run time complexity O(log(N)) on average, O(N) (guaranteed)
        let node = Self::recursive_floor(&self.root, key);
        if node.is_none() {
            return None;
        }
        return Some(&node.as_ref().unwrap().key);
    }
}

// ###########################################
#[derive(Default, Clone, Debug)]
pub struct OrderedVecSymbolTable<T, U> {
    // collection of key-value pair (no duplicate keys)
    vec: Vec<Pair<T, Option<U>>>,
}
impl<T, U> OrderedVecSymbolTable<T, U> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
    pub fn init(key: T, value: U) -> Self {
        let mut symbol_table = Self::new();
        symbol_table.vec.push(Pair::init(key, Some(value)));
        symbol_table
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    pub fn min(&self) -> Option<&T> {
        // smallest key O(1)
        if self.is_empty() {
            None
        } else {
            Some(self.vec[0].get_first())
        }
    }
    pub fn max(&self) -> Option<&T> {
        // largest key O(1)
        if self.vec.is_empty() {
            None
        } else {
            Some(self.vec[self.vec.len() - 1].get_first())
        }
    }
}
impl<T: Ord + Clone, U: Eq> OrderedVecSymbolTable<T, U> {
    pub fn contains(&self, key: &T) -> bool {
        // run time complexity O(log(N))
        self.get(key).is_some()
    }

    pub fn get(&self, key: &T) -> Option<&U> {
        // run time complexity O(log(N))
        if let Ok(index) = self.vec.binary_search(&Pair::init(key.clone(), None)) {
            return self.vec[index].get_second().as_ref();
        } else {
            None
        }
    }
    pub fn floor(&self, key: &T) -> Option<&T> {
        // largest key smaller or equal to key O(log(N))
        if self.is_empty() {
            None
        } else {
            let index = self.vec.binary_search(&Pair::init(key.clone(), None));
            match index {
                Ok(ind) => Some(self.vec[ind].get_first()),
                Err(ind) => {
                    if ind > 0 {
                        Some(self.vec[ind - 1].get_first())
                    } else {
                        // all keys in the table are > keys
                        None
                    }
                }
            }
        }
    }
    pub fn ceil(&self, key: &T) -> Option<&T> {
        // smallest key larger or equal to key , O(log(N))
        if self.is_empty() {
            None
        } else {
            let index = self.vec.binary_search(&Pair::init(key.clone(), None));
            match index {
                Ok(ind) => Some(self.vec[ind].get_first()),
                Err(ind) => {
                    if ind < self.vec.len() - 1 {
                        Some(self.vec[ind + 1].get_first())
                    } else {
                        // all keys in the table are < key
                        None
                    }
                }
            }
        }
    }
}
impl<T: Ord + Clone, U: Eq + Clone> OrderedVecSymbolTable<T, U> {
    fn put(&mut self, key: T, value: Option<U>) -> Option<U> {
        // run time complexity O(N) due to insertion
        let index = self.vec.binary_search(&Pair::init(key.clone(), None));
        match index {
            // key is found
            Ok(ind) => {
                let temp_val = self.vec[ind].get_second().as_ref().cloned();
                let mut_val = self.vec[ind].set_second();
                *mut_val = value;
                // self.vec[ind] = Pair::init(key, value);
                temp_val
            }
            Err(ind) => {
                // index where to insert key to keep self.vec sorted
                if ind < self.vec.len() {
                    self.vec.insert(ind, Pair::init(key, value));
                } else {
                    self.vec.push(Pair::init(key, value))
                }
                None
            }
        }
    }
    pub fn insert(&mut self, key: T, value: U) {
        // run time complexity O(N)
        self.put(key, Some(value));
    }

    pub fn delete(&mut self, key: &T) -> Option<U>
// run time complexity O(N)
    {
        self.put(key.clone(), None) // lazy implementation
    }
}

#[derive(Default, Clone, Debug)]
struct Pair<T, U> {
    tuple: (T, U),
}
impl<T, U> Pair<T, U> {
    pub fn init(key: T, value: U) -> Self {
        Self {
            tuple: (key, value),
        }
    }
    pub fn get_first(&self) -> &T {
        &self.tuple.0
    }

    pub fn get_second(&self) -> &U {
        &self.tuple.1
    }
    pub fn set_first(&mut self) -> &mut U {
        &mut self.tuple.1
    }
    pub fn set_second(&mut self) -> &mut U {
        &mut self.tuple.1
    }
}
impl<T: Ord, U> Ord for Pair<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tuple.0.cmp(&other.tuple.0)
    }
}
impl<T: Ord, U> PartialOrd for Pair<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.tuple.0.cmp(&other.tuple.0))
    }
}
impl<T: Ord, U> Eq for Pair<T, U> {}
impl<T: Ord, U> PartialEq for Pair<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.tuple.0 == other.tuple.0
    }
}

// ###############################################
#[derive(Default, Clone, Debug)]
pub struct UnorderedVecSymbolTable<T, U> {
    // collection of key-value pair (no duplicate keys)
    vec: Vec<(T, Option<U>)>,
}
impl<T, U> UnorderedVecSymbolTable<T, U> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
    pub fn init(key: T, value: U) -> Self {
        let mut symbol_table = Self::new();
        symbol_table.vec.push((key, Some(value)));
        symbol_table
    }
}
impl<T: Eq, U: Eq> UnorderedVecSymbolTable<T, U> {
    pub fn contains(&self, key: &T) -> bool {
        // run time complexity O(N)
        self.get(key).is_some()
        // self.list.iter().any(|e| e.0 == key)
    }
}
impl<T: Eq, U> UnorderedVecSymbolTable<T, U> {
    pub fn get(&self, key: &T) -> Option<&U> {
        // run time complexity O(N)
        for k in 0..self.vec.len() {
            if &self.vec[k].0 == key {
                return self.vec[k].1.as_ref();
            }
        }
        None
    }
}
impl<T: Eq, U: Clone> UnorderedVecSymbolTable<T, U> {
    fn put(&mut self, key: T, value: Option<U>) -> Option<U> {
        // run time complexity O(N)
        let mut k = 0;
        let mut val = None;
        while k < self.vec.len() {
            if self.vec[k].0 == key {
                val = self.vec[k].1.clone();
                self.vec[k].1 = value.clone();
                break;
            }
            k += 1;
        }
        // check wether or not key is in self.vec
        // <=> comparing to the (k-1)^th element in self.vec
        if self.vec[k - 1].0 != key {
            // key is not in self.vec
            self.vec.push((key, value));
        }
        val
    }

    pub fn insert(&mut self, key: T, value: U) {
        // run time complexity O(N)
        let _ = self.put(key, Some(value));
    }
}
impl<T: Eq + Clone, U: Clone> UnorderedVecSymbolTable<T, U> {
    pub fn delete(&mut self, key: &T) -> Option<U>
// run time complexity O(N)
    {
        self.put(key.clone(), None) // lazy implementation
    }
}
