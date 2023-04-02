use crate::collection::Document;
use std::collections::{HashMap, HashSet};
pub fn character_ngram(word: &str, size: usize) -> HashSet<String> {
    let _word = word.trim();
    let n = _word.len();
    if n <= size {
        return HashSet::from([_word.to_string()]);
    }
    let mut n_grams = HashSet::new();
    for i in 0..n - size + 1 {
        let mut chars = _word[i..].chars();
        let mut ngram = String::new();
        for _ in i..i + size {
            let _char = chars.next();
            ngram.push(_char.expect("Failed to retrieve char."));
        }
        n_grams.insert(ngram);
    }
    n_grams
}

pub fn clean(doc: &Document) -> HashSet<&str> {
    return doc.content().split(" ").collect();
}

pub fn preprocess(doc: &Document) -> HashMap<String, usize> {
    let content: Vec<_> = doc.content().split(" ").collect();
    let mut counter = Counter::new();
    counter.count(content);
    counter.into_hashmap()
}

struct Counter {
    data: HashMap<String, usize>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn count(&mut self, content: Vec<&str>) {
        for word in content {
            if let Some(count) = self.get_mut(word) {
                *count += 1;
            } else {
                self.data.insert(word.to_string(), 1);
            }
        }
    }
    fn insert(&mut self, word: String, count: usize) {
        self.data.insert(word, count);
    }
    fn get_mut(&mut self, word: &str) -> Option<&mut usize> {
        self.data.get_mut(word)
    }
    pub fn into_hashmap(self) -> HashMap<String, usize> {
        self.data
    }
}