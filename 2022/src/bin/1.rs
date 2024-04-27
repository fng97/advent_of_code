use std::env;

mod helpers;

fn snacks_calories_sums(input: &str) -> Vec<u32> {
    let mut sums = Vec::new();

    let mut calories;

    for snacks in input.split("\n\n") {
        calories = 0;
        for snack in snacks.lines() {
            calories += snack.trim().parse::<u32>().unwrap_or(0);
        }

        sums.push(calories);
    }

    sums.sort();

    sums
}

fn part_one(input: &str) -> u32 {
    let sums = snacks_calories_sums(input);
    sums.iter().rev().take(1).sum()
}

fn part_two(input: &str) -> u32 {
    let sums = snacks_calories_sums(input);
    sums.iter().rev().take(3).sum()
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

    helpers::print_solution(&day, 1, part_one(&input));
    helpers::print_solution(&day, 2, part_two(&input));
}
