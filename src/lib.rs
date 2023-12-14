use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub fn get_input(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(format!("input/{filename}"))
        .expect("Unable to open file: {filename}");

    return BufReader::new(file).lines();
}
