#[cfg(test)]
mod unit_test;
use std::result::Result;

/// Get the index of key in vec if key is in vec
/// vec elements should be ordered in ascending order.
/// This seems to work only if the length of vec is small enough
/// so that conversions from usize to isize work normally
/// It is expected to run in O(log(N))
pub fn binary_search<T>(key: T, vec: &[T]) -> Result<usize, usize>
where
    T: Ord,
{
    
    if vec.len() > 0 {
        let mut high = vec.len() - 1;
        let mut low = 0;

        while low <= high {
            let mid = (high + low) / 2;
            if vec[mid] < key {
                low = mid + 1;
            } else if vec[mid] > key && mid > 0 {
                high = mid - 1;
            } else if vec[mid] > key && mid == 0 && vec[high] == key {
                // mid = 0 <=> high=0=low or (high=1 and low=0)
                // when mid=0 (mid-1 not working since mid is usize)
                // hence the following conditions
                return Ok(high);
            } else if vec[mid] > key && mid == 0 && vec[high] != key {
                return Err(mid);
            } else {
                return Ok(mid);
            }
        }
        return Err(high);
    } else {
        Err(0)
    }
}
