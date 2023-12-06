use aoc::day1::{calibrate, get_numbers, get_text_numbers};
use aoc::get_input;

fn main() {
    println!("Day 1");

    let input: Vec<String> = get_input("day1")
        .map_while(Result::ok)
        .collect();

    let part1 = input.iter()
        .map(|string| get_numbers(string.as_str()))
        .map(calibrate)
        .sum::<u32>();

    println!("Part 1: {}", part1);

    let part2 = input.iter()
        .map(|string| get_text_numbers(string.as_str()))
        .map(calibrate)
        .sum::<u32>();

    println!("Part 2: {}", part2);
}
