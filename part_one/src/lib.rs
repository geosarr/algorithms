pub mod union_find;
pub use union_find::UnionFind;
pub use union_find::algorithm::Algorithm;
// use union_find::input_output::read_lines;
use std::thread;
use rand::prelude::*;
use std::thread::JoinHandle;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Percolation {
    pub grid_size: usize,
    pub uf: UnionFind,
    pub state: Vec<bool>,
}

impl Percolation {
    pub fn new() -> Self {
        Self {
            grid_size: 50,
            uf: UnionFind::new(),
            state: Vec::new(),
        }
    }

    pub fn init(n: usize, algo: Algorithm) -> Self {
        Self {
            grid_size: n,
            uf: UnionFind::init(n*n, algo),
            state: vec![false; n * n],
        }
    }

    fn open(&mut self, row: usize, col: usize) {
        if !self.is_open(row, col) {
            // open the site
            let site_id = row * self.grid_size + col;
            let nb = self.grid_size;
            self.state[site_id] = true;

            // connect the site (row, col) to its open neighbors sites
            if row > 0 && self.is_open(row - 1, col) {
                let neighbor_site_id = (row-1)*nb + col;
                if !self.uf.connected(site_id, neighbor_site_id){
                    self.uf.union(site_id, neighbor_site_id);
                }
            }
            if col > 0 && self.is_open(row, col - 1) {
                let neighbor_site_id = row*nb + col - 1;
                if !self.uf.connected(site_id, neighbor_site_id){
                    self.uf.union(site_id, neighbor_site_id);
                }
            }
            if row < nb - 1 && self.is_open(row + 1, col) {
                let neighbor_site_id = (row+1)*nb + col;
                if !self.uf.connected(site_id, neighbor_site_id){
                    self.uf.union(site_id, neighbor_site_id);
                }
            }
            if col < nb - 1 && self.is_open(row, col + 1) {
                let neighbor_site_id = row*nb + col + 1;
                if !self.uf.connected(site_id, neighbor_site_id){
                    self.uf.union(site_id, neighbor_site_id);
                }
            }
        }
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        self.state[row * self.grid_size + col]
    }

    fn is_full(&mut self, row: usize, col: usize) -> bool {
        let nb = self.grid_size;
        if self.is_open(row, col) {
            let site_id = row*nb + col;
            // check if there is any path of open sites connected the current site (row, col)
            // to at least one open top site
            // NB: top sites are the self.uf.nb_objects first objects
            let root = self.uf.root(site_id);
            let top =  (0..nb).collect::<Vec<usize>>();
            top.iter().any(|&i| self.uf.root(i)==root)
        } else { false }
    }

    fn number_of_open_sites(&self) -> usize {
        self.state.iter()
                  .map(|&open| if open {1} else {0})
                  .sum()
    }

    fn is_percolated(&mut self) -> bool {
        // checks whether or not at least one the bottom site is full
        let nb = self.grid_size;
        let bottom = ((nb-1)*nb..nb*nb).collect::<Vec<usize>>(); 
        bottom.iter().any(|&i| self.is_full(i/nb, i%nb))
    }

    pub fn threshold(&mut self) -> f32 {
        let mut rng = rand::thread_rng();
        let nb = self.grid_size;
        let candidates = (0..nb*nb).collect::<Vec<usize>>();
        while !self.is_percolated(){
            // Generate a random number between 0 and nb-1
            // It suffices to take u = floor(n*rand) where rand in (0,1)
            // let site_id = (n as f32)*rng.gen::<f32>()
            //                             .to_string()
            //                             .parse::<usize>()
            //                             .unwrap();
            let site_id = candidates.choose(&mut rng).unwrap();
            let (row, col) = (site_id/nb, site_id%nb);
            // Open the chosen site if not opened
            self.open(row, col);

        }
        (self.number_of_open_sites() as f32) / (self.uf.nb_objects as f32)
    }
}

#[derive(Debug)]
pub struct PercolationStats{
    grid_size: usize,
    algo: Algorithm,
    n_trials: usize,
    results: Vec<f32>,
}

impl PercolationStats{
    pub fn new() -> Self {
        Self {
            grid_size: 50,
            algo: Algorithm::QuickFind,
            n_trials: 10,
            results: Vec::new(),
        }
    } 

    pub fn init(n: usize, algorithm: Algorithm, trials: usize) -> Self {
        Self {
            grid_size: n,
            algo: algorithm,
            n_trials: trials,
            results: vec![0.0;n],
        }
    }
    pub fn compute(&mut self) {
        let grid_size: usize = self.grid_size; 
        let algo: Algorithm=self.algo; 
        let n_trials: usize=self.n_trials;
        let mut handles: Vec<JoinHandle<f32>> = Vec::new();

        for i in 0..n_trials {
            let id = i+1;
            println!("Running computation {id} over {n_trials}");
            let mut percolation = Percolation::init(grid_size, algo);
            let handle = thread::spawn(move || {
                // percolation = 
                percolation.threshold()
            });
            handles.push(handle)
        }

        self.results = handles.into_iter()
                              .map(|h| h.join().unwrap())
                              .collect::<Vec<f32>>();
        // println!("{:?}", self.results);
    }

    pub fn mean(&self) -> f32 {
        self.results.iter().sum::<f32>() / (self.n_trials as f32)
    }

    pub fn stddev(&self) -> f32 {
        let trials = self.n_trials as f32;
        let avg = self.mean();
        let sum_sq_avg_dist = self.results.iter()
                                          .map(|&i| (i - avg)
                                          .powf(2.0)).sum::<f32>();
        (sum_sq_avg_dist / (trials - 1.0)).sqrt()
    }

    pub fn conf_interval(&self) -> HashMap<&str, f32>{
        let avg = self.mean();
        let std = self.stddev();
        let trials = self.n_trials as f32;
        let low =  avg - ((1.96*std) / trials);
        let up =  avg + ((1.96*std) / trials);
        HashMap::from([("low", low) , ("up", up)])
    } 
}


