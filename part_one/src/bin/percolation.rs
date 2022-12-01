use lib::{Percolation, PercolationStats, Algorithm};

fn main() {
    // let mut percol = Percolation::init(50, Algorithm::WeightedQuickUnionPathComp);
    // println!("{:?}", percol);
    // percol.state[1] = true;
    // println!("{:?}", percol);
    // println!("{}", percol.number_of_open_sites());
    // println!("{}", percol.threshold());
    let mut percol_stats = PercolationStats::init(50,  Algorithm::QuickFind, 50);
    // println!("{:?}", percol_stats);
    percol_stats.compute();
    println!("\nmean = {}", percol_stats.mean());
    println!("unbiased standard deviation : {}", percol_stats.stddev());
    let ci = percol_stats.conf_interval();
    println!("confidence interval [{}, {}]", ci["low"], ci["up"]);

    // use rand::Rng;

    // let mut rng = rand::thread_rng();
    // loop{
    //     println!("{}", rng.gen::<f32>());
    // }
    
}

