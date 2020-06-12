mod computer;

use computer::Computer;
use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

pub struct Robot {
    pc: JoinHandle<()>,
    pc_sender: Sender<i64>,
    pc_receiver: Receiver<i64>,
    position: Point,
    orientation: Orientation,
    panels_painted: HashMap<Point, Color>,
}

struct RobotInstruction {
    paint_color: Color,
    turn_direction: Turn,
}

enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

enum Turn {
    Left,
    Right,
}

impl RobotInstruction {
    fn from(input_1: i64, input_2: i64) -> Self {
        let color = match input_1 {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Invalid input"),
        };

        let turn = match input_2 {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("Invalid input"),
        };

        Self {
            paint_color: color,
            turn_direction: turn,
        }
    }
}

impl Robot {
    pub fn new(program: Vec<i64>) -> Self {
        let mut computer = Computer::new();
        let (pc_input_sender, pc_input_receiver): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (pc_output_sender, pc_output_receiver): (Sender<i64>, Receiver<i64>) = mpsc::channel();

        computer.add_concurreny_channel(pc_output_sender, pc_input_receiver);
        computer.load_program(program, vec![]);

        let computer_thread = thread::spawn(move || {
            computer.run();
        });

        Self {
            pc: computer_thread,
            pc_sender: pc_input_sender,
            pc_receiver: pc_output_receiver,
            position: (0, 0),
            orientation: Orientation::Up,
            panels_painted: HashMap::new(),
        }
    }

    fn send_input(&self, value: i64) {
        self.pc_sender
            .send(value)
            .expect("Error sending value to receiver");
    }

    fn wait_output(&self) -> Option<RobotInstruction> {
        let output_1 = self.pc_receiver.recv().ok()?;
        let output_2 = self.pc_receiver.recv().ok()?;

        Some(RobotInstruction::from(output_1, output_2))
    }

    fn process_instruction(&mut self, instruction: RobotInstruction) {
        // Paint panel
        self.panels_painted
            .insert(self.position, instruction.paint_color);

        // Orientate the robot
        self.orientation = match self.orientation {
            Orientation::Up => match instruction.turn_direction {
                Turn::Left => Orientation::Left,
                Turn::Right => Orientation::Right,
            },
            Orientation::Right => match instruction.turn_direction {
                Turn::Left => Orientation::Up,
                Turn::Right => Orientation::Down,
            },
            Orientation::Down => match instruction.turn_direction {
                Turn::Left => Orientation::Right,
                Turn::Right => Orientation::Left,
            },
            Orientation::Left => match instruction.turn_direction {
                Turn::Left => Orientation::Down,
                Turn::Right => Orientation::Up,
            },
        };

        // Step over
        self.position = match self.orientation {
            Orientation::Up => (self.position.0, self.position.1 + 1),
            Orientation::Right => (self.position.0 - 1, self.position.1),
            Orientation::Down => (self.position.0, self.position.1 - 1),
            Orientation::Left => (self.position.0 + 1, self.position.1),
        }
    }

    fn get_current_panel_color(&self) -> &Color {
        self.panels_painted
                .get(&self.position)
                .unwrap_or(&Color::Black)
    }

    pub fn run(mut self, first_input: i64) -> HashMap<Point, Color> {
        self.send_input(first_input);

        loop {
            let response = self.wait_output();

            match response {
                Some(instruction) => {
                    self.process_instruction(instruction);

                    let panel_color = match self.get_current_panel_color() {
                        Color::Black => 0,
                        Color::White => 1,
                    };
                    self.send_input(panel_color);
                }
                None => break,
            }
        }

        self.pc.join().expect("Computer panicked");

        self.panels_painted
    }
}

type Point = (i64, i64);