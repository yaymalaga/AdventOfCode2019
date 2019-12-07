use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);

    let mut simple_fuel_counter = 0;
    let mut fuel_counter = 0;
    for line in reader.lines() {
        if let Ok(x) = line {

            let number: i32 = x.parse().expect("Invalid number");
            let mut module_fuel: i32 = (number/3)-2;
            simple_fuel_counter += module_fuel;

            let mut extra_fuel = module_fuel;
            while extra_fuel > 0 {
                extra_fuel = (extra_fuel/3)-2;
                if extra_fuel > 0 {
                    module_fuel += extra_fuel;
                }
            }

            fuel_counter += module_fuel;
        };
    }

    println!("Part 1: {}", simple_fuel_counter);
    println!("Part 2: {}", fuel_counter);
}