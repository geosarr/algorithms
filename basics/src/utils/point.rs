use std::cmp::{max, min, PartialOrd, Ord, Ordering};
use std::fmt;
use crate::utils::{read_lines};

#[derive(Debug, Clone)]
pub struct Point<T> {
    x: T,
    y: T,
}
impl<T: Default> Default for Point<T>{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}
impl<T: Default> Point<T> {
    pub fn new() -> Self {
        Default::default()
    }
}
impl<T> Point<T> {
    pub fn init(abscissa: T, ordinate: T) -> Self {
        Self { x: abscissa, y: ordinate}
    }
}
impl<T: Clone + ToString> Point<T>{
    pub fn get_x(&self) -> f32{
        self.x.clone().to_string()
        .parse::<f32>()
        .expect("Failed to convert abscissa to f32")
    }
    pub fn get_y(&self) -> f32{
        self.y.clone().to_string()
        .parse::<f32>()
        .expect("Failed to convert ordinate to f32")
    }
}
impl<T: std::str::FromStr + Clone> Point<T>
where <T as std::str::FromStr>::Err: std::fmt::Debug
{
    pub fn from_file_to_vec(filename: String, sep: char) -> Vec<Self> {
        // creates a list of points from a file
        // there should at least be to values in a given row of the file
        // the values should be "parsable" to T 
        let mut vec = Vec::<Self>::new();
        if let Ok(lines) = read_lines(filename.as_str()) {
            for line in lines.into_iter() {
                if let Ok(row) = line {
                    // println!("{counter}");
                    let values: Vec<T> = row
                        .split(sep)
                        .map(|i| i.parse::<T>().unwrap())
                        .collect();
                    vec.push(Point::<T>::init(values[0].clone(), values[1].clone()));
                } else {
                    panic!("bad row, check if the rows are correct.");
                }
                // counter += 1;
            }
        } else {
            panic!("Error in file, check its content, the separator and the file absolute path")
        }
        vec
        
    }
}
impl<T: ToString> Point<T> {
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
impl<T: Ord> Point<T> {
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
}
impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl<T: Ord> Ord for Point<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_to(other)
    }
}
impl<T: Ord> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Ord> Eq for Point<T> {}
impl<T: Ord> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}



//##################################################
#[derive(Debug, Clone)]
pub struct LineSegment<T> {
    p: Point<T>,
    q: Point<T>,
}
impl<T: fmt::Display> fmt::Display for LineSegment<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} --> {}", self.p, self.q)
    }
}
impl<T: Clone> LineSegment<T> {
    pub fn init(point1: Point<T>, point2: Point<T>) -> Self {
        Self { p: point1, q: point2 }
    }

    pub fn get_p(&self) -> Point<T>{
        self.p.clone()
    }

    pub fn get_q(&self) -> Point<T>{
        self.q.clone()
    }
}
impl<T: Ord + Clone> LineSegment<T> {
    pub fn largest_point(&self) -> Point<T>{
        max(self.p.clone(), self.q.clone())
    }

    pub fn smallest_point(&self) -> Point<T>{
        min(self.p.clone(), self.q.clone())
    }
}
impl<T: ToString> LineSegment<T> {
    pub fn slope(&self) -> f32 {
        self.p.slope_to(&self.q)
    }
}
impl<T: Ord + ToString + Clone> Ord for LineSegment<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lines are ordered with respect to their slopes
        if self.slope() < other.slope() {
            return Ordering::Less;
        } else if self.slope() > other.slope() {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}
impl<T: Ord + ToString + Clone> PartialOrd for LineSegment<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Ord + ToString + Clone> Eq for LineSegment<T> {}
impl<T: Ord + ToString + Clone> PartialEq for LineSegment<T> {
    fn eq(&self, other: &Self) -> bool {
        // (self.p == other.p && self.q == other.q) ||
        // (self.p == other.q && self.q == other.p) ||
        self.slope() == other.slope() 
        // && self.smallest_point() == other.smallest_point()
    }
}


//##################################################
#[derive(Debug, Clone)]
pub struct Segment<T> {
    p: Point<T>,
    q: Point<T>,
}
impl<T: fmt::Display> fmt::Display for Segment<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} --> {}", self.p, self.q)
    }
}
impl<T: Clone> Segment<T> {
    pub fn init(point1: Point<T>, point2: Point<T>) -> Self {
        Self { p: point1, q: point2 }
    }

    pub fn get_p(&self) -> Point<T>{
        self.p.clone()
    }

    pub fn get_q(&self) -> Point<T>{
        self.q.clone()
    }
}
impl<T: Ord + Clone> Segment<T> {
    pub fn largest_point(&self) -> Point<T>{
        max(self.p.clone(), self.q.clone())
    }

    pub fn smallest_point(&self) -> Point<T>{
        min(self.p.clone(), self.q.clone())
    }
}
impl<T: ToString> Segment<T> {
    pub fn slope(&self) -> f32 {
        self.p.slope_to(&self.q)
    }
}
impl<T: Ord + ToString + Clone> Ord for Segment<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // segments with strictly larger (smaller) slopes are larger (smaller)
        // 
        let slope1 = self.slope();
        let slope2 = other.slope();
        if slope1 < slope2 ||
        (slope1 == slope2 && -slope1 * self.get_p().get_x() + self.get_p().get_y() < -slope1 * other.get_p().get_x() + other.get_p().get_y()
                          && -slope1 * self.get_q().get_x() + self.get_q().get_y() < -slope1 * other.get_q().get_x() + other.get_q().get_y() // (for security purpose only)
        ){
            return Ordering::Less;
        } else if slope1 > slope2 ||
        (slope1 == slope2 && -slope1 * self.get_p().get_x() + self.get_p().get_y() > -slope1 * other.get_p().get_x() + other.get_p().get_y()
                          && -slope1 * self.get_q().get_x() + self.get_q().get_y() > -slope1 * other.get_q().get_x() + other.get_q().get_y() // (for security purpose only)
        ){   return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}
impl<T: Ord + ToString + Clone> PartialOrd for Segment<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Ord + ToString + Clone> Eq for Segment<T> {}
impl<T: Ord + ToString + Clone> PartialEq for Segment<T> {
    fn eq(&self, other: &Self) -> bool {
        // to segments are equal if they belong to the same line (i.e their extremal points are on the same line)
        // maybe better to put absolute value of differences instead of strict equality (due to f32 not Ord)
        self.slope()*(other.get_p().get_x() - self.get_p().get_x()) == other.get_p().get_y() - self.get_p().get_y() 
        && self.slope()*(other.get_q().get_x() - self.get_p().get_x()) == other.get_q().get_y() - self.get_p().get_y() 
    }
}