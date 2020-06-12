mod computer;

use computer::Computer;
use std::{collections::HashMap, sync::mpsc::{self, Receiver, Sender}, thread::{self, JoinHandle, }};

struct Robot {
    pc: JoinHandle<()>,
    pc_sender: Sender<i64>,
    pc_receiver: Receiver<i64>,
    position: Point,
    orientation: Orientation,
    pub panels_painted: HashMap<Point, Color>,
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
enum Color {
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
            _ => panic!("Invalid input")
        };

        let turn = match input_2 {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("Invalid input")
        };

        Self { paint_color: color, turn_direction: turn }
    }
}

impl Robot {
    fn new(program: Vec<i64>) -> Self {
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
        self.panels_painted.insert(self.position, instruction.paint_color);

        // Orientate the robot
        self.orientation = match self.orientation {
            Orientation::Up => {
                match instruction.turn_direction {
                    Turn::Left => Orientation::Left,
                    Turn::Right => Orientation::Right,
                }
            },
            Orientation::Right => {
                match instruction.turn_direction {
                    Turn::Left => Orientation::Up,
                    Turn::Right => Orientation::Down,
                }
            },
            Orientation::Down => {
                match instruction.turn_direction {
                    Turn::Left => Orientation::Right,
                    Turn::Right => Orientation::Left,
                }
            },
            Orientation::Left => {
                match instruction.turn_direction {
                    Turn::Left => Orientation::Down,
                    Turn::Right => Orientation::Up,
                }
            }
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
        if !self.panels_painted.contains_key(&self.position) {
            &Color::Black
        } else {
            self.panels_painted.get(&self.position).expect("Invalid hashmap reference")
        }
    }

    fn run(mut self, first_input: i64) -> HashMap<Point, Color> {
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
                },
                None => break
            }
        }

        self.pc.join().expect("Computer panicked");

        self.panels_painted
    }
}

type Point = (i64, i64);

