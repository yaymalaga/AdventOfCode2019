use day13::ArcadeCabinet;

use std::fs::File;
use std::{cmp::Ordering, io::prelude::*};

const FILE_NAME: &str = "input";

fn main() {
    let mut file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Invalid data");
    let program: Vec<i64> = data
        .split(',')
        .map(|x| x.parse().expect("Invalid number"))
        .collect();
    
    let mut arcade_cabinet = ArcadeCabinet::new(program.clone(), None);
    arcade_cabinet.run_game_step();
    println!("Part 1: {}", arcade_cabinet.get_number_block_tiles());
    arcade_cabinet.shutdown();

    let mut arcade_cabinet = ArcadeCabinet::new(program, Some(2));
    loop {
        arcade_cabinet.run_game_step();

        arcade_cabinet.print_screen();

        let dx = arcade_cabinet.get_paddle_pos_x() as i64 - arcade_cabinet.get_ball_pos_x() as i64;
        let joystick_input = match dx.cmp(&0) {
            Ordering::Less => 'd',
            Ordering::Equal => 's',
            Ordering::Greater => 'a',
        };

        if arcade_cabinet.send_joystick_input(&joystick_input).is_none() {
            //End of the game
            break;
        }
    }

    if arcade_cabinet.get_number_block_tiles() != 0 {
        print!("Good luck next time!");
    } else {
        print!("Part 2: Congratulations! ");
    }
    println!("Your score is {}", arcade_cabinet.get_score());

    arcade_cabinet.shutdown();
}
