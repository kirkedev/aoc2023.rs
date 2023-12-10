use aoc::day4::{part1, part2};
use aoc::get_input;

fn main() {
    let input: Vec<String> = get_input("day4")
        .map_while(Result::ok)
        .collect();

    println!("Day 4");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
