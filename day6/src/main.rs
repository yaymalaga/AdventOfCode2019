use day6::OrbitMap;

use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "input";

fn main() {
    let file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");
    let reader = BufReader::new(file);

    let mut orbit_map = OrbitMap::new();
    for line in reader.lines() {
        let line = line.expect("Invalid line");
        orbit_map.add_orbit(&line);
    }

    println!("PART 1: {}", orbit_map.get_checksum());
    println!("PART 2: {}", orbit_map.get_minimum_orbit_transfers("YOU", "SAN").expect("Orbit transfer is impossible") - 2);
}
