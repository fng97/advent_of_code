use std::env;

mod helpers;

fn get_value(c: char) -> u32 {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    if letters.contains(c) {
        return letters.chars().position(|x| x == c).unwrap() as u32 + 1;
    } else {
        0
    }
}

fn main() {
    let day = env::current_exe()
        .unwrap()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let input = helpers::get_input(&day);

    let mut sum_of_value_of_duplicate_items: u32 = 0;

    // part one

    for line in input.lines() {
        let (compartment1, compartment2) = line.split_at(line.len() / 2);
        for item in compartment1.chars() {
            if compartment2.contains(item) {
                sum_of_value_of_duplicate_items += get_value(item);
                break;
            }
        }
    }

    // part two

    let mut sum_of_badge_values: u32 = 0;

    for group in input.lines().collect::<Vec<&str>>().chunks(3) {
        for item in group[0].chars() {
            if group[1].contains(item) && group[2].contains(item) {
                sum_of_badge_values += get_value(item);
                break;
            }
        }
    }

    helpers::print_solution(&day, 1, sum_of_value_of_duplicate_items);
    helpers::print_solution(&day, 2, sum_of_badge_values);
}
