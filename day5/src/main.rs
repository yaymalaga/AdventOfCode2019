use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io;

fn main() {
    let opcodes: Vec<i32> = generate_opcodes_list();
    
    // input = 1
    let result_1 = get_opcodes_result(&mut opcodes.clone());
    println!("Result 1: {}", result_1);

    // input = 5
    let result_2 = get_opcodes_result(&mut opcodes.clone());
    println!("Result 2: {}", result_2);
}

fn generate_opcodes_list() -> Vec<i32> {
    let file = File::open("./input").unwrap();
    let mut reader = BufReader::new(file);

    let mut opcodes_input = String::new();
    reader.read_line(&mut opcodes_input).unwrap();

    opcodes_input.split(',').map(|x| x.parse().expect("Invalid opcode list number")).collect()
}

fn get_opcodes_result(opcodes: &mut Vec<i32>) -> i32 {
    let mut result = 0;
    let mut opcode_pointer = 0;
    while opcode_pointer < opcodes.len() {
        let complete_operation = format!("{:0>5}", opcodes[opcode_pointer]);
        let operation: i32 = complete_operation[3..].parse().expect("Invalid operation code");
        if operation == 99 || opcode_pointer+3 > opcodes.len() {
            break;
        }

        if operation == 1 || operation == 2 || operation == 7 || operation == 8 {
            let input_pos1 = opcodes[opcode_pointer+1];
            let input_pos2 = opcodes[opcode_pointer+2];
            let output_pos = opcodes[opcode_pointer+3];
            
            let input1 = 
                if complete_operation.chars().nth(2).unwrap() == "0".chars().nth(0).unwrap() {
                    opcodes[input_pos1 as usize]
                } else if complete_operation.chars().nth(2).unwrap() == "1".chars().nth(0).unwrap() {
                    input_pos1
                } else {
                    println!("error1");
                    break;
                };

            let input2 = 
                if complete_operation.chars().nth(1).unwrap() == "0".chars().nth(0).unwrap() {
                    opcodes[input_pos2 as usize]
                } else if complete_operation.chars().nth(1).unwrap() == "1".chars().nth(0).unwrap() {
                    input_pos2
                } else {
                    println!("error1");
                    break;
                };

            if operation == 1 {
                opcodes[output_pos as usize] = input1 + input2;
            } else if operation == 2 {
                opcodes[output_pos as usize] = input1 * input2;
            } else if operation == 7 {
                opcodes[output_pos as usize] = 
                    if input1 < input2 {
                        1
                    } else {
                        0
                    };
            } else if operation == 8 {
                opcodes[output_pos as usize] = 
                    if input1 == input2 {
                        1
                    } else {
                        0
                    };
            }

            opcode_pointer += 4;
        } else if operation == 3 || operation == 4 {
            let input_pos1 = opcodes[opcode_pointer+1];

            if operation == 3 {
                let mut user_input = String::new();
                println!("Introduce an integer: ");
                io::stdin().read_line(&mut user_input).unwrap();
                let user_input: i32 = user_input.trim().parse().expect("Invalid user input");
                opcodes[input_pos1 as usize] = user_input;
            } else if operation == 4 {
                if complete_operation.chars().nth(2).unwrap() == "0".chars().nth(0).unwrap() {
                    result = opcodes[input_pos1 as usize];
                } else if complete_operation.chars().nth(2).unwrap() == "1".chars().nth(0).unwrap() {
                    result = input_pos1;
                } else {
                    println!("error1");
                    break;
                };
                println!("Output: {}", result);
            }
            opcode_pointer += 2;
        } else if operation == 5 || operation == 6 {
            let input_pos1 = opcodes[opcode_pointer+1];
            let input_pos2 = opcodes[opcode_pointer+2];
            
            let input1 = 
                if complete_operation.chars().nth(2).unwrap() == "0".chars().nth(0).unwrap() {
                    opcodes[input_pos1 as usize]
                } else if complete_operation.chars().nth(2).unwrap() == "1".chars().nth(0).unwrap() {
                    input_pos1
                } else {
                    println!("error1");
                    break;
                };

            let input2 = 
                if complete_operation.chars().nth(1).unwrap() == "0".chars().nth(0).unwrap() {
                    opcodes[input_pos2 as usize]
                } else if complete_operation.chars().nth(1).unwrap() == "1".chars().nth(0).unwrap() {
                    input_pos2
                } else {
                    println!("error1");
                    break;
                };

            if operation == 5 && input1 != 0 || operation == 6 && input1 == 0 {
                opcode_pointer = input2 as usize;
            } else {
                opcode_pointer += 3;
            }
        } else {
            break;
        }
    }

    result
}