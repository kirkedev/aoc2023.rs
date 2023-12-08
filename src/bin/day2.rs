use aoc::day2::{part1, part2};
use aoc::day2::game::Game;
use aoc::get_input;

fn main() {
    let games: Vec<Game> = get_input("day2")
        .map_while(Result::ok)
        .map(|line| line.parse::<Game>())
        .map_while(Result::ok)
        .collect();

    println!("Day 2");
    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
}
