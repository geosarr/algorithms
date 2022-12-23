use crate::utils::{Point, LineSegment};
use crate::sort_algorithm::InsertionSort;
// use std::collections::HashSet;

#[derive(Debug)]
pub struct BruteCollinearPoints<T> {
    // List of Points
    vec: Vec<Point<T>>,
    // Line segments of 4 points
    seg: Option<Vec<LineSegment<T>>>,
}

impl<T> BruteCollinearPoints<T>{
    pub fn init(v: Vec<Point<T>>) -> Self {
        Self { vec: v, seg: None }
    }
}

impl<T: ToString + Ord + Clone> BruteCollinearPoints<T> {
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
        // run time complexity O(N^4)
        if !self.seg.is_none(){}
        else {
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
                                let m = InsertionSort::init(vec);
                                let vec = m.into_sorted_vec();
                                v.push(LineSegment::init(vec[0].clone(), vec[3].clone()));
                            }
                        }
                    }
                }
            }
            self.seg = Some(v);
        }
        self.seg.clone().expect("Failed to get LineSegments")
    }
}





#[derive(Debug)]
pub struct FastCollinearPoints<T> {
    // List of Points
    vec: Vec<Point<T>>,
    // Line segments of 4 points
    seg: Option<Vec<LineSegment<T>>>,
}

impl<T> FastCollinearPoints<T>{
    pub fn init(v: Vec<Point<T>>) -> Self {
        // make sure that points are different
        Self { vec: v, seg: None }
    }
}

impl<T: ToString + Ord + Clone + std::fmt::Debug> FastCollinearPoints<T> {
    pub fn number_of_segments(&mut self) -> usize {
        match &self.seg {
            None => self.segments().len(),
            Some(lsegments) => lsegments.len(),
        }
    }

    pub fn segments(&mut self) -> Vec<LineSegment<T>> {
        // run time complexity O(N^2)
        // may returns many times the same line(with different points)
        if !self.seg.is_none(){}
        else {
            let mut vec = Vec::<LineSegment<T>>::new();
            let n = self.vec.len();
            let mut vec_indices = (0..n).collect::<Vec<usize>>();
            println!("{:?}", vec_indices);
            for k in 0..n {
                // build the lines passing throught the k^th point 
                vec_indices.retain(|&e| e != k);
                let mut lines_with_point_k = vec_indices
                    .iter()
                    .map(|&l| LineSegment::<T>::init(
                        self.vec[k].clone(), 
                        self.vec[l].clone(),
                    ))
                    .collect::<Vec<LineSegment<T>>>();
                vec_indices.push(k);
                // println!("{:?}", vec_indices);
                println!("{:?}", lines_with_point_k);
                
                // ordering the lines with respect to their slope
                lines_with_point_k.sort();
                println!("{:?}", lines_with_point_k);
                
                // collinear points with point k are consecutive in lines_with_point_k 
                let mut i = 0;
                while i < n-1 {
                    let mut temp_vec = Vec::<Point<T>>::new();
                    temp_vec.push(lines_with_point_k[i].get_p()); // point k
                    temp_vec.push(lines_with_point_k[i].get_q());
                    let mut l = i+1;
                    while l < n-1 && lines_with_point_k[i].slope() == lines_with_point_k[l].slope() {
                        temp_vec.push(lines_with_point_k[l].get_q());                        
                        l += 1;
                    }
                    println!("temp_vec = {:?}, l={}", temp_vec, l);
                    if temp_vec.len() >= 4 {
                        // a line of at least 4 points is found
                        temp_vec.sort();
                        vec.push(LineSegment::<T>::init(temp_vec[0].clone(), temp_vec.pop().unwrap()));
                    }
                    println!("vec = {:?}", vec);
                    i = l;
                    println!("\n");
                }
            }
            vec.sort();
            // vec.dedup();
            self.seg = Some(vec);
        }
        self.seg.clone().expect("Failed to get LineSegments")
    }
}
