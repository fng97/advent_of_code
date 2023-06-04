use std::fs;

pub fn get_input(day: &str) -> String {
    let error_message = format!("Unable to find input file for {}.", day);
    fs::read_to_string(format!("inputs/{day}.txt")).expect(&error_message)
}

pub fn print_solution(day: &str, part: u8, solution: u32) {
    println!("Day {} - Part {}: {}", day, part, solution);
}
