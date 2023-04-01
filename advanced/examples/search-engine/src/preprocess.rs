use crate::collection::Document;
use std::collections::{HashMap, HashSet};
pub fn character_ngram(word: &str, size: usize) -> HashSet<String> {
    let _word = word.trim();
    println!("{_word}");
    let n = _word.len();
    if n <= size {
        return HashSet::from([_word.to_string()]);
    }
    let mut n_grams = HashSet::new();
    for i in 0..n - size + 1 {
        println!("{:?}", &_word[i..i + size]);
        n_grams.insert(_word[i..i + size].to_string());
    }
    println!("\n");
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
