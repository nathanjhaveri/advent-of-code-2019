mod lib;
use lib::find_oxygen;

fn main() {
    let dist = find_oxygen();
    println!("\n{}", dist);
}
