mod utils;
use utils::{QuickFindUF, QuickUnionUF, WeightedQuickUnionPCUF};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut nb_objects = 0;
    let mut nb_iter = 0;
    let mut uf = WeightedQuickUnionPCUF::new_empty();

    if let Ok(lines) = read_lines("./sample.txt") {
        // Consumes the iterator, returns an (Optional) String
        for (pos, line) in lines.enumerate() {
            if let Ok(row) = line {
                let values = row.split(' ').collect::<Vec<&str>>();
                if pos == 0 {
                    nb_objects = values[0].parse::<usize>().unwrap();
                    uf = WeightedQuickUnionPCUF::new(nb_objects);
                } else {
                    let (p, q) = (
                        values[0].parse::<usize>().unwrap(), 
                        values[1].parse::<usize>().unwrap()
                    );
                    if !uf.connected(p, q){
                        uf.union(p,q);
                        // println!("Connection between objects {p} and {q}");
                        // println!("{:?}", uf.ids);
                        println!("{}", nb_iter);
                        nb_iter +=1
                    }
                }
            }
        }
    }
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}