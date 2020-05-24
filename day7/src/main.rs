use day7::{AmplificationCircuit, AmplificationCircuitMode};

use std::fs::File;
use std::io::prelude::*;

const FILE_NAME: &str = "input";

fn main() {
    let mut file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Invalid data");
    let program: Vec<i32> = data
        .split(',')
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    let first_result = AmplificationCircuit::get_largest_signal(AmplificationCircuitMode::Serial, program.clone(), 0, 4);
    println!("PART 1: {}", first_result);

    let second_result = AmplificationCircuit::get_largest_signal(AmplificationCircuitMode::Feedback, program, 5, 9);
    println!("PART 2: {}", second_result);
}
