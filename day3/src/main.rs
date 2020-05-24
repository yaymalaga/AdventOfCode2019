use day3::Wire;

use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "input";

fn main() {
    let file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");
    let reader = BufReader::new(file);

    let mut wires_list = Vec::new();
    for line in reader.lines() {
        let mut wire = Wire::new();

        for coordenate in line.expect("Invalid text line").split(',') {
            wire.add_coordenate(coordenate);
        }

        wires_list.push(wire)
    }

    if wires_list.len() != 2 {
        panic!(format!("Invalid number of wires: {}", wires_list.len()));
    }

    let minimum_distance = Wire::nearest_intersection_distance(&wires_list[0], &wires_list[1])
        .expect("No intersections found.");
    println!(
        "Part 1 - Manhattan distance of the nearest intersection: {}",
        minimum_distance
    );

    let minimum_steps = Wire::nearest_intersection_steps(&wires_list[0], &wires_list[1])
        .expect("No intersections found.");
    println!(
        "Part 2 - Fewest steps to reach an intersection: {}",
        minimum_steps
    );
}
