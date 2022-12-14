use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct Point<T> where T: Clone{
    x: T,
    y: T,
}


impl<T: fmt::Display + Clone> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "({}, {})", self.x, self.y)
    }
}




impl<T: Default + ToString + PartialOrd + Clone> Point<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(_x: T, _y: T) -> Self {
        Self {x: _x, y: _y}
    }

    pub fn slope_to(&self, other: &Self) -> f32
    {
        // computes the slope between two points
        // this computation can be wrong when the coordinates
        // are too large (due to conversions)
        let y0 = self.y.to_string().parse::<f32>().expect("Failed to convert to f32"); 
        let x0 = self.x.to_string().parse::<f32>().expect("Failed to convert to f32");
        let y1 = other.y.to_string().parse::<f32>().expect("Failed to convert to f32");
        let x1 = other.x.to_string().parse::<f32>().expect("Failed to convert to f32");
        
        if x1 != x0 {
            (y1 - y0) / (x1 - x0) 
        } else if x1 == x0 && y1 != y0 {
            f32::INFINITY
        } else {
            f32::NEG_INFINITY
        }  
        
    }

    pub fn compare_to(&self, other: &Self) -> isize {
        if self.y < other.y || (self.y == other.y && self.x < other.x) {
            // self < other
            return -1;
        } else if self.x == other.x && self.y == other.y {
            // self == other
            return 0
        } else {
            // self > other
            return 1
        }
    }

    pub fn slope_order(&self, p1: &Self, p2: &Self) -> Ordering {
        let slope_01 = self.slope_to(&p1);
        let slope_02 = self.slope_to(&p2);
        if slope_01 < slope_02 {
            return Ordering::Less
        } else if slope_01 == slope_02 {
            return Ordering::Equal
        } else {
            return Ordering::Greater
        }
    }
}

#[derive(Debug, Default)]
pub struct LineSegment<T> where T: Clone{
    pub p: Point<T>,
    pub q: Point<T>,
}

impl<T: fmt::Display + Clone> fmt::Display for LineSegment<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} --> {}", self.p, self.q)
    }
}

impl<T: Default + Clone> LineSegment<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct BruteCollinearPoints<T> where T: Clone {
    pub vec: Vec<Point<T>>,
}

impl<T: Default + Clone> BruteCollinearPoints<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(v: Vec<Point<T>>) -> Self {
        Self { vec: v }
    }

    pub fn number_of_segments(&self) -> usize {
        self.segments().len()
    }

    pub fn segments(&self) -> Vec<LineSegment<T>> {
        let mut v = Vec::<LineSegment<T>>::new();
        let n = v.len();
        for i in 0..n {
            let p = self.vec[i].clone();
            for j in i+1..n{
                let q = self.vec[j].clone();
                let slope1 = p.slope_to(q);
                for k in j+1..n{
                    let r = self.vec[k].clone();
                    let slope2 = p.slope_to(r);
                    for l in k+1..n{
                        let s = self.vec[l].clone();
                        let slope3 = p.slope_to(s);
                        if slope1 == slope2 && slope2 == slope3 {
                            v.push(LineSegment::<T>::new()) // to change
                        }
                    }
                }
            }
        }
        v
    }
}