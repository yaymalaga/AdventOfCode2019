mod computer;

use computer::Computer;
use std::{thread, sync::mpsc::{Receiver, Sender, self}, time::Duration, io::stdout};
use thread::JoinHandle;
use crossterm::{
    ExecutableCommand, cursor::{RestorePosition, SavePosition, Hide, Show, MoveTo}
};

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Tile {
    fn from_id(id: i64) -> Self {
        match id {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::HorizontalPaddle,
            4 => Self::Ball,
            _ => panic!("Invalid tile id"),
        }
    }

    fn id(&self) -> usize {
        match self {
            Tile::Empty => 0,
            Tile::Wall => 1,
            Tile::Block => 2,
            Tile::HorizontalPaddle => 3,
            Tile::Ball => 4,
        }
    }
}

enum JoyStick {
    Neutral,
    Left,
    Right,
}

impl JoyStick {
    fn from(input: &char) -> Self {
        match input {
            'a' => JoyStick::Left,
            's' => JoyStick::Neutral,
            'd' => JoyStick::Right,
            _ => {
                println!("{}", input);
                panic!("Invalid joystick input");
            },
        }
    }

    fn id(&self) -> i64 {
        match self {
            JoyStick::Neutral => 0,
            JoyStick::Left => -1,
            JoyStick::Right => 1
        }
    }
}

pub struct ArcadeCabinet {
    pc: JoinHandle<()>,
    screen: Vec<Vec<Tile>>,
    pc_sender: Sender<i64>,
    pc_receiver: Receiver<i64>,
    //quarters: Option<i64>,
    score: usize,
    ball_pos_x: usize,
    paddle_pos_x: usize,
    cursor_end_pos: Option<(u16, u16)>
}

impl ArcadeCabinet {
    pub fn new(game: Vec<i64>, quarters: Option<i64>) -> Self {
        let mut computer = Computer::new();
        
        let (pc_input_sender, pc_input_receiver): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (pc_output_sender, pc_output_receiver): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        computer.add_concurreny_channel(pc_output_sender, pc_input_receiver);
        
        computer.load_program(game, vec![]);

        if let Some(quarters) = quarters {
            computer.write_value(0, quarters);
        }

        let computer_thread = thread::spawn(move || {
            computer.run();
        });

        Self { pc: computer_thread, screen: vec![vec![]], pc_sender: pc_input_sender, pc_receiver: pc_output_receiver, score: 0, ball_pos_x: 0, paddle_pos_x: 0, cursor_end_pos: None }
    }

    pub fn run_game_step(&mut self) {    
        let mut tiles_data = Vec::new();
        while let Some(output) = self.pc_receiver.recv_timeout(Duration::from_millis(1)).ok() {
            tiles_data.push(output);
        }

        for tile_data in tiles_data.chunks_exact(3) {
            let x = tile_data[0];
            let y = tile_data[1];

            if x == -1 && y == 0 {
                self.score = tile_data[2] as usize;
                continue;
            }

            let x = x as usize;
            let y = y as usize;
            let tile = Tile::from_id(tile_data[2]);

            if tile == Tile::Ball {
                self.ball_pos_x = x;
            } else if tile == Tile::HorizontalPaddle {
                self.paddle_pos_x = x;
            }

            if self.screen.len() <= y {
                let empty_vector = || -> Vec<Tile> { vec![] };
                self.screen.resize_with((y + 1) as usize, empty_vector);
            }

            if self.screen[y].len() <= x {
                let empty_tile = || -> Tile { Tile::from_id(0) };
                self.screen[y].resize_with((x + 1) as usize, empty_tile)
            }

            self.screen[y][x] = tile;
        }
    }

    pub fn get_number_block_tiles(&self) -> usize {
        self.screen.iter().flatten().filter(|tile| **tile == Tile::Block ).count()
    }

    pub fn get_ball_pos_x(&self) -> usize {
        self.ball_pos_x
    }

    pub fn get_paddle_pos_x(&self) -> usize {
        self.paddle_pos_x
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn print_screen(&mut self) {
        stdout().execute(SavePosition).expect("Error interacting with the console");
        stdout().execute(Hide).expect("Error interacting with the console");

        for y in &self.screen {
            for tile in y {
                let tile = match tile.id() {
                    0 => " ",
                    1 => "=",
                    2 => "#",
                    3 => "-",
                    4 => "*",
                    _ => panic!("Invalid tile id")
                };
                print!("{}", tile)
            }
            println!()
        }
        println!("SCORE: {}", self.score);
        
        if self.cursor_end_pos.is_none() {
            self.cursor_end_pos = crossterm::cursor::position().ok();
        }
        stdout().execute(RestorePosition).expect("Error interacting with the console");
    }

    pub fn send_joystick_input(&self, input: &char) -> Option<()> {
        let status = self.pc_sender.send(JoyStick::from(input).id()).ok();

        if status.is_none() && self.cursor_end_pos.is_some() {
            let cursor_end_pos = self.cursor_end_pos.unwrap();
            stdout().execute(MoveTo(cursor_end_pos.0, cursor_end_pos.1)).expect("Error interacting with the console");
            stdout().execute(Show).expect("Error interacting with the console");
        }

        status
    }

    pub fn shutdown(self) {
        self.pc.join().expect("Computer thread panicked");
    }
}