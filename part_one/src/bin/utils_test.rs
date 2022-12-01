use std::thread;
use super::{*};
mod union_find;
use union_find::{UnionFind, Algorithm};

#[derive(Debug)]
pub struct Percolation {
    pub uf: UnionFind,
    pub state: Vec<bool>,
}

impl Percolation {
    pub fn from(n: usize, algo: Algorithm) -> Self {
        Self {
            uf: UnionFind::from(n, algo),
            state: vec![false; n * n],
        }
    }

    // pub fn open(&mut self, row: usize, col: usize) {
    //     if !self.is_open(row, col) {
    //         // open the site
    //         let site_id = row * self.nb + col;
    //         self.state[site_id] = true;

    //         // connect the site (row, col) to its open neighbors sites
    //         if row > 0 && self.is_open(row - 1, col){
    //             let neighbor_site_id = (row - 1) * self.nb + col;
    //             if !self.connected(site_id, neighbor_site_id){
    //                 self.union(site_id, neighbor_site_id);
    //             }
    //         }
    //         if col > 0 && self.is_open(row, col - 1) {
    //             let neighbor_site_id = row * self.nb + col - 1;
    //             if !self.connected(site_id, neighbor_site_id){
    //                 self.union(site_id, neighbor_site_id);
    //             }
    //         }
    //         if row < self.nb && self.is_open(row + 1, col) {
    //             let neighbor_site_id = (row + 1) * self.nb + col;
    //             if !self.connected(site_id, neighbor_site_id){
    //                 self.union(site_id, neighbor_site_id);
    //             }
    //         }
    //         if col < self.nb && self.is_open(row, col + 1) {
    //             let neighbor_site_id = row * self.nb + col + 1;
    //             if !self.connected(site_id, neighbor_site_id){
    //                 self.union(site_id, neighbor_site_id);
    //             }
    //         }
    //     }
    // }

    // pub fn is_open(&self, row: usize, col: usize) -> bool {
    //     self.state[row * self.nb + col]
    // }

    // pub fn root(&mut self, mut i: usize) -> usize {
    //     // Finding the root of an object i
    //     while i != self.ids[i] {
    //         self.ids[i] = self.ids[self.ids[i]]; // one-pass path compression
    //         i = self.ids[i];
    //     }
    //     i
    // }

    // pub fn connected(&mut self, p: usize, q: usize) -> bool {
    //     self.root(p) == self.root(q)
    // }

    // pub fn union(&mut self, p: usize, q: usize) {
    //     let i = self.root(p);
    //     let j = self.root(q);
    //     // Put the smallest tree under the tallest one
    //     if self.size[i] < self.size[j] {
    //         self.ids[i] = j;
    //         self.size[j] += self.size[i];
    //     } else {
    //         self.ids[j] = i;
    //         self.size[i] += self.size[j];
    //     }
    // }

    // pub fn is_full(&mut self, row: usize, col: usize) -> bool {
    //     if self.is_open(row, col) {
    //         let site_id = row*self.nb + col;
    //         // check if there is any top site connected to current site (row, col)
    //         // NB: top sites (i.e the self.nb first ids) are the roots 
    //         // of the trees they belong to
    //         let root = self.root(site_id);
    //         let top =  (0..self.nb).collect::<Vec<usize>>();
    //         top.iter().any(|&i| i==root)
    //     } else { false }
    // }

    // pub fn number_of_open_sites(&self) -> usize {
    //     self.state.iter()
    //               .map(|&open| if open {1} else {0})
    //               .sum()
    // }

    // pub fn is_percolated(&mut self) -> bool {
    //     let bottom = ((self.nb-1)*self.nb..self.nb*self.nb).collect::<Vec<usize>>(); 
    //     bottom.iter().any(|&i| self.is_full(i/self.nb, i%self.nb))
    // }
}


// pub struct PercolationStats{
//     nb: usize,
//     n_trials: usize,
// }

// impl PercolationStats{
//     pub fn from(n: usize, trials: usize) -> Vec<usize>{
//         let handle = thread::spawn(|| {
//             for _ in 0..trials{
//                 let mut percolation = Percolation::new(n);
//                 while !percolation.is_percolated(){
//                     percolation
//                 }
//             } 
//         });
//         let results = handle.join().unwrap();
//     }

//     pub mean(&self){}
// }