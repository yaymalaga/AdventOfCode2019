use day10::AsteroidsMap;

use std::fs::File;
use std::io::prelude::*;

const FILE_NAME: &str = "input";

fn main() {
    let mut file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");

    let asteroids_map = AsteroidsMap::from(&data);

    let (asteroids_detected, base_pos) = asteroids_map.calculate_best_base();
    let base_pos = base_pos.unwrap();
    println!(
        "Part 1: {} asteroids detected",
        asteroids_detected
    );

    let vaporized_asteroids = asteroids_map.calculate_vaporised_asteroids_queue(base_pos);

    println!(
        "Part 2: {}",
        vaporized_asteroids[199].0 * 100 + vaporized_asteroids[199].1
    );
}
