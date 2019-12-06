mod utility;
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

const POSITION_MODE: i64 = 0;
const IMMEDIATE_MODE: i64 = 1;

fn part_2() {
    let input = load_file("resources/day_5_input.txt");
    let stopwatch = Stopwatch::start_new();
    let mut ids = split_by_into_numbers(input, ",".parse().unwrap());
    run_instructions(ids.as_mut(), 5);
    stopwatch.print_elapsed();
}

fn part_1() {
    let input = load_file("resources/day_5_input.txt");
    let stopwatch = Stopwatch::start_new();
    let mut ids = split_by_into_numbers(input, ",".parse().unwrap());
    run_instructions(ids.as_mut(), 1);
    stopwatch.print_elapsed();
}

fn run_instructions(ids: &mut Vec<i64>, input: i64) {
    let mut position = 0;
    loop {
        let full_instruction = ids[position];
        let mut full_instruction_vec: Vec<i64> = full_instruction
            .to_string()
            .chars()
            .map(|char| (char.to_digit(10).unwrap_or(0)) as i64)
            .collect();
        let instruction_piece_right = full_instruction_vec.pop().unwrap_or(0);
        let instruction_piece_left = full_instruction_vec.pop().unwrap_or(0);
        let instruction: i64 = (instruction_piece_left.to_string()
            + instruction_piece_right.to_string().as_ref())
        .parse()
        .unwrap();
        //full_instruction_vec now has [0] = parameter mode 1, [1] parameter mode 2, ect
        full_instruction_vec.reverse();

        match instruction {
            1 => {
                let part1: i64 = get_part_value(&full_instruction_vec, ids, position, 1);
                let part2: i64 = get_part_value(&full_instruction_vec, ids, position, 2);

                let added = part1 + part2;
                let part3 = ids[position + 3] as usize;
                ids[part3] = added;
                position += 4;
            }
            2 => {
                let part1: i64 = get_part_value(&full_instruction_vec, ids, position, 1);
                let part2: i64 = get_part_value(&full_instruction_vec, ids, position, 2);

                let added = part1 * part2;
                let part3 = ids[position + 3] as usize;
                ids[part3] = added;
                position += 4;
            }
            3 => {
                /*
                Opcode 3 takes a single integer as input and saves it to the address given by its only parameter.
                For example, the instruction 3,50 would take an input value and store it at address 50.
                */
                let store_at_address = ids[position + 1] as usize;
                ids[store_at_address] = input;
                position += 2;
            }
            4 => {
                /*
                Opcode 4 outputs the value of its only parameter.
                For example, the instruction 4,50 would output the value at address 50.
                */
                let part1: i64 = get_part_value(&full_instruction_vec, ids, position, 1);
                println!("Output: {}", part1);
                position += 2;
            }
            5 => {
                /*
                Opcode 5 is jump-if-true: if the first parameter is non-zero,
                it sets the instruction pointer to the value from the second
                parameter. Otherwise, it does nothing.
                */
                let part1: i64 = get_part_value(&full_instruction_vec, ids, position, 1);
                if part1 != 0 {
                    let part2: i64 = get_part_value(&full_instruction_vec, ids, position, 2);
                    position = part2 as usize;
                } else {
                    position += 3;
                }
            }
            6 => {
                /*
                Opcode 6 is jump-if-false: if the first parameter is zero,
                it sets the instruction pointer to the value from the second
                parameter. Otherwise, it does nothing.
                */
                let part1: i64 = get_part_value(&full_instruction_vec, ids, position, 1);
                if part1 == 0 {
                    let part2: i64 = get_part_value(&full_instruction_vec, ids, position, 2);
                    position = part2 as usize;
                } else {
                    position += 3;
                }
            }
            7 => {
                /*
                Opcode 7 is less than: if the first parameter is less than the
                second parameter, it stores 1 in the position given by the third
                parameter. Otherwise, it stores 0.
                */
                let part1: i64 = get_part_value(&full_instruction_vec, ids, position, 1);
                let part2: i64 = get_part_value(&full_instruction_vec, ids, position, 2);
                let part3 = ids[position + 3] as usize;
                ids[part3] = if part1 < part2 { 1 } else { 0 };
                position += 4;
            }
            8 => {
                /*
                Opcode 8 is equals: if the first parameter is equal to the second
                parameter, it stores 1 in the position given by the third parameter.
                Otherwise, it stores 0.
                */
                let part1: i64 = get_part_value(&full_instruction_vec, ids, position, 1);
                let part2: i64 = get_part_value(&full_instruction_vec, ids, position, 2);
                let part3 = ids[position + 3] as usize;
                ids[part3] = if part1 == part2 { 1 } else { 0 };
                position += 4;
            }
            _ => {
                break;
            }
        }
    }
}

fn get_part_value(
    full_instruction_vec: &Vec<i64>,
    ids: &mut Vec<i64>,
    instruction_position: usize,
    part_position: usize,
) -> i64 {
    let part_mode = *full_instruction_vec
        .get(part_position - 1)
        .unwrap_or(&POSITION_MODE);
    match part_mode {
        POSITION_MODE => {
            let part_index = ids[instruction_position + part_position] as usize;
            ids[part_index]
        }
        IMMEDIATE_MODE => ids[instruction_position + part_position],
        _ => {
            println!("Error! Instruction {} invalid mode", instruction_position);
            0
        }
    }
}

fn main() {
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
}
