mod utility;
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

fn part_2() {
    let input = load_file("resources/day_2_input.txt");
    let stopwatch = Stopwatch::start_new();
    let original_ids = split_by_into_numbers(input, ",".parse().unwrap());
    let mut ids;
    let target_output = 19690720;

    let mut closest_result = -1;
    let mut closest_value1 = -1;
    let mut closest_value2 = -1;
    let mut value1 = 0;
    let value1max = 99;
    let mut value2 = 0;
    let value2max = 99;
    loop {
        ids = original_ids.clone();
        ids[1] = value1;
        ids[2] = value2;
        run_instructions(ids.as_mut());
        if closest_result == -1
            || (ids[0] - target_output).abs() < (closest_result - target_output).abs()
        {
            closest_result = ids[0];
            closest_value1 = value1;
            closest_value2 = value2;
            if (ids[0] - target_output).abs() == 0 {
                break;
            }
        }
        value1 += 1;
        if value1 > value1max {
            value2 += 1;
            value1 = 0;
            if value2 > value2max {
                break;
            }
        }
    }
    stopwatch.print_elapsed();
    println!("Distance: {}", closest_result - target_output);
    println!("Closest Result: {}", closest_result);
    println!("Closest value 1: {}", closest_value1);
    println!("Closest value 2: {}", closest_value2);
}

fn part_1() {
    let input = load_file("resources/day_2_input.txt");
    let stopwatch = Stopwatch::start_new();
    let mut ids = split_by_into_numbers(input, ",".parse().unwrap());
    run_instructions(ids.as_mut());
    stopwatch.print_elapsed();
    println!("Result: {}", ids[0]);
}

fn run_instructions(ids: &mut Vec<i64>) {
    let mut position = 0;
    loop {
        let instruction = ids[position];
        match instruction {
            1 => {
                let part1index = ids[position + 1] as usize;
                let part2index = ids[position + 2] as usize;
                let part1 = ids[part1index];
                let part2 = ids[part2index];
                let added = part1 + part2;
                let part3 = ids[position + 3] as usize;
                ids[part3] = added;
                position += 4;
            }
            2 => {
                let part1index = ids[position + 1] as usize;
                let part2index = ids[position + 2] as usize;
                let part1 = ids[part1index];
                let part2 = ids[part2index];
                let added = part1 * part2;
                let part3 = ids[position + 3] as usize;
                ids[part3] = added;
                position += 4;
            }
            _ => {
                break;
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
