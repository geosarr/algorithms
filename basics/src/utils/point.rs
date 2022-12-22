use std::cmp::{PartialOrd, Ord, Ordering};
use std::fmt;

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

impl<T> LineSegment<T> {
    pub fn init(point1: Point<T>, point2: Point<T>) -> Self {
        Self { p: point1, q: point2 }
    }
}
