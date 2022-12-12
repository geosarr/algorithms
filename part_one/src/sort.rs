use std::mem::replace;

#[derive(Debug, Default)]
pub struct Merge<T> {
    pub vec: Vec<T>,
}

impl<T: PartialOrd + Default + Copy> Merge<T> {
    pub fn new() -> Self {
        Default::default()
    }

    fn is_sorted(&mut self, low: usize, high: usize) -> bool{
        if self.vec[low..high].is_empty() {
            return true;
        }
        for k in low..high {
            if self.vec[k] > self.vec[k+1] {
                return false;
            }
        }
        true
    }

    pub fn merge(&mut self, mut aux_vec: Vec<T>, low: usize, high: usize, mid: usize) {
        // The halves of v, that is v[lo..mid+1] and v[mid+1..], should be sorted
        assert!(self.is_sorted(low, mid));
        assert!(self.is_sorted(mid+1, high));

        aux_vec = self.vec.clone();
        let mut i = low; let mut j = mid+1;
        for k in low..high+1 {
            if i > mid {
                let _ = replace(&mut self.vec[k], aux_vec[j]);
                j += 1;
            } else if j > high {
                let _ = replace(&mut self.vec[k], aux_vec[i]);
                i += 1;
            } else if aux_vec[j] <= aux_vec[i] {
                let _ = replace(&mut self.vec[k], aux_vec[j]);
                j += 1;
            } else {
                let _ = replace(&mut self.vec[k], aux_vec[i]);
                i += 1;
            }
        }
        assert!(self.is_sorted(low, high));
    }
    
    fn _sort(&mut self, aux_vec: Vec<T>, low: usize, high: usize) {
        let mid = low + (high - low) / 2;
        if high <= low { return }
        self._sort(aux_vec.clone(), low, mid);
        self._sort(aux_vec.clone(), mid+1, high);
        self.merge(aux_vec.clone(), low, high, mid);
    }

    pub fn sort(&mut self){
        let mut aux_vec = Vec::<T>::new();
        self._sort(aux_vec, 0, self.vec.len() - 1);
    }
}