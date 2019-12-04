mod utility;
use std::collections::HashMap;
use utility::stopwatch::stopwatch::*;

fn part_2() {
    let stopwatch = Stopwatch::start_new();

    let minimum = 171309;
    let maximum = 643603;

    let numbers: Vec<Vec<i32>> = (minimum..maximum + 1)
        .map(|num| {
            num.to_string()
                .chars()
                .map(|char| (char.to_digit(10).unwrap_or(0)) as i32)
                .collect()
        })
        .filter(|my_vec| {
            let mut decreases = false;
            let mut multiple: HashMap<i32, i32> = HashMap::new();
            let mut last_n = -1;
            for &n in my_vec {
                if n < last_n {
                    decreases = true;
                    break;
                }
                *multiple.entry(n).or_insert(0) += 1;
                last_n = n;
            }
            let mut same = false;
            for (m1, m2) in multiple {
                if m2 == 2 {
                    same = true;
                }
            }
            return !decreases && same;
        })
        .collect();

    stopwatch.print_elapsed();
    println!("Numbers: {}", numbers.len());
    println!("First number: {:?}", numbers.first().unwrap());
    println!("Last number: {:?}", numbers.last().unwrap());
}

fn part_1() {
    let stopwatch = Stopwatch::start_new();

    let minimum = 171309;
    let maximum = 643603;

    let numbers: Vec<Vec<i32>> = (minimum..maximum + 1)
        .map(|num| {
            num.to_string()
                .chars()
                .map(|char| (char.to_digit(10).unwrap_or(0)) as i32)
                .collect()
        })
        .filter(|my_vec| {
            let mut decreases = false;
            let mut contains_same = false;
            let mut last_n = -1;
            for &n in my_vec {
                if n < last_n {
                    decreases = true;
                    break;
                }
                if last_n == n {
                    contains_same = true;
                }
                last_n = n;
            }
            return !decreases && contains_same;
        })
        .collect();

    stopwatch.print_elapsed();
    println!("Numbers: {}", numbers.len());
    println!("First number: {:?}", numbers.first().unwrap());
    println!("Last number: {:?}", numbers.last().unwrap());
}

fn main() {
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
}
