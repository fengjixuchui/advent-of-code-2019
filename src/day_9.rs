mod utility;
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

fn part_1() {
    let input = load_file("resources/day_9_input.txt");
    let stopwatch = Stopwatch::start_new();
    let mut ids = split_by_into_numbers(input, ",".parse().unwrap());

    let mut a_computer = Computer::new(ids.clone());
    a_computer.input.push(2);
    a_computer.run_instructions();
    let a_output = a_computer.output.clone();

    stopwatch.print_elapsed();
    println!("Program output {:?}", a_output);
}

struct Computer {
    pub position: usize,
    pub input: Vec<i64>,
    pub input_position: usize,
    pub output: Vec<i64>,
    pub instruction_memory: Vec<i64>,
    pub running: bool,
    pub relative_base: i64,
}

impl Computer {
    pub const POSITION_MODE: i64 = 0;
    pub const IMMEDIATE_MODE: i64 = 1;
    pub const RELATIVE_MODE: i64 = 2;
    pub fn new(instruction_memory: Vec<i64>) -> Computer {
        Computer {
            position: 0,
            input: vec![],
            input_position: 0,
            output: vec![],
            instruction_memory: instruction_memory.clone(),
            running: true,
            relative_base: 0,
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
                    self.set_value_with_mode(&parameter_mode_vec, 3, added);
                    self.position += 4;
                }
                2 => {
                    let part1: i64 = self.get_part_value(&parameter_mode_vec, 1);
                    let part2: i64 = self.get_part_value(&parameter_mode_vec, 2);

                    let added = part1 * part2;
                    self.set_value_with_mode(&parameter_mode_vec, 3, added);
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
                    let input_value = *self.input.get(self.input_position).unwrap();
                    self.set_value_with_mode(&parameter_mode_vec, 1, input_value);
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
                    self.set_value_with_mode(
                        &parameter_mode_vec,
                        3,
                        if part1 < part2 { 1 } else { 0 },
                    );
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
                    self.set_value_with_mode(
                        &parameter_mode_vec,
                        3,
                        if part1 == part2 { 1 } else { 0 },
                    );
                    self.position += 4;
                }
                9 => {
                    let part1: i64 = self.get_part_value(&parameter_mode_vec, 1);
                    self.relative_base += part1;
                    self.position += 2;
                }
                99 => {
                    self.running = false;
                    println!("Program completed");
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

    fn get_part_value(&mut self, parameter_mode_vec: &Vec<i64>, part_position: usize) -> i64 {
        let part_mode = *parameter_mode_vec
            .get(part_position - 1)
            .unwrap_or(&Computer::POSITION_MODE);
        match part_mode {
            Computer::POSITION_MODE => {
                let part_index = self.get_value_from_memory(self.position + part_position);
                self.get_value_from_memory(part_index as usize)
            }
            Computer::IMMEDIATE_MODE => self.get_value_from_memory(self.position + part_position),
            Computer::RELATIVE_MODE => {
                let part_index: i64 = self.get_value_from_memory(self.position + part_position);
                self.get_value_from_memory((self.relative_base + part_index) as usize)
            }
            _ => {
                println!("Error! Instruction {} invalid mode", self.position);
                0
            }
        }
    }

    fn set_value_with_mode(
        &mut self,
        parameter_mode_vec: &Vec<i64>,
        part_position: usize,
        input_value: i64,
    ) {
        let part_mode = *parameter_mode_vec
            .get(part_position - 1)
            .unwrap_or(&Computer::POSITION_MODE);
        let store_at_address = self.get_value_from_memory(self.position + part_position);
        match part_mode {
            Computer::RELATIVE_MODE => {
                self.set_value_in_memory(
                    (self.relative_base + store_at_address) as usize,
                    input_value,
                );
            }
            _ => {
                self.set_value_in_memory(store_at_address as usize, input_value);
            }
        }
    }

    fn get_value_from_memory(&mut self, position: usize) -> i64 {
        self.expand_memory(position);
        self.instruction_memory[position]
    }
    fn set_value_in_memory(&mut self, position: usize, value: i64) {
        self.expand_memory(position);
        self.instruction_memory[position] = value;
    }
    fn expand_memory(&mut self, position: usize) {
        while self.instruction_memory.len() <= position {
            self.instruction_memory.push(0);
        }
    }
}

fn main() {
    println!("Part 1");
    part_1();
}
