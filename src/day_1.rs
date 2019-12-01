pub mod utility;
use std::collections::HashMap;
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

fn part_1() {
    let input = load_file("resources/day_1_input.txt");
    let stopwatch = Stopwatch::start_new();
    let numbers = split_by_new_line_integer(input);
    let mut fuel: i64 = 0;
    for number in numbers {
        fuel += get_fuel(number);
    }
    println!("Result: {}", fuel);

    stopwatch.print_elapsed();
}

fn part_2() {
    let input = load_file("resources/day_1_input.txt");
    let stopwatch = Stopwatch::start_new();
    let numbers = split_by_new_line_integer(input);
    let mut fuel: i64 = 0;
    for number in numbers {
        let mut next_fuel: i64 = number;
        let mut single_fuel: i64 = 0;
        while next_fuel > 0 {
            next_fuel = get_fuel(next_fuel);
            single_fuel += next_fuel;
        }
        fuel += single_fuel;
    }
    println!("Result: {}", fuel);

    stopwatch.print_elapsed();
}

fn get_fuel(number: i64) -> i64 {
    let fuel = (number as f64 / 3.0).floor() as i64 - 2;
    if fuel > 0 {
        fuel
    } else {
        0
    }
}

fn main() {
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
}
