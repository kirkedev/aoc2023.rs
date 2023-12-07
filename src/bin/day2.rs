use aoc::day2::{cube, get_minimums, validate};
use aoc::day2::game::Game;
use aoc::get_input;

fn main() {
    let games: Vec<Game> = get_input("day2")
        .map_while(Result::ok)
        .map(|line| line.parse::<Game>())
        .map_while(Result::ok)
        .collect();

    let part1 = games.iter()
        .filter(validate(12, 13, 14))
        .map(|game| game.id)
        .sum::<u32>();

    let part2 = games.iter()
        .map(get_minimums)
        .map(cube)
        .sum::<u32>();

    println!("Day 2");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
