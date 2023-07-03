use std::env;

mod helpers;

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn points(&self) -> u8 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn vs(&self, other: &Shape) -> Outcome {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Loss,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Paper => match other {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Loss,
            },
            Shape::Scissors => match other {
                Shape::Rock => Outcome::Loss,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw,
            },
        }
    }

    fn points(&self) -> u8 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn shape_from_letter(letter: char) -> Shape {
    match letter {
        'A' | 'X' => Shape::Rock,
        'B' | 'Y' => Shape::Paper,
        'C' | 'Z' => Shape::Scissors,
        _ => panic!("Invalid shape letter: {}", letter),
    }
}

fn outcome_from_letter(letter: char) -> Outcome {
    match letter {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Invalid outcome letter: {}", letter),
    }
}

fn char_from_string(string: &str) -> char {
    string.chars().next().expect("Expected a single character.")
}

fn letter_pairs_from_input(input: &str) -> Vec<(char, char)> {
    let mut shape_pairs: Vec<(char, char)> = Vec::new();

    for line in input.lines() {
        let (letter1, letter2) = line
            .split_once(' ')
            .map(|(letter1, letter2)| (char_from_string(letter1), char_from_string(letter2)))
            .expect("Expected line to be two letters separated by a space.");

        shape_pairs.push((letter1, letter2));
    }

    shape_pairs
}

fn shape_for_outcome(opponent_shape: &Shape, desired_outcome: &Outcome) -> Shape {
    match desired_outcome {
        Outcome::Win => match opponent_shape {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        Outcome::Loss => match opponent_shape {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        Outcome::Draw => match opponent_shape {
            Shape::Rock => Shape::Rock,
            Shape::Paper => Shape::Paper,
            Shape::Scissors => Shape::Scissors,
        },
    }
}

// TODO: Refactor to use map instead of for loop.
fn part_one(input: &str) -> u32 {
    let mut total_score: u32 = 0;

    for (letter1, letter2) in letter_pairs_from_input(input) {
        let my_shape = shape_from_letter(letter2);
        let their_shape = shape_from_letter(letter1);

        total_score += my_shape.points() as u32;
        total_score += my_shape.vs(&their_shape).points() as u32;
    }

    total_score
}

fn part_two(input: &str) -> u32 {
    let mut total_score = 0;

    for (letter1, letter2) in letter_pairs_from_input(input) {
        let their_shape = shape_from_letter(letter1);
        let desired_outcome = outcome_from_letter(letter2);
        let my_shape = shape_for_outcome(&their_shape, &desired_outcome);

        total_score += my_shape.points() as u32;
        total_score += desired_outcome.points() as u32;
    }

    total_score
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
