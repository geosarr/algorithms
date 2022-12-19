#[cfg(test)]
mod unit_test;
use std::mem::replace;


#[derive(Debug, Default)]
pub struct InsertionSort<T>{
    pub vec: Vec<T>
}

impl<T: Default + Clone + PartialOrd> InsertionSort<T>{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(v: Vec<T>) -> Self {
        Self {vec: v}
    }

    pub fn sort(&mut self) {
        let n = self.vec.len();
        for i in 0..n{
            let mut j = i;
            while j > 0 && self.vec[j] < self.vec[j-1] {
                let val = self.vec[j-1].clone();
                self.vec[j-1] = replace(&mut self.vec[j], val);
                j -= 1;
            }
        }
    }

}