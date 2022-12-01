pub mod input_output;
use std::path::Path;
use input_output::read_lines;

#[derive(Copy, Clone, Debug)]
pub enum Algorithm {
    QuickFind, 
    QuickUnion, 
    WeightedQuickUnion, 
    WeightedQuickUnionPathComp
}

#[derive(Debug)]
pub struct UnionFind {
    // number of objects in the graph/tree
    // objects are 0..(nb_objects - 1)
    pub nb_objects: usize,

    // union-find algorithm 
    pub algo: Algorithm,
    
    // objects connection information data
    // each objects is j is is connected to ids[j]
    pub ids: Vec<usize>,
    
    // optional size of the trees the objects belong to,
    // except for QuickFind algorithm (no tree representation)
    pub size: Vec<usize>,

    // Some axioms of the connection relation:
    //  - reflexive: object p is connected to p
    //  - transitive: p connected to q and q connected to r => q connected to r
    //  - symmetric: p connected to q => q connected to p
}

impl UnionFind {
    pub fn new() -> Self {
        // defining a simple union find structure
        let nb = 2;
        Self {
            nb_objects: nb,
            algo: Algorithm::QuickFind,
            ids: (0..nb).collect::<Vec<usize>>(),
            size: Vec::new(),
        }
    }

    pub fn init(nb: usize, algorithm: Algorithm) -> Self {
        match algorithm {
            Algorithm::QuickFind => Self {
                nb_objects: nb,
                algo: algorithm,
                ids: (0..nb).collect::<Vec<usize>>(),
                size: Vec::new(),
            },
            _ => Self {
                nb_objects: nb,
                algo: algorithm,
                ids: (0..nb).collect::<Vec<usize>>(),
                size: vec![1;nb],
            },
        }
    }

    pub fn union_find_from<P>(filename: P, sep: char, algo: Algorithm) -> UnionFind
    where
        P: AsRef<Path>,
    {
        // Builds an Union Find object from a file
        // first elements is the number of objects
        // the subsequent rows give the pairs of
        // connected objects
        let mut nb_iter = 0;
        let mut uf = UnionFind::new();

        if let Ok(lines) = read_lines(filename) {
            for (pos, line) in lines.enumerate() {
                if let Ok(row) = line {
                    let values = row.split(sep).collect::<Vec<&str>>();
                    if pos == 0 {
                        let nb_objects = values[0].parse::<usize>().unwrap();
                        uf = UnionFind::init(nb_objects, algo);
                    } else {
                        let (p, q) = (
                            values[0].parse::<usize>().unwrap(),
                            values[1].parse::<usize>().unwrap(),
                        );
                        if !uf.connected(p, q) {
                            uf.union(p, q);
                            println!("{}", nb_iter);
                            nb_iter += 1
                        }
                    }
                }
            }
            
        }
        uf
    }

    pub fn root(&mut self, mut i: usize) -> usize {
        // Finding the root of an object i
        match self.algo {
            // For QuickFind algorithm we could return whatever compiles 
            // because this algo does not use the notion of root
            Algorithm::QuickFind => self.ids[i],
            _ => { 
                while i != self.ids[i] {
                // one-pass path compression
                self.ids[i] = self.ids[self.ids[i]]; 
                i = self.ids[i];
                }
                i 
            }
        }
        
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        match self.algo {
            Algorithm::QuickFind => self.ids[p] == self.ids[q],
            _ => self.root(p) == self.root(q),
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        match self.algo {
            Algorithm::QuickFind => {
                let pid: usize = self.ids[p];
                let qid: usize = self.ids[q];
                // Replace the 
                for i in 0..self.ids.len() {
                    if self.ids[i] == pid {
                        self.ids[i] = qid;
                    }
                }
            }
            Algorithm::QuickUnion => {
                let i = self.root(p);
                let j = self.root(q);
                self.ids[i] = j;
            }
            _ => {
                let i = self.root(p);
                let j = self.root(q);
                // Put the smallest tree under the tallest one
                if self.size[i] < self.size[j] {
                    self.ids[i] = j;
                    self.size[j] += self.size[i];
                } else {
                    self.ids[j] = i;
                    self.size[i] += self.size[j];
                }
            }
        }
    }
}



