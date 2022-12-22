#[cfg(test)]
mod unit_test;
use std::mem::replace;


#[derive(Debug)]
pub struct InsertionSort<T>{
    vec: Vec<T>
}

impl<T: Clone + Ord> InsertionSort<T>{
    pub fn init(v: Vec<T>) -> Self {
        Self {vec: v}
    }

    pub fn into_sorted_vec(mut self) -> Vec<T>{
        let n = self.vec.len();
        for i in 0..n{
            let mut j = i;
            while j > 0 && self.vec[j] < self.vec[j-1] {
                let val = self.vec[j-1].clone();
                self.vec[j-1] = replace(&mut self.vec[j], val);
                j -= 1;
            }
        }
        self.vec
    }

}