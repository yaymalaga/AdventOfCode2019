use day12::{Position, Space};
use std::{fs::File, io::{BufRead, BufReader}};

const FILE_NAME: &str = "input";

fn main() {
    let file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");

    let mut space = Space::new();
    for line in BufReader::new(file).lines() {
        let moon_position = Position::from(&line.expect("Invalid line"));
        space.add_moon(moon_position);
    }
    
    space.run_steps(1000);

    println!("Part 1: {}", space.get_total_energy())
}
