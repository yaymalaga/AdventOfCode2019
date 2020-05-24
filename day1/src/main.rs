use day1::Module;

use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "input";

fn main() {
    let file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");
    let reader = BufReader::new(file);

    let mut total_fuel_1: u32 = 0;
    let mut total_fuel_2: u32 = 0; 
    for line in reader.lines() {
        let data: u32 = line.expect("Error while reading the file").parse().expect("Invalid number"); 
        let module = Module::from(data);
        total_fuel_1 += module.initial_fuel();
        total_fuel_2 += module.initial_fuel() + module.extra_fuel();
    }

    println!("PROBLEM 1 - Total fuel needed is: {}", total_fuel_1);

    println!("PROBLEM 2 - Total fuel needed is: {}", total_fuel_2);
}
