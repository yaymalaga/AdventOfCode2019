use day14::{CookBook, Recipe};

use std::fs::File;
use std::io::{BufReader, prelude::*};

const FILE_NAME: &str = "input";

fn main() {
    let file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");

    let mut cookbook = CookBook::new();
    for line in BufReader::new(file).lines() {
        let recipe = Recipe::parse(&line.expect("Invalid line"));
        cookbook.add_recipe(recipe)
    }

    let part1 = cookbook.produce_from("ORE", "FUEL", 1);
    println!("Part 1: {}", part1);

    let part2 = cookbook.produce_maximum_from("ORE", 1_000_000_000_000, "FUEL");
    println!("Part 2: {}", part2);
}