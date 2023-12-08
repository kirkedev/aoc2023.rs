use std::fmt::Error;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Cubes(pub u32, pub u32, pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Cubes>,
}

fn parse_round(input: &str) -> Cubes {
    let cubes: Vec<(u32, &str)> = input.split(", ")
        .map_while(|cubes| cubes.split_once(' ')
            .and_then(|(count, color)| Some((count.parse::<u32>().ok()?, color))))
        .into_iter()
        .collect();

    let red = cubes.iter()
        .find(|(_, color)| *color == "red")
        .map_or(0, |(count, _)| *count);

    let green = cubes.iter()
        .find(|(_, color)| *color == "green")
        .map_or(0, |(count, _)| *count);

    let blue = cubes.iter()
        .find(|(_, color)| *color == "blue")
        .map_or(0, |(count, _)| *count);

    return Cubes(red, green, blue);
}

fn parse_rounds(input: &str) -> Vec<Cubes> {
    input.split("; ")
        .map(parse_round)
        .collect()
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.split_once(": ")
            .and_then(|(game, rounds)|
                game.split_once(' ')
                    .and_then(|(_, game)| game.parse::<u32>().ok())
                    .map(|id| Game {
                        id,
                        rounds: parse_rounds(rounds),
                    }))
            .ok_or(Error)
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::game::{Cubes, Game, parse_round};

    #[test]
    pub fn test_parse_game() {
        assert_eq!("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse::<Game>().unwrap(), Game {
            id: 1,
            rounds: vec![
                Cubes(4, 0, 3),
                Cubes(1, 2, 6),
                Cubes(0, 2, 0),
            ],
        });

        assert_eq!("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".parse::<Game>().unwrap(), Game {
            id: 2,
            rounds: vec![
                Cubes(0, 2, 1),
                Cubes(1, 3, 4),
                Cubes(0, 1, 1),
            ],
        });
    }

    #[test]
    #[should_panic]
    pub fn test_parse_invalid_game() {
        "Game 1".parse::<Game>().unwrap();
    }

    #[test]
    pub fn test_parse_round() {
        assert_eq!(parse_round("3 blue, 4 red"), Cubes(4, 0, 3));
        assert_eq!(parse_round("1 red, 2 green, 6 blue"), Cubes(1, 2, 6));
        assert_eq!(parse_round("2 green"), Cubes(0, 2, 0));
    }
}