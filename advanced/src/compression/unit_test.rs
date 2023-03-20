#[cfg(test)]
mod tests {
    use super::super::*;
    #[test]
    fn test_compress() {
        let bits = BitVec::from_bitslice(bits![0, 0, 1, 1]);
        let run_length = RunLength::init(bits);
        let compressed_bits = run_length.compress();
        assert_eq!(vec![2, 2], compressed_bits);

        let bits = BitVec::from_bitslice(bits![1, 0, 1, 1]);
        let run_length = RunLength::init(bits);
        let compressed_bits = run_length.compress();
        assert_eq!(vec![0, 1, 1, 2], compressed_bits);

        let bits = BitVec::from_bitslice(bits![1, 0, 1, 0]);
        let run_length = RunLength::init(bits);
        let compressed_bits = run_length.compress();
        assert_eq!(vec![0, 1, 1, 1, 1], compressed_bits);
    }
}
