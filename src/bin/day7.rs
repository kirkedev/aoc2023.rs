use anyhow::Result;

use aoc::day7::{part1, part2};
use aoc::get_input;

fn main() -> Result<()> {
    let input: Vec<String> = get_input("day7")
        .map_while(Result::ok)
        .collect();

    println!("Day 7");
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);

    Ok(())
}
