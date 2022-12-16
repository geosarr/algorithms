#[cfg(test)]
mod tests {
    use super::super::*;
    #[test]
    fn test_evaluation() {
        let mut e = Evaluation::new();
        let res = e.run("( ( 1 * ( 2 + 3 ) ) + ( 4 * ( 5 + 6 ) ) )".to_string());
        assert_eq!(res, 49);
    }
}
