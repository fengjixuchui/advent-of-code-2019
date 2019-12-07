mod utility;
use std::collections::HashSet;
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

fn part_2() {
    let input = load_file("resources/day_7_input.txt");
    let stopwatch = Stopwatch::start_new();
    let mut ids = split_by_into_numbers(input, ",".parse().unwrap());

    let mut highest_signal = 0;
    let mut highest_sequence: Vec<i64> = Vec::new();

    let interval = 5..10;
    for a_input in interval.clone() {
        for b_input in interval.clone() {
            for c_input in interval.clone() {
                for d_input in interval.clone() {
                    for e_input in interval.clone() {
                        let mut valid_input: HashSet<i64> = HashSet::new();
                        valid_input.insert(a_input);
                        valid_input.insert(b_input);
                        valid_input.insert(c_input);
                        valid_input.insert(d_input);
                        valid_input.insert(e_input);
                        if valid_input.len() < 5 {
                            continue;
                        }
                        // A
                        let mut a_computer = Computer::new(ids.clone());
                        a_computer.input.push(a_input);
                        a_computer.input.push(0);
                        // B
                        let mut b_computer = Computer::new(ids.clone());
                        b_computer.input.push(b_input);
                        // C
                        let mut c_computer = Computer::new(ids.clone());
                        c_computer.input.push(c_input);
                        // D
                        let mut d_computer = Computer::new(ids.clone());
                        d_computer.input.push(d_input);
                        // E
                        let mut e_computer = Computer::new(ids.clone());
                        e_computer.input.push(e_input);

                        while a_computer.running
                            || b_computer.running
                            || c_computer.running
                            || d_computer.running
                            || e_computer.running
                        {
                            // A
                            if a_computer.running {
                                a_computer.input.append(e_computer.output.as_mut());
                                a_computer.run_instructions();
                            }
                            // B
                            if b_computer.running {
                                b_computer.input.append(a_computer.output.as_mut());
                                b_computer.run_instructions();
                            }
                            // C
                            if c_computer.running {
                                c_computer.input.append(b_computer.output.as_mut());
                                c_computer.run_instructions();
                            }
                            // D
                            if d_computer.running {
                                d_computer.input.append(c_computer.output.as_mut());
                                d_computer.run_instructions();
                            }
                            // E
                            if e_computer.running {
                                e_computer.input.append(d_computer.output.as_mut());
                                e_computer.run_instructions();
                            }
                        }

                        if *e_computer.output.last().unwrap() > highest_signal {
                            highest_signal = *e_computer.output.last().unwrap();
                            highest_sequence = vec![a_input, b_input, c_input, d_input, e_input];
                        }
                    }
                }
            }
        }
    }

    stopwatch.print_elapsed();
    println!("Sent to thrusters {}", highest_signal);
    println!("Highest sequence {:?}", highest_sequence);
}

fn part_1() {
    let input = load_file("resources/day_7_input.txt");
    let stopwatch = Stopwatch::start_new();
    let mut ids = split_by_into_numbers(input, ",".parse().unwrap());

    let mut highest_signal = 0;
    let mut highest_sequence: Vec<i64> = Vec::new();

    let interval = 0..5;
    for a_input in interval.clone() {
        for b_input in interval.clone() {
            for c_input in interval.clone() {
                for d_input in interval.clone() {
                    for e_input in interval.clone() {
                        let mut valid_input: HashSet<i64> = HashSet::new();
                        valid_input.insert(a_input);
                        valid_input.insert(b_input);
                        valid_input.insert(c_input);
                        valid_input.insert(d_input);
                        valid_input.insert(e_input);
                        if valid_input.len() < 5 {
                            continue;
                        }
                        let sequence = vec![a_input, b_input, c_input, d_input, e_input];
                        // A
                        let mut a_computer = Computer::new(ids.clone());
                        a_computer.input.push(sequence[0]);
                        a_computer.input.push(0);
                        a_computer.run_instructions();
                        let a_output = a_computer.output.clone();
                        // B
                        let mut b_computer = Computer::new(ids.clone());
                        b_computer.input.push(sequence[1]);
                        b_computer.input.append(a_computer.output.clone().as_mut());
                        b_computer.run_instructions();
                        let b_output = b_computer.output.clone();
                        // C
                        let mut c_computer = Computer::new(ids.clone());
                        c_computer.input.push(sequence[2]);
                        c_computer.input.append(b_computer.output.clone().as_mut());
                        c_computer.run_instructions();
                        let c_output = c_computer.output.clone();
                        // D
                        let mut d_computer = Computer::new(ids.clone());
                        d_computer.input.push(sequence[3]);
                        d_computer.input.append(c_computer.output.clone().as_mut());
                        d_computer.run_instructions();
                        let d_output = d_computer.output.clone();
                        // E
                        let mut e_computer = Computer::new(ids.clone());
                        e_computer.input.push(sequence[4]);
                        e_computer.input.append(d_computer.output.clone().as_mut());
                        e_computer.run_instructions();
                        let e_output = e_computer.output.clone();
                        if e_output[0] > highest_signal {
                            highest_signal = e_output[0];
                            highest_sequence = sequence.clone();
                        }
                    }
                }
            }
        }
    }

    stopwatch.print_elapsed();
    println!("Sent to thrusters {}", highest_signal);
    println!("Highest sequence {:?}", highest_sequence);
}

struct Computer {
    pub position: usize,
    pub input: Vec<i64>,
    pub input_position: usize,
    pub output: Vec<i64>,
    pub instruction_memory: Vec<i64>,
    pub running: bool,
}

