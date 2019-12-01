pub mod utility;
use std::collections::HashMap;
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

fn part_2() {
    let input = load_file("resources/day_1_input.txt");
    let stopwatch = Stopwatch::start_new();

    let numbers = split_by_new_line_integer(input);
    let mut previous_results: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;
    previous_results.insert(result, 1);
    'fullLoop: loop {
        for number in &numbers {
            result += *number;
            let result_amount = match previous_results.get(&result) {
                Some(amount) => {
                    let new_amount = amount + 1;
                    previous_results.insert(result, new_amount);
                    new_amount
                }
                _ => {
                    previous_results.insert(result, 1);
                    1
                }
            };
            if result_amount >= 2 {
                break 'fullLoop;
            }
        }
    }

    stopwatch.print_elapsed();
    println!("Result: {}", result);
}

fn part_1() {
    let input = load_file("resources/day_1_input.txt");
    let stopwatch = Stopwatch::start_new();
    let numbers = split_by_new_line_integer(input);
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    println!("Result: {}", result);

    stopwatch.print_elapsed();
}

fn main() {
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
}
