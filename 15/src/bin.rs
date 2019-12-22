mod lib;
use lib::*;

fn main() {
    let dist = biggest_dist_to_oxygen();
    println!("\n{}", dist);
}
