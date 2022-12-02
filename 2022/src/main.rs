use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use config::{Config, ConfigError};

mod day01;

fn main() -> std::io::Result<()> {
    let days = vec![day01::AocDay01::preprocessing];

    let inputs_dir: String = get_settings().unwrap().get("input_dir").unwrap();

    let puzzle_index: usize = env::args()
        .skip(1)
        .next()
        .expect("Please provide a day number")
        .parse()
        .expect("Puzzle day must be a number");

    let input_file = format!("{inputs_dir}/day{puzzle_index:0>2}.txt");
    let input = read_input_file(input_file);

    let puzzle = days.get(puzzle_index - 1).unwrap()(input);
    println!("Part 1: {}", puzzle.part1());
    println!("Part 2: {}", puzzle.part2());

    Ok(())
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
