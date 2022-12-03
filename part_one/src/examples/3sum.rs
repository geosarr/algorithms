use lib::{BinarySearch, ThreeSum, read_lines};
use clap::{Parser};


#[derive(Parser)]
#[command(
    about = "\nFinds the number of triplets in a set of integers that sum up to zero",  
    long_about = "Solutions are counted in terms of triplets not sets, \nE.g solution (-1,0,1) != (1,-1,0)",    
)]
struct Cli {
    /// Absolute path to the file with the integers of the problem
    #[arg(short, long)]
    file_abs_path: String,

    /// Separator between the integers
    #[arg(short, default_value_t = ';')]
    sep: char,

}

fn main(){
    let cli = Cli::parse();
    let filename = cli.file_abs_path;
    let sep = cli.sep;
    let mut vec: Vec<isize> = Vec::new();
    if let Ok(lines) = read_lines(filename.as_str()) {
        for line in lines.into_iter() {
            if let Ok(row) = line {
                let values: Vec<isize> = row.split(sep)
                                            .map(|i| i.parse::<isize>().unwrap())
                                            .collect();
                vec.extend_from_slice(&values);
            } else {panic!("bad row, check if the rows are correct.");}
    }} 
    else {panic!("Error in file, check its content and the value of --sep")}
    
    let binary = BinarySearch{list: vec};
    let three_sum = ThreeSum{algo: binary}; 
    println!("{}", three_sum.run());
}