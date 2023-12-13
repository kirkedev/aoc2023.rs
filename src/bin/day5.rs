use anyhow::Result;

use aoc::day5::part1;
use aoc::get_input;

fn main() -> Result<()> {
    let input: Vec<String> = get_input("day5")
        .map_while(Result::ok)
        .collect();

    println!("Day 5");
    println!("Part 1: {}", part1(input)?);
    Ok(())
}