fn main() {
    let robot = Robot::new(vec![3,8,1005,8,324,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1001,8,0,29,1,1107,14,10,1006,0,63,1006,0,71,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,61,1,103,18,10,1006,0,14,1,105,7,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,94,1006,0,37,1006,0,55,2,1101,15,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,101,0,8,126,2,1006,12,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,152,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,173,1006,0,51,1006,0,26,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,202,2,8,18,10,1,103,19,10,1,1102,1,10,1006,0,85,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,238,2,1002,8,10,1006,0,41,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,267,2,1108,17,10,2,105,11,10,1006,0,59,1006,0,90,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1001,8,0,304,101,1,9,9,1007,9,993,10,1005,10,15,99,109,646,104,0,104,1,21102,936735777688,1,1,21101,341,0,0,1105,1,445,21101,0,937264173716,1,21101,352,0,0,1106,0,445,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,3245513819,0,1,21102,1,399,0,1105,1,445,21102,1,29086470235,1,21102,410,1,0,1105,1,445,3,10,104,0,104,0,3,10,104,0,104,0,21101,825544712960,0,1,21102,1,433,0,1106,0,445,21102,825460826472,1,1,21101,0,444,0,1106,0,445,99,109,2,22102,1,-1,1,21101,0,40,2,21101,0,476,3,21102,466,1,0,1105,1,509,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,471,472,487,4,0,1001,471,1,471,108,4,471,10,1006,10,503,1101,0,0,471,109,-2,2106,0,0,0,109,4,2101,0,-1,508,1207,-3,0,10,1006,10,526,21101,0,0,-3,21202,-3,1,1,21201,-2,0,2,21101,0,1,3,21101,0,545,0,1105,1,550,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,573,2207,-4,-2,10,1006,10,573,21202,-4,1,-4,1106,0,641,21202,-4,1,1,21201,-3,-1,2,21202,-2,2,3,21101,0,592,0,1105,1,550,22101,0,1,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,611,21102,1,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,633,22101,0,-1,1,21102,633,1,0,105,1,508,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0]);
    let painted_panels = robot.run(0);
    println!("Part 1: {}", painted_panels.len());

    let robot = Robot::new(vec![3,8,1005,8,324,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1001,8,0,29,1,1107,14,10,1006,0,63,1006,0,71,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,61,1,103,18,10,1006,0,14,1,105,7,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,101,0,8,94,1006,0,37,1006,0,55,2,1101,15,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,101,0,8,126,2,1006,12,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,1,10,4,10,1001,8,0,152,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,173,1006,0,51,1006,0,26,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1001,8,0,202,2,8,18,10,1,103,19,10,1,1102,1,10,1006,0,85,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,1001,8,0,238,2,1002,8,10,1006,0,41,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,267,2,1108,17,10,2,105,11,10,1006,0,59,1006,0,90,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1001,8,0,304,101,1,9,9,1007,9,993,10,1005,10,15,99,109,646,104,0,104,1,21102,936735777688,1,1,21101,341,0,0,1105,1,445,21101,0,937264173716,1,21101,352,0,0,1106,0,445,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,3245513819,0,1,21102,1,399,0,1105,1,445,21102,1,29086470235,1,21102,410,1,0,1105,1,445,3,10,104,0,104,0,3,10,104,0,104,0,21101,825544712960,0,1,21102,1,433,0,1106,0,445,21102,825460826472,1,1,21101,0,444,0,1106,0,445,99,109,2,22102,1,-1,1,21101,0,40,2,21101,0,476,3,21102,466,1,0,1105,1,509,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,471,472,487,4,0,1001,471,1,471,108,4,471,10,1006,10,503,1101,0,0,471,109,-2,2106,0,0,0,109,4,2101,0,-1,508,1207,-3,0,10,1006,10,526,21101,0,0,-3,21202,-3,1,1,21201,-2,0,2,21101,0,1,3,21101,0,545,0,1105,1,550,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,573,2207,-4,-2,10,1006,10,573,21202,-4,1,-4,1106,0,641,21202,-4,1,1,21201,-3,-1,2,21202,-2,2,3,21101,0,592,0,1105,1,550,22101,0,1,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,611,21102,1,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,633,22101,0,-1,1,21102,633,1,0,105,1,508,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0]);
    let painted_panels = robot.run(1);

    let mut min_x = None;
    let mut min_y = None;
    let mut max_x = None;
    let mut max_y = None;
    for (position, color) in &painted_panels {
        // Use white color to get boundary box
        if *color == Color::Black {
            continue;
        }

        let update_min_x = match min_x {
            None => true,
            Some(x) => position.0 < x
        };
        if update_min_x {
            min_x = Some(position.0);
        }

        let update_min_y = match min_y {
            None => true,
            Some(y) => position.1 < y
        };
        if update_min_y {
            min_y = Some(position.1);
        }

        let update_max_x = match max_x {
            None => true,
            Some(x) => position.0 > x
        };
        if update_max_x {
            max_x = Some(position.0);
        }

        let update_max_y = match max_y {
            None => true,
            Some(y) => position.1 > y
        };
        if update_max_y {
            max_y = Some(position.1);
        }
    }

    // Offset to move everything to origin (0,0)
    let offset_x = -1 * min_x.unwrap();
    let offset_y = -1 * min_y.unwrap();

    // Total length equals to the offset len + max distance from the new origin. 
    // We add 1 as coordenates start from (0,0)
    let length_x = offset_x.abs() + max_x.unwrap() + 1;
    let length_y = offset_y.abs() + max_y.unwrap() + 1;

    let mut matrix: Vec<Vec<&str>> = Vec::with_capacity(length_y as usize);
    for _ in 0..length_y {
        let mut new_vec = Vec::with_capacity(length_x as usize);
        new_vec.resize(length_x as usize, " ");
        matrix.push(new_vec);
    }

    for (point, color) in painted_panels {
        match color {
            Color::White => matrix[(point.1 + offset_y) as usize][(point.0 + offset_x) as usize] = "#",
            Color::Black => (),
        }
    }

    println!("Part 2:");
    for row in matrix.iter() {
        for item in row.iter() {
            print!("{}", item)
        }
        print!("\n")
    }
}
