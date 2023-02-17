use std::fs::File;
use std::io::{self, BufRead};
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct Reader<T> {
    filename: String,
    sep: char,
    value_type: PhantomData<T>,
}

// Light way to constraint T to be a primitive type using Copy (to be improved)
impl<T: Copy + FromStr + std::fmt::Debug> Reader<T> {
    pub fn init(file_abs_path: String, separator: char) -> Self {
        Self {
            filename: file_abs_path,
            sep: separator,
            value_type: PhantomData,
        }
    }

    pub fn into_vec(&self) -> Vec<Vec<T>>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        // creates a pseudo matrix of data row-wise from a file.
        let mut vec: Vec<Vec<T>> = Vec::new();
        // let mut counter = 1;
        if let Ok(lines) = read_lines(self.filename.as_str()) {
            for line in lines {
                if let Ok(row) = line {
                    // println!("{counter}");
                    let values: Vec<T> = row
                        .split(self.sep)
                        .map(|i| i.parse::<T>().unwrap())
                        .collect();
                    // vec.extend_from_slice(&values);
                    // println!("{:?}", values);
                    vec.push(values);
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
