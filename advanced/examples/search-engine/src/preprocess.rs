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
        n_grams.insert(_word[i..n - size + 1].to_string());
    }
    n_grams
}

pub fn clean(doc: &Document) -> HashSet<&str> {
    return doc.content().split(" ").collect();
}

pub fn inv_index_preproc(content: &String) -> HashMap<String, usize> {
    content
        .split(" ")
        .into_iter()
        .map(|e| (e.to_string(), 1))
        .collect()
}
