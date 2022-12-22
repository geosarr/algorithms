#[cfg(test)]
mod unit_test;
use crate::search_algorithm::binary_search;

pub struct ThreeSum {
    // Solves the problem find among distincts values a1, a2, .., an,
    // how many triplets sump up to a target value. The run time complexity is O(N^2log(N))
    target: isize,
    // list the values ak
    vec: Vec<isize>,
}

impl ThreeSum {
    pub fn init(t: isize, v: Vec<isize>) -> Self {
        Self { target: t, vec: v }
    }
    pub fn run(&mut self) -> usize {
        // run time complexity O(N^2 log(N))
        let mut res = 0;
        // the binary search algo needs the values to be sorted first in ascending order
        self.vec.sort_unstable();
        let n = self.vec.len();
        for i in 0..n {
            for j in (i + 1)..n {
                // binary search target-vec[i]-vec[j] in vec[(j+1)..]
                let a = self.vec[i];
                let b = self.vec[j];
                let key = self.target - a - b;
                let mut ind = binary_search(key, &self.vec[(j + 1)..]);
                ind = j as isize + 1 + ind;
                if ind >= j as isize + 1 {
                    // a solution is found
                    res += 1;
                    // println!("({}) + ({}) + ({})", a, b, self.vec[ind as usize]);
                }
            }
        }
        // println!("Result:");
        res
    }
}
