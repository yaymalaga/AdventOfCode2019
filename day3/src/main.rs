use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    steps: i32,
}

fn main() {
    let file = File::open("./input").unwrap();
    let mut reader = BufReader::new(file);

    let mut wire1_data = String::new();
    reader.by_ref().read_line(&mut wire1_data).unwrap();
    let wire1_positions = generate_wire_positions(wire1_data.replace('\n', ""));

    let mut wire2_data = String::new();
    reader.by_ref().read_line(&mut wire2_data).unwrap();
    let wire2_positions = generate_wire_positions(wire2_data.replace('\n', ""));

    get_closer_interaction(wire1_positions, wire2_positions);
}

fn generate_wire_positions(data: String) -> Vec<Point> {

    let mut wire_positions: Vec<Point> = vec!(Point { x: 0, y: 0, steps: 0 });
    for item in data.split(',') {
        let code = item.chars().next().unwrap().to_string();
        let movement: Vec<char> = item.chars().skip(1).collect();
        let movement_str: String = movement.into_iter().collect();
        let number: i32 = movement_str.parse().expect("Number not valid");

        let point = &wire_positions.last().unwrap().clone();
        if code == "L" {
            for i in 1..=number {
                wire_positions.push(Point { x: point.x-i, y: point.y, steps: point.steps+i });
            }
        } else if code == "R" {
            for i in 1..=number {
                wire_positions.push(Point { x: point.x+i, y: point.y, steps: point.steps+i });
            }
        } else if code == "U" {
            for i in 1..=number {
                wire_positions.push(Point { x: point.x, y: point.y+i, steps: point.steps+i });
            }
        } else if code == "D" {
            for i in 1..=number {
                wire_positions.push(Point { x: point.x, y: point.y-i, steps: point.steps+i });
            }
        } else {
            println!("Unknown code");
        }
    }
    wire_positions
}

fn get_closer_interaction(data1: Vec<Point>, data2: Vec<Point>) {
    let mut interactions: Vec<Point> = Vec::new();
    for item1 in data1.iter() {
        for item2 in data2.iter() {
            if item1.x == item2.x && item1.y == item2.y && item1.x != 0 && item1.y != 0 {
                interactions.push(Point { x: item1.x, y: item1.y, steps: item1.steps + item2.steps });
            }
        }
    }

    let mut closest: i32 = interactions.first().unwrap().x.abs() + interactions.first().unwrap().y.abs();
    let mut shortest: i32 = interactions.first().unwrap().steps;
    for item in interactions.iter() {
        if item.x.abs() + item.y.abs() < shortest {
            closest = item.x.abs() + item.y.abs();
        }
        if item.steps < shortest {
            shortest = item.steps;
        }
    }

    println!("Part 1: {}", closest);
    println!("Part 2: {}", shortest);
}