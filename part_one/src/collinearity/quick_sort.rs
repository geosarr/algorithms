#[cfg(test)]
mod unit_test;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::mem::replace;

#[derive(Debug, Default)]
pub struct QuickSort<T> {
    pub vec: Vec<T>,
    // pub algo: MergeSortAlgorithm,
    // QuickSort is unstable in the sense that
    // items with equal keys can be exchanged ?
}

impl<T: PartialOrd + Default + Clone> QuickSort<T> {
    pub fn new() -> Self {
        Default::default()
    }

    fn partition(&mut self, low: usize, high: usize) -> usize {
        let mut i = low;
        // let mut j = high+1;
        let mut j = high + 1;
        loop {
            // find item on left to swap
            while self.vec[i + 1] <= self.vec[low] {
                i += 1;
                if i == high {
                    break;
                }
            }
            // find item on right to swap
            while self.vec[low] <= self.vec[j - 1] {
                j -= 1;
                if j == low {
                    break;
                }
            }
            // check if positions/pointers cross
            if i >= j {
                break;
            }
            let n = self.vec[j].clone();
            self.vec[j] = replace(&mut self.vec[i], n);
        }
        // swap with partitionning item
        let n = self.vec[j].clone();
        self.vec[j] = replace(&mut self.vec[low], n);
        return j;
    }

    pub fn sort(&mut self) {
        // shuffle garantees performance
        let mut rng = thread_rng();
        self.vec.shuffle(&mut rng);
        self.recursive_sort(0, self.vec.len() - 1);
    }

    fn recursive_sort(&mut self, low: usize, high: usize) {
        if high <= low {
            return;
        }
        let j: usize = self.partition(low, high);
        // println!("{j}");
        // if j >= 1 && j < self.vec.len() - 1{
        self.recursive_sort(low, j - 1);
        self.recursive_sort(j + 1, high);
        // }
    }

    pub fn select(&mut self, k: usize) -> T {
        // Selects the k^th element in self.vec
        // in linear time on average for median (k = N/2)
        let mut rng = thread_rng();
        self.vec.shuffle(&mut rng);
        let mut low = 0;
        let mut high = self.vec.len() - 1;
        while high > low {
            let j = self.partition(low, high);
            if j < k {
                low = j + 1;
            } else if j < k {
                high = j - 1;
            } else {
                return self.vec[k].clone();
            }
        }
        return self.vec[k].clone();
    }
}
