mod utility;
use std::collections::HashMap;
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

fn part_2() {
    let input = load_file("resources/day_2_input.txt");
    let stopwatch = Stopwatch::start_new();
    let original_ids = split_by_into_numbers(input, ",".parse().unwrap());
    let mut ids;
    let targetOutput = 19690720;

    let mut closestResult = -1;
    let mut closestValue1 = -1;
    let mut closestValue2 = -1;
    let mut value1 = 0;
    let value1Max = 99;
    let mut value2 = 0;
    let value2Max = 99;
    loop {
        ids = original_ids.clone();
        ids[1] = value1;
        ids[2] = value2;
        let mut position = 0;
        'outer: loop {
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
                    break 'outer;
                }
            }
        }
        if closestResult == -1
            || (ids[0] - targetOutput).abs() < (closestResult - targetOutput).abs()
        {
            closestResult = ids[0];
            closestValue1 = value1;
            closestValue2 = value2;
        }
        value1 += 1;
        if value1 > value1Max {
            value2 += 1;
            value1 = 0;
            if value2 > value2Max {
                break;
            }
        }
    }
    stopwatch.print_elapsed();
    println!("Distance: {}", closestResult - targetOutput);
    println!("Closest Result: {}", closestResult);
    println!("Closest value 1: {}", closestValue1);
    println!("Closest value 2: {}", closestValue2);
}

fn part_1() {
    let input = load_file("resources/day_2_input.txt");
    let stopwatch = Stopwatch::start_new();
    let mut ids = split_by_into_numbers(input, ",".parse().unwrap());

    let mut position = 0;
    'outer: loop {
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
                break 'outer;
            }
        }
    }
    stopwatch.print_elapsed();
    println!("Result: {}", ids[0]);
}

fn main() {
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
}
