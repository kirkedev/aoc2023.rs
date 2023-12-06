use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};

pub mod day1;

pub fn get_input(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(format!("input/{filename}"))
        .expect("Unable to open file: {filename}");

    return BufReader::new(file).lines();
}
