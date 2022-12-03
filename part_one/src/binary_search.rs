
pub struct BinarySearch{
    // list of sorted integers
    // objective: find the index of a key in list
    //  if it exists.
    pub list: Vec<isize>
}

impl BinarySearch{
    pub fn new() -> Self {
        Self {
            list: vec![1,2,3,6,10,19,35,46,78,95],
        }
    }

    pub fn init(n: usize) -> Self {
        Self {
            list: (-(n as isize)..n as isize).collect::<Vec<isize>>(),
        }
    }

    pub fn find(&self, elt: isize) -> isize {
        // Find the index of elt if elt is in self.list
        // otherwise return -1
        let mut high = (self.list.len() - 1) as isize;
        let mut low = 0 as isize;

        while low <= high {
            let mid = ((high+low) / 2) as usize;
            if self.list[mid] < elt {
                low = (mid + 1) as isize;
            } else if self.list[mid] > elt{
                high = (mid) as isize - 1;
            } else {
                return (mid).try_into().unwrap();
            }
        }
        return (-1).try_into().unwrap();
    }
}


pub struct ThreeSum {
    pub algo: BinarySearch,
}

impl ThreeSum {
    pub fn run(&self) -> usize {
        // gives the number of triplet of elements in list
        // that sum up to zero
        let n = self.algo.list.len();
        let mut res = 0;
        for i in 0..n{
            for j in i+1..n{
                // search -list[i]-list[j] in list
                let a = self.algo.list[i];
                let b = self.algo.list[j];
                let key = -a-b;
                let ind = self.algo.find(key);
                if ind >= 0 && ind != i as isize && ind != j as isize{
                    // a solution is found
                    println!("({}) + ({}) + ({}) = 0", a, b, self.algo.list[ind as usize]);
                    res+=1;
                }
            }
        } 
        res
    }
}