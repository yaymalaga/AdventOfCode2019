mod computer;

use computer::Computer;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
pub struct AmplificationCircuit {}

pub enum AmplificationCircuitMode {
    Serial,
    Feedback,
}

impl AmplificationCircuit {
    fn generate_phase_sequences(min_value: i32, max_value: i32) -> Vec<String> {
        let mut sequences = Vec::new();

        for a in min_value..=max_value {
            for b in min_value..=max_value {
                if b == a {
                    continue;
                }
                for c in min_value..=max_value {
                    if c == a || c == b {
                        continue;
                    }
                    for d in min_value..=max_value {
                        if d == a || d == b || d == c {
                            continue;
                        }
                        for e in min_value..=max_value {
                            if e == a || e == b || e == c || e == d {
                                continue;
                            }
                            sequences.push(format!("{}{}{}{}{}", a, b, c, d, e));
                        }
                    }
                }
            }
        }

        sequences
    }

    pub fn get_largest_signal(
        mode: AmplificationCircuitMode,
        program: Vec<i32>,
        phase_min: i32,
        phase_max: i32,
    ) -> i32 {
        let sequences = Self::generate_phase_sequences(phase_min, phase_max);
        let mut result = 0;

        for sequence in sequences {
            let output = match mode {
                AmplificationCircuitMode::Serial => {
                    Self::get_serial_amplifiers_result(program.clone(), sequence)
                }
                AmplificationCircuitMode::Feedback => {
                    Self::get_feedback_amplifiers_result(program.clone(), sequence)
                }
            };

            if output > result {
                result = output;
            }
        }

        result
    }

    fn get_serial_amplifiers_result(program: Vec<i32>, phases: String) -> i32 {
        let mut inputs = vec![0];

        for phase in phases.chars() {
            inputs.insert(0, phase.to_digit(10).expect("Invalid number") as i32);

            let mut computer = Computer::new();
            computer.load_program(program.clone(), inputs.clone());
            computer.run();

            inputs.clear();
            inputs.push(
                *computer
                    .get_outputs()
                    .expect("No outputs were found")
                    .last()
                    .unwrap(),
            );
        }

        *inputs.last().expect("No outputs were found")
    }

    fn get_feedback_amplifiers_result(program: Vec<i32>, phases: String) -> i32 {
        let (amp_a_sender, amp_b_receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let (amp_b_sender, amp_c_receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let (amp_c_sender, amp_d_receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let (amp_d_sender, amp_e_receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let (amp_e_sender, amp_a_receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();

        let mut senders_list = vec![
            amp_e_sender,
            amp_d_sender,
            amp_c_sender,
            amp_b_sender,
            amp_a_sender,
        ];
        let mut receivers_list = vec![
            amp_e_receiver,
            amp_d_receiver,
            amp_c_receiver,
            amp_b_receiver,
            amp_a_receiver,
        ];

        let result = Arc::new(Mutex::new(0));
        let mut threads_list = Vec::new();

        for (index, phase) in phases.chars().enumerate() {
            let phase = phase.to_digit(10).expect("Invalid phase number") as i32;

            let computer_program = program.clone();
            let computer_index = index.clone();
            let result_clone = Arc::clone(&result);

            let sender = senders_list.pop().unwrap();
            let receiver = receivers_list.pop().unwrap();

            let thread = thread::spawn(move || {
                let mut amp_computer = Computer::new();

                let inputs = if index == 0 {
                    vec![phase, 0]
                } else {
                    vec![phase]
                };

                amp_computer.load_program(computer_program, inputs);
                amp_computer.add_concurreny_channel(sender, receiver);
                amp_computer.run();

                if computer_index == 4 {
                    let output = *amp_computer
                        .get_outputs()
                        .expect("No outputs available")
                        .last()
                        .unwrap();

                    let mut num = result_clone.lock().unwrap();
                    *num = output;
                }
            });

            threads_list.push(thread);
        }

        for thread in threads_list {
            thread.join().unwrap();
        }

        let signal_value = *result.lock().unwrap();
        signal_value
    }
}
