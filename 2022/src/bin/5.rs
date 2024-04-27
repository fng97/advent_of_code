use std::{env, vec};

mod helpers;

fn range_contains_range(range1: (u32, u32), range2: (u32, u32)) -> bool {
    let lower_bound1 = range1.0;
    let upper_bound1 = range1.1;
    let lower_bound2 = range2.0;
    let upper_bound2 = range2.1;
    lower_bound1 <= lower_bound2 && upper_bound1 >= upper_bound2
}

fn get_range_pairs_from_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let (range1, range2) = line.split_once(',').unwrap();
    let (lower_bound1, upper_bound1) = range1.split_once('-').unwrap();
    let (lower_bound2, upper_bound2) = range2.split_once('-').unwrap();

    (
        (
            lower_bound1.parse::<u32>().unwrap(),
            upper_bound1.parse::<u32>().unwrap(),
        ),
        (
            lower_bound2.parse::<u32>().unwrap(),
            upper_bound2.parse::<u32>().unwrap(),
        ),
    )
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

    let mut stacks = vec![
        vec!['B', 'G', 'S', 'C'],
        vec!['T', 'M', 'W', 'H', 'J', 'N', 'V', 'G'],
        vec!['M', 'Q', 'S'],
        vec!['B', 'S', 'L', 'T', 'W', 'N', 'M'],
        vec!['J', 'Z', 'F', 'T', 'V', 'G', 'W', 'P'],
        vec!['C', 'T', 'B', 'G', 'Q', 'H', 'S'],
        vec!['T', 'J', 'P', 'B', 'W'],
        vec!['G', 'D', 'C', 'Z', 'F', 'T', 'Q', 'M'],
        vec!['N', 'S', 'H', 'B', 'P', 'F'],
    ];

    helpers::print_solution(&day, 1, 0);
    helpers::print_solution(&day, 2, 0);
}
