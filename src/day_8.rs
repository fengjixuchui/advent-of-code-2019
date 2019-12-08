mod utility;
use utility::stopwatch::stopwatch::*;
use utility::utility::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn part_2() {
    let input = load_file("resources/day_8_input.txt");
    let stopwatch = Stopwatch::start_new();

    let picture_width = WIDTH;
    let picture_height = HEIGHT;
    let layer_size = picture_height * picture_width;
    let input_vec: Vec<i64> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect();

    let mut layer_vec: Vec<Vec<i64>> = Vec::new();
    layer_vec.push(Vec::new());
    let mut current_count = 0;
    let mut current_layer = 0;
    for number in input_vec {
        if current_count >= layer_size {
            current_count = 0;
            current_layer += 1;
            layer_vec.push(Vec::new());
        }
        layer_vec[current_layer].push(number);
        current_count += 1;
    }

    // Merge layers into a single image
    layer_vec.reverse();
    let mut final_image: Vec<i64> = Vec::new();
    for i in (0..layer_size) {
        // 0 is black, 1 is white, 2 is transparent
        let mut pixel = -1;
        for layer in &layer_vec {
            let number = layer[i];
            if number == 0 || number == 1 {
                pixel = number;
            }
        }
        final_image.push(pixel);
    }

    // Render image
    let mut index = 0;
    for y in (0..picture_height) {
        for x in (0..picture_width) {
            match final_image[index] {
                0 => {
                    print!(" ");
                }
                1 => {
                    print!("█");
                }
                _ => {
                    print!("-");
                }
            }
            index = index + 1;
        }
        println!();
    }

    stopwatch.print_elapsed();
    println!("Done");
}

fn part_1() {
    let input = load_file("resources/day_8_input.txt");
    let stopwatch = Stopwatch::start_new();

    let picture_width = WIDTH;
    let picture_height = HEIGHT;
    let layer_size = picture_height * picture_width;
    let input_vec: Vec<i64> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect();

    let mut layer_vec: Vec<Vec<i64>> = Vec::new();
    layer_vec.push(Vec::new());
    let mut current_count = 0;
    let mut current_layer = 0;
    for number in input_vec {
        if current_count >= layer_size {
            current_count = 0;
            current_layer += 1;
            layer_vec.push(Vec::new());
        }
        layer_vec[current_layer].push(number);
        current_count += 1;
    }

    // Get vec with smallest number of 0s
    let mut vec_index_lowest_zeros: i64 = -1;
    let mut lowest_number_of_zeros: i64 = 0;
    let mut layer_index = 0;
    for layer in &layer_vec {
        let count: i64 = layer.iter().filter(|x| **x == 0).count() as i64;
        if count < lowest_number_of_zeros || vec_index_lowest_zeros == -1 {
            lowest_number_of_zeros = count;
            vec_index_lowest_zeros = layer_index;
        }
        layer_index += 1;
    }

    let count_one_digits = layer_vec[vec_index_lowest_zeros as usize]
        .iter()
        .filter(|x| **x == 1)
        .count();
    let count_two_digits = layer_vec[vec_index_lowest_zeros as usize]
        .iter()
        .filter(|x| **x == 2)
        .count();

    stopwatch.print_elapsed();
    println!("Result {}", count_one_digits * count_two_digits);
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
    fn run_instructions(&mut self) {}
}

fn main() {
    println!("Part 1");
    part_1();
    println!("Part 2");
    part_2();
}
