mod utility;
use std::collections::{HashMap, HashSet};
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

fn part_2() {
    let input = load_file("resources/day_3_input.txt");
    let stopwatch = Stopwatch::start_new();
    let lines = split_by_new_line(input);
    let mut grid: HashMap<i32, HashMap<i32, HashMap<i32, i32>>> = HashMap::new();
    let origin_x = 0;
    let origin_y = 0;

    let mut key = 0;
    for raw_line in lines {
        let line = split_by(raw_line, ",".parse().unwrap());
        draw_line2(&mut grid, line, key, origin_x as i32, origin_y as i32);
        key += 1;
    }

    // Find the closest intersection and return the distance from origin
    let mut closest_steps = -1;
    let mut closest_x: i32 = -1;
    let mut closest_y: i32 = -1;
    for (x, x_grid) in grid {
        for (y, ids) in x_grid {
            if ids.len() > 1 {
                let mut steps = 0;
                for (id, step) in ids {
                    steps += step;
                }
                if steps < closest_steps || closest_steps == -1 {
                    closest_x = x as i32;
                    closest_y = y as i32;
                    closest_steps = steps;
                }
            }
        }
    }

    stopwatch.print_elapsed();
    println!("Closest steps: {}", closest_steps);
    println!("Closest x: {}", closest_x);
    println!("Closest y: {}", closest_y);
}

fn draw_line2(
    grid: &mut HashMap<i32, HashMap<i32, HashMap<i32, i32>>>,
    line: Vec<String>,
    key: i32,
    origin_x: i32,
    origin_y: i32,
) {
    let mut steps = 0;
    // Line is (R/D/L/U)[Number] list
    let mut position_x = origin_x;
    let mut position_y = origin_y;
    for item in line {
        let instruction = &item[..1];
        let move_amount: i32 = item[1..].parse().unwrap();
        let mut x_direction = 0;
        let mut y_direction = 0;
        match instruction {
            "R" => {
                x_direction = 1;
            }
            "D" => {
                y_direction = -1;
            }
            "L" => {
                x_direction = -1;
            }
            "U" => {
                y_direction = 1;
            }
            _ => {
                println!("Error!");
            }
        }
        for _ in 0..move_amount {
            position_x += x_direction;
            position_y += y_direction;
            steps += 1;
            let grid_x = grid.entry(position_x).or_insert(Default::default());
            let ids = grid_x.entry(position_y).or_insert(Default::default());
            if !ids.contains_key(&key) {
                ids.insert(key, steps);
            }
        }
    }
}

fn part_1() {
    let input = load_file("resources/day_3_input.txt");
    let stopwatch = Stopwatch::start_new();
    let lines = split_by_new_line(input);
    let mut grid: HashMap<i32, HashMap<i32, HashSet<i32>>> = HashMap::new();
    let origin_x = 0;
    let origin_y = 0;

    let mut key = 0;
    for raw_line in lines {
        let line = split_by(raw_line, ",".parse().unwrap());
        draw_line(&mut grid, line, key, origin_x as i32, origin_y as i32);
        key += 1;
    }

    // Find the closest intersection and return the distance from origin
    let mut closest_distance = -1;
    let mut closest_x: i32 = -1;
    let mut closest_y: i32 = -1;
    for (x, x_grid) in grid {
        for (y, ids) in x_grid {
            if ids.len() > 1 {
                let distance =
                    (x as i32 - origin_x as i32).abs() + (y as i32 - origin_y as i32).abs();
                if distance < closest_distance || closest_distance == -1 {
                    closest_x = x as i32;
                    closest_y = y as i32;
                    closest_distance = distance;
                }
            }
        }
    }

    stopwatch.print_elapsed();
    println!("Closest distance: {}", closest_distance);
    println!("Closest x: {}", closest_x);
    println!("Closest y: {}", closest_y);
}

fn draw_line(
    grid: &mut HashMap<i32, HashMap<i32, HashSet<i32>>>,
    line: Vec<String>,
    key: i32,
    origin_x: i32,
    origin_y: i32,
) {
    // Line is (R/D/L/U)[Number] list
    let mut position_x = origin_x;
    let mut position_y = origin_y;
    for item in line {
        let instruction = &item[..1];
        let move_amount: i32 = item[1..].parse().unwrap();
        let mut x_direction = 0;
        let mut y_direction = 0;
        match instruction {
            "R" => {
                x_direction = 1;
            }
            "D" => {
                y_direction = -1;
            }
            "L" => {
                x_direction = -1;
            }
            "U" => {
                y_direction = 1;
            }
            _ => {
                println!("Error!");
            }
        }
        for _ in 0..move_amount {
            position_x += x_direction;
            position_y += y_direction;
            let grid_x = grid.entry(position_x).or_insert(Default::default());
            let ids = grid_x.entry(position_y).or_insert(Default::default());
            ids.insert(key);
        }
    }
}

fn main() {
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
}
