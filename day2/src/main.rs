use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut opcodes: Vec<u32> = generate_opcodes_list();
    opcodes[1] = 12;
    opcodes[2] = 2;
    
    let result_1 = get_opcodes_result(&mut opcodes.clone());
    
    let mut result_2 = 0;
    'main_loop: for a in 0..100 {
        for b in 0..100 {
            opcodes[1] = a;
            opcodes[2] = b;

            if get_opcodes_result(&mut opcodes.clone()) == 19_690_720 {
                result_2 = (100*a)+b;
                break 'main_loop;
            }
        }
    }

    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}

fn generate_opcodes_list() -> Vec<u32> {
    let file = File::open("./input").unwrap();
    let mut reader = BufReader::new(file);

    let mut opcodes_input = String::new();
    reader.read_line(&mut opcodes_input).unwrap();

    opcodes_input.split(',').map(|x| x.parse().expect("Invalid number")).collect()
}

fn get_opcodes_result(opcodes: &mut Vec<u32>) -> u32 {
    for i in (0..opcodes.len()).step_by(4) {
        let operation = opcodes[i];
        if operation == 99 || i+3 > opcodes.len() {
            break;
        }

        let input_pos1 = opcodes[i+1];
        if input_pos1 as usize > opcodes.len() {
            break;
        }

        let input_pos2 = opcodes[i+2];
        if input_pos2 as usize > opcodes.len() {
            break;
        }

        let output_pos = opcodes[i+3];
        if output_pos as usize > opcodes.len() {
            break;
        }

        let input1 = opcodes[input_pos1 as usize];
        let input2 = opcodes[input_pos2 as usize];

        if operation == 1 {
            opcodes[output_pos as usize] = input1 + input2;
        } else if operation == 2 {
            opcodes[output_pos as usize] = input1 * input2;
        }
    }

    opcodes[0]
}