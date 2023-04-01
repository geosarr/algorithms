#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_character_ngram() {
        let words = "rustyboy".to_owned();
        let char_gram = character_ngram(words, 4);
        let expected_result = HashSet::from(["rust", "usty", "styb", "tybo", "yboy"]);
        assert_eq!(char_gram, expected_result);
    }
}
