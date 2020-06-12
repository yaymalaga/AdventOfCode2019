use day11::{Robot, Color};

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
    
    let robot = Robot::new(program.clone());
    let painted_panels = robot.run(0);
    println!("Part 1: {}", painted_panels.len());

    let robot = Robot::new(program);
    let painted_panels = robot.run(1);

    let min_x = painted_panels.keys().min_by_key(|point| point.0).unwrap().0;
    let min_y = painted_panels.keys().min_by_key(|point| point.1).unwrap().1;
    let max_x = painted_panels.keys().max_by_key(|point| point.0).unwrap().0;
    let max_y = painted_panels.keys().max_by_key(|point| point.1).unwrap().1;

    println!("Part 2:");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let coordenate_color = painted_panels.get(&(x, y)).unwrap_or(&Color::Black);
            
            match coordenate_color {
                Color::White => print!("#"),
                Color::Black => print!(" "),
            }
        }

        println!()
    }
}
