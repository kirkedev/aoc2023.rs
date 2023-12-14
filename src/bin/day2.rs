use anyhow::Result;

use aoc::day2::{part1, part2};
use aoc::day2::game::Game;
use aoc::get_input;

fn main() -> Result<()> {
    let games: Vec<Game> = get_input("day2")?
        .into_iter()
        .map(|line| line.parse::<Game>())
        .collect::<Result<Vec<Game>>>()?;

    println!("Day 2");
    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));

    Ok(())
}
