mod computer;

use computer::Computer;

use std::fs::File;
use std::io::prelude::*;

const FILE_NAME: &str = "input";

fn main() {
    let mut file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Invalid data");
    let program: Vec<i64> = data
        .split(',')
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    let mut computer = Computer::new();
    let inputs = vec![1];
    computer.load_program(program.clone(), inputs);
    computer.run();
    
    println!("PART 1: {:?}", computer.get_outputs().expect("No outputs available"));

    let inputs = vec![2];
    computer.load_program(program, inputs);
    computer.run();
    
    println!("PART 2: {:?}", computer.get_outputs().expect("No outputs available"));
}