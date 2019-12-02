#![allow(dead_code)]
pub mod stopwatch;
pub mod utility {
    use std::fs;

    pub fn load_file(location: &str) -> String {
        return match fs::read_to_string(location) {
            Ok(string) => string,
            _ => {
                eprint!("Failed to load {}", location);
                String::from("")
            }
        };
    }

    pub fn split_by(input: String, split: String) -> Vec<String> {
        input
            .split(split.as_str())
            .map(|string: &str| string.parse().unwrap())
            .collect::<Vec<String>>()
    }

    pub fn split_by_into_numbers(input: String, split: String) -> Vec<i64> {
        input
            .split(split.as_str())
            .map(|string: &str| string.parse().unwrap())
            .collect::<Vec<i64>>()
    }

    pub fn split_by_new_line(input: String) -> Vec<String> {
        input
            .lines()
            .map(|string: &str| String::from(string))
            .collect::<Vec<String>>()
    }

    pub fn split_by_new_line_as_char_vector(input: String) -> Vec<Vec<char>> {
        input
            .lines()
            .map(|string: &str| String::from(string.trim()).chars().collect())
            .collect::<Vec<Vec<char>>>()
    }

    pub fn split_by_new_line_integer(input: String) -> Vec<i64> {
        input
            .lines()
            .map(|string: &str| match string.parse() {
                Ok(number) => number,
                _ => 0,
            })
            .collect::<Vec<i64>>()
    }
}
