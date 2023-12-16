use anyhow::Result;

use aoc::day8::part1;
use aoc::get_input;

fn main() -> Result<()> {
    let input: Vec<String> = get_input("day8")?;

    println!("Day 8");
    println!("Part 1: {}", part1(&input)?);

    Ok(())
}
