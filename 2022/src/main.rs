use core::fmt;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use config::{Config, ConfigError};

use crate::day01::AocDay01;
use crate::day02::AocDay02;
use crate::day03::AocDay03;
use crate::day04::AocDay04;
use crate::day05::AocDay05;
use crate::day06::AocDay06;
use crate::day07::AocDay07;
use crate::day08::AocDay08;
use crate::day09::AocDay09;
use crate::day10::AocDay10;
use crate::day11::AocDay11;
use crate::day12::AocDay12;
use crate::day13::AocDay13;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

fn main() {
    let inputs_dir: String = get_settings().unwrap().get("inputs_dir").unwrap();

    let puzzle_index: usize = env::args()
        .nth(1)
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
        5 => solve(AocDay05::preprocessing(input)),
        6 => solve(AocDay06::preprocessing(input)),
        7 => solve(AocDay07::preprocessing(input)),
        8 => solve(AocDay08::preprocessing(input)),
        9 => solve(AocDay09::preprocessing(input)),
        10 => solve(AocDay10::preprocessing(input)),
        11 => solve(AocDay11::preprocessing(input)),
        12 => solve(AocDay12::preprocessing(input)),
        13 => solve(AocDay13::preprocessing(input)),
        _ => unimplemented!("No such puzzle"),
    };
}

pub trait AocDay {
    type R1: fmt::Display;
    type R2: fmt::Display;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self;
    fn part1(&self) -> Self::R1;
    fn part2(&self) -> Self::R2;
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
