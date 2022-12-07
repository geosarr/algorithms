pub mod binary_search;
use binary_search::binary_search;

pub struct ThreeSum{
    pub target: isize,
    pub vec: Vec<isize>,
}

impl ThreeSum{
    pub fn run(&mut self) -> usize{
    let mut res = 0;
    self.vec.sort_unstable();
    let n = self.vec.len();
    for i in 0..n{
        for j in (i+1)..n{
            // binary search target-vec[i]-vec[j] in vec[(j+1)..]
            let a = self.vec[i];
            let b = self.vec[j];
            let key = self.target-a-b;
            let mut ind = binary_search(key, &self.vec[(j+1)..]);
            if ind >= 0 {
                // a solution is found
                res+=1;
                // ind = j as isize + 1 + ind ; // index in vec
                // println!("({}) + ({}) + ({})", a, b, self.vec[ind as usize]);
                
    }}} 
        // println!("Result:");
        res
}}