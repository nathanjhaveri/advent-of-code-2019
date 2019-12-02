use std::error::{Error};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut total_fuel = 0;
    for line in f.lines() {
        let mass: i32 = line?.parse()?;
        let mut fuel = fuel_needed(mass);
        while fuel > 0 {
            total_fuel += fuel;
            fuel = fuel_needed(fuel);
        }
    }

    println!("total fuel: {}", total_fuel);

    Ok(())
}

fn fuel_needed(mass: i32) -> i32 {
    mass / 3 - 2
}

