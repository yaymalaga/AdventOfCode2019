pub struct Computer {
    inputs: Vec<IntCode>,
    outputs: Vec<IntCode>,
    program: Vec<IntCode>,
    pointer: usize,
}

#[derive(PartialEq)]
enum Status {
    Running,
    Halt,
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    opcode: OpCode,
    first_param_mode: ParamMode,
    second_param_mode: ParamMode,
    third_param_mode: ParamMode,
}

#[derive(Copy, Clone, Debug)]
enum ParamMode {
    Position,
    Inmediate,
}

#[derive(Copy, Clone, Debug)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            inputs: Vec::new(),
            outputs: Vec::new(),
            program: Vec::new(),
            pointer: 0,
        }
    }

    pub fn get_outputs(&self) -> Option<&Vec<i32>> {
        if self.outputs.is_empty() {
            None
        } else {
            Some(&self.outputs)
        }
    }

    pub fn load_program(&mut self, program: Vec<IntCode>, inputs: Vec<IntCode>) {
        self.inputs = inputs;
        self.program = program;
        self.outputs = Vec::new();
        self.pointer = 0;
    }

    fn run_step(self: &mut Self) -> Option<Status> {
        if !self.program.is_empty() {
            let status;

            let instruction = self.get_instruction();
            match instruction.opcode {
                OpCode::Add | OpCode::Multiply | OpCode::LessThan | OpCode::Equals => {
                    let first_param = match instruction.first_param_mode {
                        ParamMode::Position => self.get_value(self.get_first_parameter() as usize),
                        ParamMode::Inmediate => self.get_first_parameter(),
                    };

                    let second_param = match instruction.second_param_mode {
                        ParamMode::Position => self.get_value(self.get_second_parameter() as usize),
                        ParamMode::Inmediate => self.get_second_parameter(),
                    };

                    let result_position = match instruction.third_param_mode {
                        ParamMode::Position => self.get_third_parameter() as usize,
                        ParamMode::Inmediate => {
                            panic!("OPCODE result position can't have Inmediate mode")
                        }
                    };

                    let result = if let OpCode::Add = instruction.opcode {
                        first_param + second_param
                    } else if let OpCode::Multiply = instruction.opcode {
                        first_param * second_param
                    } else if let OpCode::LessThan = instruction.opcode {
                        if first_param < second_param {
                            1
                        } else {
                            0
                        }
                    } else {
                        if first_param == second_param {
                            1
                        } else {
                            0
                        }
                    };

                    self.write_value(result_position, result);

                    self.pointer += 4;
                    status = Status::Running;
                }
                OpCode::Input => {
                    let first_param = match instruction.first_param_mode {
                        ParamMode::Position => self.get_first_parameter() as usize,
                        ParamMode::Inmediate => panic!("OPCODE 3 param can't have Inmediate mode"),
                    };

                    let input = if !self.inputs.is_empty() {
                        self.inputs.remove(0)
                    } else {
                        panic!("No input is available");
                    };

                    self.write_value(first_param, input);

                    self.pointer += 2;
                    status = Status::Running;
                }
                OpCode::Output => {
                    let first_param = match instruction.first_param_mode {
                        ParamMode::Position => self.get_value(self.get_first_parameter() as usize),
                        ParamMode::Inmediate => self.get_first_parameter(),
                    };

                    self.outputs.push(first_param);

                    self.pointer += 2;
                    status = Status::Running;
                }
                OpCode::JumpIfTrue | OpCode::JumpIfFalse => {
                    let first_param = match instruction.first_param_mode {
                        ParamMode::Position => self.get_value(self.get_first_parameter() as usize),
                        ParamMode::Inmediate => self.get_first_parameter(),
                    };

                    let jump = if let OpCode::JumpIfTrue = instruction.opcode {
                        first_param != 0
                    } else {
                        first_param == 0
                    };

                    if jump {
                        let second_param = match instruction.second_param_mode {
                            ParamMode::Position => {
                                self.get_value(self.get_second_parameter() as usize)
                            }
                            ParamMode::Inmediate => self.get_second_parameter(),
                        };
                        self.pointer = second_param as usize;
                    } else {
                        self.pointer += 3;
                    }

                    status = Status::Running;
                }
                OpCode::Halt => status = Status::Halt,
            }

            Some(status)
        } else {
            None
        }
    }

    fn get_instruction(&self) -> Instruction {
        let instruction = format!("{:05}", self.get_value(self.pointer));
        let mut params = Vec::with_capacity(3);
        instruction[..3].chars().for_each(|x| {
            params.push(ParamMode::from(
                x.to_digit(10).expect("Invalid param digit"),
            ))
        });

        let opcode = OpCode::from(instruction[3..].parse().expect("Invalid opcode number"));

        Instruction::from(opcode, params[2], params[1], params[0])
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

impl Instruction {
    fn from(
        opcode: OpCode,
        first_param_mode: ParamMode,
        second_param_mode: ParamMode,
        third_param_mode: ParamMode,
    ) -> Instruction {
        Instruction {
            opcode,
            first_param_mode,
            second_param_mode,
            third_param_mode,
        }
    }
}

impl ParamMode {
    fn from(data: u32) -> Self {
        match data {
            0 => Self::Position,
            1 => Self::Inmediate,
            _ => panic!("Invalid ParamMode digit"),
        }
    }
}

impl OpCode {
    fn from(data: u32) -> Self {
        match data {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            _ => panic!(format!("invalid opcode {}", data)),
        }
    }
}

type IntCode = i32;
