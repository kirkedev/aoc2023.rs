use anyhow::Result;

use aoc::day5::{part1, part2};
use aoc::get_input;

fn main() -> Result<()> {
    let input: Vec<String> = get_input("day5")?;

    println!("Day 5");
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);

    Ok(())
}
