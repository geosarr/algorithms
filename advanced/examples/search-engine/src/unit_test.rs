#[cfg(test)]
mod tests {
    use search::preprocess::character_ngram;
    use std::collections::HashSet;

    #[test]
    fn test_character_ngram() {
        let words = "rustyboy".to_owned();
        let char_gram = character_ngram(&words, 4);
        let expected_result = HashSet::from(["rust", "usty", "styb", "tybo", "yboy"]);
        let expected_result = expected_result
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();
        assert_eq!(char_gram, expected_result);
    }
}
