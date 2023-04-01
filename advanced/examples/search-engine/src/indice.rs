use search::collection::{Collection, Document};
use search::preprocess::{character_ngram, clean, inv_index_preproc};
use std::collections::{HashMap, HashSet};
pub struct InvertedIndex {
    index: HashMap<String, Vec<usize>>, // stores the postings
    raw_freq: HashMap<usize, HashMap<String, usize>>, // stores the number of occurrences of tokens in the documents they appear
    sort_postings: bool, // indicates whether or not the postings are sorted
    char_t_index: HashMap<String, HashSet<String>>, // character to term index
    t_char_index: HashMap<String, HashSet<String>>, // term to character index
    include_char_index: bool, // says whether or not to include the (term to) character (to term) index
    ngram: usize,             // the number of characters to consider for the character n-gram index
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            raw_freq: HashMap::new(),
            sort_postings: true,
            char_t_index: HashMap::new(),
            t_char_index: HashMap::new(),
            include_char_index: true,
            ngram: 2,
        }
    }

    pub fn index(&self) -> &HashMap<String, Vec<usize>> {
        &self.index
    }

    pub fn raw_freq(&self) -> &HashMap<usize, HashMap<String, usize>> {
        &self.raw_freq
    }

    pub fn init(sort_postings: bool, include_char_index: bool, ngram: usize) -> Self {
        let mut inv_index = Self::new();
        inv_index.sort_postings = sort_postings;
        inv_index.include_char_index = include_char_index;
        inv_index.ngram = ngram;
        inv_index
    }

    pub fn index_document(&mut self, document: Document, collection: &mut Collection) {
        let doc_id = document.id();
        if !collection.contains(&doc_id) {
            collection.insert(doc_id, document);
        }
        // Character indexing the document
        // if self.include_char_index {
        //     let cleaned_doc_terms = clean(collection.get_document(&doc_id)); // String -> Vec<> or HashSet
        //     for term in cleaned_doc_terms {
        //         let chars = character_ngram(term, self.ngram); // String, usize -> HashSet
        //         for _char in &chars {
        //             if let Some(h) = self.char_t_index.get_mut(_char) {
        //                 (*h).insert(term.to_string());
        //             } else {
        //                 self.char_t_index.insert(_char.to_string(), HashSet::new());
        //             }
        //         }
        //         self.t_char_index.insert(term.to_string(), chars);
        //     }
        // }
        // Invert indexing the document
        let terms = inv_index_preproc(collection.get_document(&doc_id).content()); // Either HashMap<tokens, usize>
        for (token, _) in &terms {
            if let Some(posting) = self.index.get_mut(token) {
                (*posting).push(doc_id); // works if the documents are indexed iteratively with increasing IDs.
            } else {
                self.index.insert(token.to_string(), Vec::new());
            }
        }
        self.raw_freq.insert(doc_id, terms);
    }
}
