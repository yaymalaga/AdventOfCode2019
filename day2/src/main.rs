use day2::Computer;

use std::io::prelude::*;
use std::fs::File;

const FILE_NAME: &str = "input";

fn main() {
    let mut file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Invalid data");
    let program: Vec<u32> = data.split(',').map(|x| x.parse().expect("Invalid number")).collect();
    
    let mut computer = Computer::new();
    computer.load_program(program.clone());
    computer.write_value(1, 12);
    computer.write_value(2, 2);
    computer.run();

    println!("PART1: {}", computer.get_value(0));

    'main: loop {
        for x in 0..=99 {
            for y in 0..=99 {
                computer.load_program(program.clone());
                computer.write_value(1, x);
                computer.write_value(2, y);
                computer.run();
                
                if computer.get_value(0) == 19690720 {
                    println!("PART2: {}", 100 * x + y);
                    break 'main;
                }
            }
        }
    }
}
