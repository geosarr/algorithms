

// Quick-Find: for fast find algorithm (O(1)) but slow union algorithm (O(N))
pub struct QuickFindUF{
    pub ids: Vec<usize>,
}

impl QuickFindUF{
    pub fn new_empty() -> QuickFindUF{
        QuickFindUF{
            ids: Vec::new(),
        }
    }

    pub fn new(nb: usize) -> QuickFindUF {
        QuickFindUF {
            ids : (0..nb).collect::<Vec<usize>>()}
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.ids[p] == self.ids[q]
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pid: usize = self.ids[p];
        let qid: usize = self.ids[q];
        for i in 0..self.ids.len() {
            if self.ids[i] == pid {
                self.ids[i] = qid;
            }
        }
    }
}

// Quick-Union: faster union algorithm in general but O(N) in a worst case scenario (tall trees)
pub struct QuickUnionUF{
    pub ids: Vec<usize>,
}

impl QuickUnionUF{
    pub fn new_empty() -> QuickUnionUF{
        QuickUnionUF{
            ids: Vec::new(),
        }
    }
    pub fn new(nb: usize) -> QuickUnionUF {
        QuickUnionUF {
            ids : (0..nb).collect::<Vec<usize>>()
        }
    }

    pub fn root(&self, mut i: usize) -> usize {
        while i != self.ids[i] {
            i = self.ids[i];
        }
        return i
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self,  p: usize, q: usize){
        let i = self.root(p);
        let j = self.root(q);
        self.ids[i] = j;
    }
}

// Weighted Quick Union

pub struct WeightedQuickUnionUF{
    ids: Vec<usize>,
    size: Vec<usize>,
}

impl WeightedQuickUnionUF{
    pub fn new_empty() -> WeightedQuickUnionUF{
        WeightedQuickUnionUF{
            ids: Vec::new(),
            size: Vec::new(),
        }
    }
    pub fn new(nb: usize) -> WeightedQuickUnionUF{
        WeightedQuickUnionUF{
            ids : (0..nb).collect::<Vec<usize>>(),
            size: vec![1;nb],
        }
    }
   
    pub fn root(&self, mut i: usize) -> usize {
        while i != self.ids[i] {
            i = self.ids[i];
        }
        return i
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self,  p: usize, q: usize){
        let i = self.root(p);
        let j = self.root(q);
        if self.size[i] < self.size[j]{
            self.ids[i] = j;
            self.size[j] += self.size[i]; 
        } else {
            self.ids[j] = i;
            self.size[i] += self.size[j]; 
        }
        
    }
}


// Weighted Quick Union with Path Compression

pub struct WeightedQuickUnionPCUF{
    ids: Vec<usize>,
    size: Vec<usize>,
}

impl WeightedQuickUnionPCUF{
    pub fn new_empty() -> WeightedQuickUnionPCUF{
        WeightedQuickUnionPCUF{
            ids: Vec::new(),
            size: Vec::new(),
        }
    }
    pub fn new(nb: usize) -> WeightedQuickUnionPCUF{
        WeightedQuickUnionPCUF{
            ids : (0..nb).collect::<Vec<usize>>(),
            size: vec![1;nb],
        }
    }
   
    pub fn root(&mut self, mut i: usize) -> usize {
        while i != self.ids[i] {
            self.ids[i] = self.ids[self.ids[i]]; // one-pass path compression
            i = self.ids[i];
        }
        return i
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self,  p: usize, q: usize){
        let i = self.root(p);
        let j = self.root(q);
        if self.size[i] < self.size[j]{
            self.ids[i] = j;
            self.size[j] += self.size[i]; 
        } else {
            self.ids[j] = i;
            self.size[i] += self.size[j]; 
        }
        
    }
}