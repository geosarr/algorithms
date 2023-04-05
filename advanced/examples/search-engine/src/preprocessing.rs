use crate::collection::Document;
use crate::constant::PUNCTUATION;
use std::collections::{HashMap, HashSet};
pub fn character_ngram(word: &str, size: usize) -> HashSet<String> {
    let _word = word.trim();
    // println!("word : {_word}\nlen = {}", _word.len());
    let len_chars: Vec<_> = _word.chars().map(|ch| ch.len_utf8()).collect();
    let n = len_chars.len();
    if n <= size {
        return HashSet::from([word.to_string()]);
    }
    let mut ngrams = HashSet::new();
    let mut pos = 0;
    let mut chars = _word.chars();
    let mut i = 0;
    // println!("len_chars = {}", len_chars.len());
    for i in 0..n - size + 1 {
        let mut len = 0;
        for j in i..i + size {
            len += len_chars[j]
        }
        let ngram = (&_word[pos..pos + len]).to_string();
        // println!("{:?}", ngram);
        ngrams.insert(ngram);
        pos += len - len_chars[i + size - 1];
    }
    ngrams
}

// pub fn clean(doc: &Document) -> HashSet<&str> {
//     return doc.content().split(" ").collect();
// }
pub fn is_not_punct(character: char) -> bool {
    !PUNCTUATION.contains(character)
}

pub fn preprocess(doc: &Document) -> HashMap<String, usize> {
    let mut content = doc.content().to_lowercase();
    content.retain(is_not_punct);
    let content = content.split_whitespace().collect();
    let mut counter = Counter::new();
    counter.count(content)
    // counter.into_hashmap()
}

struct Counter {
    // data: HashMap<String, usize>;
}

impl Counter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn count(&mut self, content: Vec<&str>) -> HashMap<String, usize> {
        let mut counter = HashMap::with_capacity(content.len());
        for word in content {
            if let Some(count) = counter.get_mut(word) {
                *count += 1;
            } else {
                counter.insert(word.to_string(), 1);
            }
        }
        counter
    }
    // fn insert(&mut self, word: String, count: usize) {
    //     self.data.insert(word, count);
    // }
    // fn get_mut(&mut self, word: &str) -> Option<&mut usize> {
    //     self.data.get_mut(word)
    // }
    // pub fn into_hashmap(self) -> HashMap<String, usize> {
    //     self.data
    // }
}
