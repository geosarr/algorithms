use clap::Parser;
use basics::{Point, FastCollinearPoints, BruteCollinearPoints};

#[derive(Parser)]
struct Cli {
    /// Absolute path to the file with the integers of the problem
    #[arg(short, long)]
    file_abs_path: String,

    /// Separator between the integers
    #[arg(short)]
    sep: char,
}

fn main(){
    let cli = Cli::parse();
    let filename = cli.file_abs_path;
    let sep = cli.sep;
    let points = Point::<isize>::from_file_to_vec(filename, sep);
    // println!("{:?} {:?}", points[0], points[1]);
    let mut brute_force = FastCollinearPoints::<isize>::init(points);
    let segments = brute_force.segments();
    println!("{}", segments.len()); 
    println!("{:?}", segments);
}


