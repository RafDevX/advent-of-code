use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use config::{Config, ConfigError};

use crate::day01::AocDay01;
use crate::day02::AocDay02;
use crate::day03::AocDay03;
use crate::day04::AocDay04;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let inputs_dir: String = get_settings().unwrap().get("inputs_dir").unwrap();

    let puzzle_index: usize = env::args()
        .skip(1)
        .next()
        .expect("Please provide a day number")
        .parse()
        .expect("Puzzle day must be a number");

    let input_file = format!("{inputs_dir}/day{puzzle_index:0>2}.txt");
    let input = read_input_file(input_file);

    match puzzle_index {
        1 => solve(AocDay01::preprocessing(input)),
        2 => solve(AocDay02::preprocessing(input)),
        3 => solve(AocDay03::preprocessing(input)),
        4 => solve(AocDay04::preprocessing(input)),
        _ => unimplemented!("No such puzzle"),
    };
}

pub trait AocDay {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self;
    fn part1(&self) -> i64;
    fn part2(&self) -> i64;
}

fn get_settings() -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
}

fn read_input_file(path: String) -> impl Iterator<Item = String> {
    BufReader::new(File::open(path).expect("Failed to open input file"))
        .lines()
        .map(|r| r.expect("I/O error while reading input"))
}

fn solve(puzzle: impl AocDay) {
    println!("Part 1: {}", puzzle.part1());
    println!("Part 2: {}", puzzle.part2());
}

#[cfg(test)]
mod tests {
    use crate::{get_settings, read_input_file, AocDay};

    pub fn setup_example<T: AocDay>(puzzle_index: usize) -> T {
        let examples_dir: String = get_settings().unwrap().get("examples_dir").unwrap();
        let example_file = format!("{examples_dir}/day{puzzle_index:0>2}.txt");
        let lines = read_input_file(example_file);
        T::preprocessing(lines)
    }
}