impl Computer {
    pub const POSITION_MODE: i64 = 0;
    pub const IMMEDIATE_MODE: i64 = 1;
    pub fn new(instruction_memory: Vec<i64>) -> Computer {
        Computer {
            position: 0,
            input: vec![],
            input_position: 0,
            output: vec![],
            instruction_memory: instruction_memory.clone(),
            running: true,
        }
    }
    fn run_instructions(&mut self) {
        loop {
            let full_instruction: i64 = self.instruction_memory[self.position];
            let mut parameter_mode_vec: Vec<i64> = full_instruction
                .to_string()
                .chars()
                .map(|char| (char.to_digit(10).unwrap_or(0)) as i64)
                .collect();
            let instruction_piece_right = parameter_mode_vec.pop().unwrap_or(0);
            let instruction_piece_left = parameter_mode_vec.pop().unwrap_or(0);
            let instruction: i64 = (instruction_piece_left.to_string()
                + instruction_piece_right.to_string().as_ref())
            .parse()
            .unwrap();
            //full_instruction_vec now has [0] = parameter mode 1, [1] parameter mode 2, ect
            parameter_mode_vec.reverse();

            match instruction {
                1 => {
                    let part1: i64 = self.get_part_value(&parameter_mode_vec, 1);
                    let part2: i64 = self.get_part_value(&parameter_mode_vec, 2);

                    let added = part1 + part2;
                    let part3 = self.instruction_memory[self.position + 3] as usize;
                    self.instruction_memory[part3] = added;
                    self.position += 4;
                }
                2 => {
                    let part1: i64 = self.get_part_value(&parameter_mode_vec, 1);
                    let part2: i64 = self.get_part_value(&parameter_mode_vec, 2);

                    let added = part1 * part2;
                    let part3 = self.instruction_memory[self.position + 3] as usize;
                    self.instruction_memory[part3] = added;
                    self.position += 4;
                }
                3 => {
                    /*
                    Opcode 3 takes a single integer as input and saves it to the address given by its only parameter.
                    For example, the instruction 3,50 would take an input value and store it at address 50.
                    */
                    //If no available input, exit
                    if self.input_position >= self.input.len() {
                        break;
                    }
                    let store_at_address = self.instruction_memory[self.position + 1] as usize;
                    let input_value = *self.input.get(self.input_position).unwrap_or(&0);
                    self.instruction_memory[store_at_address] = input_value;
                    self.input_position += 1;
                    self.position += 2;
                }
                4 => {
                    /*
                    Opcode 4 outputs the value of its only parameter.
                    For example, the instruction 4,50 would output the value at address 50.
                    */
                    let part1: i64 = self.get_part_value(&parameter_mode_vec, 1);
                    //println!("Output: {}", part1);
                    self.output.push(part1);
                    self.position += 2;
                }
                5 => {
                    /*
                    Opcode 5 is jump-if-true: if the first parameter is non-zero,
                    it sets the instruction pointer to the value from the second
                    parameter. Otherwise, it does nothing.
                    */
                    let part1: i64 = self.get_part_value(&parameter_mode_vec, 1);
                    if part1 != 0 {
                        let part2: i64 = self.get_part_value(&parameter_mode_vec, 2);
                        self.position = part2 as usize;
                    } else {
                        self.position += 3;
                    }
                }
                6 => {
                    /*
                    Opcode 6 is jump-if-false: if the first parameter is zero,
                    it sets the instruction pointer to the value from the second
                    parameter. Otherwise, it does nothing.
                    */
                    let part1: i64 = self.get_part_value(&parameter_mode_vec, 1);
                    if part1 == 0 {
                        let part2: i64 = self.get_part_value(&parameter_mode_vec, 2);
                        self.position = part2 as usize;
                    } else {
                        self.position += 3;
                    }
                }
                7 => {
                    /*
                    Opcode 7 is less than: if the first parameter is less than the
                    second parameter, it stores 1 in the position given by the third
                    parameter. Otherwise, it stores 0.
                    */
                    let part1: i64 = self.get_part_value(&parameter_mode_vec, 1);
                    let part2: i64 = self.get_part_value(&parameter_mode_vec, 2);
                    let part3 = self.instruction_memory[self.position + 3] as usize;
                    self.instruction_memory[part3] = if part1 < part2 { 1 } else { 0 };
                    self.position += 4;
                }
                8 => {
                    /*
                    Opcode 8 is equals: if the first parameter is equal to the second
                    parameter, it stores 1 in the position given by the third parameter.
                    Otherwise, it stores 0.
                    */
                    let part1: i64 = self.get_part_value(&parameter_mode_vec, 1);
                    let part2: i64 = self.get_part_value(&parameter_mode_vec, 2);
                    let part3 = self.instruction_memory[self.position + 3] as usize;
                    self.instruction_memory[part3] = if part1 == part2 { 1 } else { 0 };
                    self.position += 4;
                }
                99 => {
                    self.running = false;
                    break;
                }
                _ => {
                    println!("Error, invalid instruction {}! Exiting!", instruction);
                    self.running = false;
                    break;
                }
            }
        }
    }

    fn get_part_value(&self, parameter_mode_vec: &Vec<i64>, part_position: usize) -> i64 {
        let part_mode = *parameter_mode_vec
            .get(part_position - 1)
            .unwrap_or(&Computer::POSITION_MODE);
        match part_mode {
            Computer::POSITION_MODE => {
                let part_index = self.instruction_memory[self.position + part_position] as usize;
                self.instruction_memory[part_index]
            }
            Computer::IMMEDIATE_MODE => self.instruction_memory[self.position + part_position],
            _ => {
                println!("Error! Instruction {} invalid mode", self.position);
                0
            }
        }
    }
}

fn main() {
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
}
