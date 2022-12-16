pub mod algorithm;
pub mod input_output;
#[cfg(test)]
mod unit_test;
use algorithm::UnionFindAlgorithm;
use input_output::read_lines;
use std::path::Path;

#[derive(Debug, Default)]
pub struct UnionFind {
    // number of objects in the graph/tree
    // objects are 0..(nb_objects - 1)
    nb_objects: usize,

    // union-find algorithm
    algo: UnionFindAlgorithm,

    // objects connection information data
    // each objects j is connected to ids[j]
    ids: Vec<usize>,

    // optional size of the trees the objects belong to,
    // except for QuickFind algorithm (no tree representation)
    size: Vec<usize>,
    // Some axioms of the connection relation:
    //  - reflexive: object p is connected to p
    //  - transitive: p connected to q and q connected to r => q connected to r
    //  - symmetric: p connected to q => q connected to p
}

impl UnionFind {
    pub fn new() -> Self {
        let nb = 2;
        Self {
            nb_objects: nb,
            algo: UnionFindAlgorithm::default(),
            ids: (0..nb).collect::<Vec<usize>>(),
            size: vec![1; nb],
        }
    }

    pub fn init(nb: usize, algorithm: UnionFindAlgorithm) -> Self {
        // complexity: O(N)
        match algorithm {
            UnionFindAlgorithm::QuickFind => Self {
                nb_objects: nb,
                algo: algorithm,
                ids: (0..nb).collect::<Vec<usize>>(),
                size: Vec::new(),
            },
            _ => Self {
                nb_objects: nb,
                algo: algorithm,
                ids: (0..nb).collect::<Vec<usize>>(),
                size: vec![1; nb],
            },
        }
    }

    pub fn from_file<P>(filename: P, sep: char, algo: UnionFindAlgorithm) -> Self
    where
        P: AsRef<Path>,
    {
        // Builds an Union Find object from a file
        // first element is the number of objects
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
        if let UnionFindAlgorithm::QuickFind = self.algo {
            self.ids[i]
        } else {
            if let UnionFindAlgorithm::WeightedQuickUnionPathComp = self.algo {
                while i != self.ids[i] {
                    // one-pass path compression
                    self.ids[i] = self.ids[self.ids[i]];
                    i = self.ids[i];
                }
                i
            } else {
                while i != self.ids[i] {
                    i = self.ids[i];
                }
                i
            }
        }
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        if let UnionFindAlgorithm::QuickFind = self.algo {
            // complexity: O(1)
            self.ids[p] == self.ids[q]
        } else {
            // complexity: O(N) for QuickUnion, O(log(N)) for WeightedQuickUnion*
            self.root(p) == self.root(q)
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        match self.algo {
            UnionFindAlgorithm::QuickFind => {
                // complexity: O(N)
                let pid: usize = self.ids[p];
                let qid: usize = self.ids[q];
                // Connect to q all objects connected to p
                for i in 0..self.ids.len() {
                    if self.ids[i] == pid {
                        self.ids[i] = qid;
                    }
                }
            }
            UnionFindAlgorithm::QuickUnion => {
                // complexity: O(N) (tall trees)
                let i = self.root(p);
                let j = self.root(q);
                self.ids[i] = j;
            }
            _ => {
                // complexity: given roots (of complexity O(log(N)))
                // O(log(N)) or O(log*(N)) ~ O(1) with PathComp (path compression)
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
