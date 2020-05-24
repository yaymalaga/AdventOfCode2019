pub struct Computer {
    program: Vec<IntCode>,
    pointer: usize,
}

#[derive(PartialEq)]
enum Status {
    Running,
    Halt,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            program: Vec::new(),
            pointer: 0,
        }
    }

    pub fn load_program(&mut self, program: Vec<IntCode>) {
        self.program = program;
        self.pointer = 0;
    }

    fn run_step(self: &mut Self) -> Option<Status> {
        let status;
        if !self.program.is_empty() {
            let opcode = self.get_value(self.pointer);
            match opcode {
                1 | 2 => {
                    let first_value = self.get_value(self.get_first_parameter() as usize);
                    let second_value = self.get_value(self.get_second_parameter() as usize);
                    let result_position = self.get_third_parameter() as usize;

                    let result = if opcode == 1 as u32 {
                        first_value + second_value
                    } else {
                        first_value * second_value
                    };

                    self.write_value(result_position, result);

                    status = Status::Running;
                }
                99 => status = Status::Halt,
                _ => panic!(format!("invalid opcode {}", opcode)),
            }
            self.pointer += 4;
            Some(status)
        } else {
            None
        }
    }

    pub fn run(&mut self) {
        loop {
            let status = self.run_step();
            if let None = status {
                panic!("No program loaded in the computer");
            } else if let Some(Status::Halt) = status {
                break;
            }
        }
    }

    fn get_first_parameter(&self) -> IntCode {
        self.get_value(self.pointer + 1)
    }

    fn get_second_parameter(&self) -> IntCode {
        self.get_value(self.pointer + 2)
    }

    fn get_third_parameter(&self) -> IntCode {
        self.get_value(self.pointer + 3)
    }

    pub fn get_value(&self, position: usize) -> IntCode {
        *self.program.get(position).expect(&format!(
            "Invalid read index: {}. Program length is {}",
            position,
            self.program.len()
        ))
    }

    pub fn write_value(&mut self, position: usize, value: IntCode) {
        if self.program.len() > position {
            self.program[position] = value;
        } else {
            panic!(format!(
                "Invalid write index: {} Program length is {}",
                position,
                self.program.len()
            ));
        }
    }
}

type IntCode = u32;
