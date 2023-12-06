use aoc::day1::{calibrate, get_numbers, get_text_numbers};
use aoc::get_input;

fn main() {
    println!("Day 1");

    let part1 = get_input("day1")
        .map(Result::unwrap)
        .map(get_numbers)
        .filter_map(calibrate)
        .sum::<u32>();

    println!("Part 1: {}", part1);

    let part2 = get_input("day1")
        .map(Result::unwrap)
        .map(get_text_numbers)
        .filter_map(calibrate)
        .sum::<u32>();

    println!("Part 2: {}", part2);
}
