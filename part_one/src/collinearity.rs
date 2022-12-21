pub mod merge_sort;
pub mod quick_sort;
pub mod insertion_sort;
pub mod heap_sort;
pub use insertion_sort::InsertionSort;
pub use heap_sort::{HeapSort, BinaryHeapSort};
pub use merge_sort::{MergeSort, MergeSortAlgorithm};
pub use quick_sort::QuickSort;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Implementing the PartialOrd trait for Point<T> for ordering purpose
impl<T: Default + ToString + PartialOrd> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare_to(other))
    }
}

impl<T: Default + ToString + PartialOrd> Point<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(_x: T, _y: T) -> Self {
        Self { x: _x, y: _y }
    }

    pub fn slope_to(&self, other: &Self) -> f32 {
        // computes the slope between two points
        // this computation can be wrong when the coordinates
        // are too large (due to conversions)
        let y0 = self
            .y
            .to_string()
            .parse::<f32>()
            .expect("Failed to convert to f32");
        let x0 = self
            .x
            .to_string()
            .parse::<f32>()
            .expect("Failed to convert to f32");
        let y1 = other
            .y
            .to_string()
            .parse::<f32>()
            .expect("Failed to convert to f32");
        let x1 = other
            .x
            .to_string()
            .parse::<f32>()
            .expect("Failed to convert to f32");

        if x1 != x0 {
            (y1 - y0) / (x1 - x0)
        } else if x1 == x0 && y1 != y0 {
            f32::INFINITY
        } else {
            f32::NEG_INFINITY
        }
    }

    pub fn compare_to(&self, other: &Self) -> Ordering {
        // compares two points
        if self.y < other.y || (self.y == other.y && self.x < other.x) {
            // self < other
            // return -1;
            return Ordering::Less;
        } else if self.x == other.x && self.y == other.y {
            // self == other
            // return 0
            return Ordering::Equal;
        } else {
            // self > other
            // return 1
            return Ordering::Greater;
        }
    }

    pub fn slope_order(&self, p1: &Self, p2: &Self) -> Ordering {
        let slope_01 = self.slope_to(&p1);
        let slope_02 = self.slope_to(&p2);
        if slope_01 < slope_02 {
            return Ordering::Less;
        } else if slope_01 == slope_02 {
            return Ordering::Equal;
        } else {
            return Ordering::Greater;
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct LineSegment<T> {
    p: Point<T>,
    q: Point<T>,
}

impl<T: fmt::Display + Clone> fmt::Display for LineSegment<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} --> {}", self.p, self.q)
    }
}

impl<T: Default> LineSegment<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(_p: Point<T>, _q: Point<T>) -> Self {
        Self { p: _p, q: _q }
    }
}

#[derive(Debug, Default)]
pub struct BruteCollinearPoints<T> {
    // List of Points
    vec: Vec<Point<T>>,
    // Line segments of 4 points
    pub seg: Option<Vec<LineSegment<T>>>,
}

impl<T: Default + ToString + PartialOrd + Clone> BruteCollinearPoints<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(v: Vec<Point<T>>) -> Self {
        Self { vec: v, seg: None }
    }

    pub fn number_of_segments(&mut self) -> usize {
        match &self.seg {
            None => self.segments().len(),
            Some(lsegments) => lsegments.len(),
        }
    }

    pub fn segments(&mut self) -> Vec<LineSegment<T>> {
        // In this brute force version of finding all line segments
        // we check if 4 Points are collinear by checking if the slopes
        // between one of them and the rest are equal
        let mut v = Vec::<LineSegment<T>>::new();
        let n = self.vec.len();
        for i in 0..n {
            for j in i + 1..n {
                let slope1 = self.vec[i].slope_to(&self.vec[j]);
                for k in j + 1..n {
                    let slope2 = self.vec[i].slope_to(&self.vec[k]);
                    for l in k + 1..n {
                        let slope3 = self.vec[i].slope_to(&self.vec[l]);
                        // println!("{slope1}  {slope2}  {slope3}");
                        if slope1 == slope2 && slope2 == slope3 {
                            // self.vec[i], self.vec[j], self.vec[k], self.vec[l] are collinear,
                            // they are ordered with the self.compare_to(other)/partial_cmp order
                            // The extremal points will represent the line segment
                            let vec: Vec<Point<T>> = vec![
                                self.vec[i].clone(),
                                self.vec[j].clone(),
                                self.vec[k].clone(),
                                self.vec[l].clone(),
                            ];
                            let mut m = InsertionSort::init(vec);
                            let vec = m.into_sorted_vec();
                            v.push(LineSegment::init(vec[0].clone(), vec[3].clone()));
                        }
                    }
                }
            }
        }
        self.seg = Some(v.clone());
        v
    }
}
