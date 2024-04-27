use std::env;

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

    let mut total_envelopped_pairs: u32 = 0;
    let mut total_overlapping_pairs: u32 = 0;

    // part one

    for range_pair in input.lines() {
        let (range1, range2) = get_range_pairs_from_line(range_pair);
        if range_contains_range(range1, range2) || range_contains_range(range2, range1) {
            total_envelopped_pairs += 1;
        }
    }

    // part two

    for range_pair in input.lines() {
        let ((lower_bound1, upper_bound1), (lower_bound2, upper_bound2)) =
            get_range_pairs_from_line(range_pair);

        let range1 = lower_bound1..=upper_bound1;

        for i in lower_bound2..=upper_bound2 {
            if range1.contains(&i) {
                total_overlapping_pairs += 1;
                break;
            }
        }
    }

    helpers::print_solution(&day, 1, total_envelopped_pairs);
    helpers::print_solution(&day, 2, total_overlapping_pairs);
}
