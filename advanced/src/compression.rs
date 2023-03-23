#[cfg(test)]
mod unit_test;
use bitvec::field::BitField;
use bitvec::prelude::*;
use bitvec::vec::BitVec;
use std::fs::File;
use std::io;

/// Compresses a sequence of bits using a `BitVec` representation of bits
/// the run-length algorithm
pub struct RunLength<T, O>
where
    T: bitvec::store::BitStore,
    O: bitvec::order::BitOrder,
{
    bits: BitVec<T, O>,
}
impl<T, O> RunLength<T, O>
where
    T: bitvec::store::BitStore,
    O: bitvec::order::BitOrder,
{
    pub fn new() -> Self {
        Self {
            bits: BitVec::new(),
        }
    }
    pub fn init(bit_vec: BitVec<T, O>) -> Self {
        Self { bits: bit_vec }
    }
    pub fn bits(&self) -> &BitVec<T, O> {
        &self.bits
    }
    pub fn push(&mut self, bit: bool) {
        self.bits.push(bit)
    }
    pub fn extend_from_raw_slice(&mut self, slice: &[T]) {
        self.bits.extend_from_raw_slice(slice)
    }
    pub fn compress(&self) -> Vec<usize> {
        let len = self.bits.len();
        let mut comp = Vec::<usize>::with_capacity(len);
        if len > 0 {
            let mut old = self.bits[0];
            let mut run = 0;
            if self.bits[0] == true {
                // The first bit is 1
                comp.push(0);
            }
            for k in 0..len {
                let bit = self.bits[k];
                if bit == old {
                    run += 1;
                } else {
                    comp.push(run);
                    run = 1;
                    old = !old;
                }
            }
            comp.push(run);
        }
        comp
    }

    pub fn expand(&self, comp: Vec<usize>, capacity: usize) -> BitVec<T, O> {
        let len = comp.len();
        assert!(capacity >= len);
        let mut exp = BitVec::<T, O>::with_capacity(capacity);
        for (pos, run) in comp.iter().enumerate() {
            let bit = pos % 2 == 1;
            for _ in 0..*run {
                exp.push(bit);
            }
        }
        exp
    }
}
impl<T, O> RunLength<T, O>
where
    T: bitvec::store::BitStore,
    O: bitvec::order::BitOrder,
    BitSlice<T, O>: BitField,
{
    pub fn from_file(path: &str) -> Self {
        let mut writer = BitVec::<T, O>::new();
        let mut reader = File::open(path);
        match reader {
            Ok(mut file) => {
                io::copy(&mut file, &mut writer);
                Self { bits: writer }
            }
            Err(error) => panic!("{error:?}"),
        }
    }
}
