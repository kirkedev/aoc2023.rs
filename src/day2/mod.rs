use crate::day2::game::{Cubes, Game};

pub mod game;

fn validate(max: Cubes) -> impl Fn(&&Game) -> bool {
    move |game| game.rounds.iter().all(|round|
        round.0 <= max.0 && round.1 <= max.1 && round.2 <= max.2)
}

fn get_minimums(game: &Game) -> Cubes {
    let red = game.rounds.iter()
        .max_by_key(|round| round.0)
        .map_or(0, |round| round.0);

    let green = game.rounds.iter()
        .max_by_key(|round| round.1)
        .map_or(0, |round| round.1);

    let blue = game.rounds.iter()
        .max_by_key(|round| round.2)
        .map_or(0, |round| round.2);

    return Cubes(red, green, blue);
}

fn cube(colors: Cubes) -> u32 {
    colors.0 * colors.1 * colors.2
}

pub fn part1(games: &Vec<Game>) -> u32 {
    games.iter()
        .filter(validate(Cubes(12, 13, 14)))
        .map(|game| game.id)
        .sum::<u32>()
}

pub fn part2(games: &Vec<Game>) -> u32 {
    games.iter()
        .map(get_minimums)
        .map(cube)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::day2::{cube, get_minimums, part1, part2, validate};
    use crate::day2::game::{Cubes, Game};

    #[test]
    pub fn test_validate() {
        let is_valid = validate(Cubes(12, 13, 14));

        let mut game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse::<Game>().unwrap();
        assert_eq!(is_valid(&&game), true);

        game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".parse::<Game>().unwrap();
        assert_eq!(is_valid(&&game), true);

        game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".parse::<Game>().unwrap();
        assert_eq!(is_valid(&&game), false);

        game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".parse::<Game>().unwrap();
        assert_eq!(is_valid(&&game), false);

        game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".parse::<Game>().unwrap();
        assert_eq!(is_valid(&&game), true);
    }

    #[test]
    pub fn test_get_minimums() {
        let mut game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse::<Game>().unwrap();
        assert_eq!(get_minimums(&game), Cubes(4, 2, 6));

        game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".parse::<Game>().unwrap();
        assert_eq!(get_minimums(&game), Cubes(1, 3, 4));

        game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".parse::<Game>().unwrap();
        assert_eq!(get_minimums(&game), Cubes(20, 13, 6));

        game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".parse::<Game>().unwrap();
        assert_eq!(get_minimums(&game), Cubes(14, 3, 15));

        game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".parse::<Game>().unwrap();
        assert_eq!(get_minimums(&game), Cubes(6, 3, 2));
    }

    #[test]
    pub fn test_cube() {
        assert_eq!(cube(Cubes(4, 2, 6)), 48);
        assert_eq!(cube(Cubes(1, 3, 4)), 12);
        assert_eq!(cube(Cubes(20, 13, 6)), 1560);
        assert_eq!(cube(Cubes(14, 3, 15)), 630);
        assert_eq!(cube(Cubes(6, 3, 2)), 36);
    }

    #[test]
    pub fn test_part_1() {
        let games = vec![
            Game { id: 1, rounds: vec![Cubes(4, 0, 3), Cubes(1, 2, 6), Cubes(0, 2, 0)] },
            Game { id: 2, rounds: vec![Cubes(0, 2, 1), Cubes(1, 3, 4), Cubes(0, 1, 1)] },
            Game { id: 3, rounds: vec![Cubes(20, 8, 6), Cubes(4, 13, 0), Cubes(1, 5, 0)] },
            Game { id: 4, rounds: vec![Cubes(3, 1, 6), Cubes(6, 3, 0), Cubes(14, 3, 15)] },
            Game { id: 5, rounds: vec![Cubes(6, 3, 1), Cubes(1, 2, 2)] },
        ];

        assert_eq!(part1(&games), 8);
    }

    #[test]
    pub fn test_part_2() {
        let games = vec![
            Game { id: 1, rounds: vec![Cubes(4, 0, 3), Cubes(1, 2, 6), Cubes(0, 2, 0)] },
            Game { id: 2, rounds: vec![Cubes(0, 2, 1), Cubes(1, 3, 4), Cubes(0, 1, 1)] },
            Game { id: 3, rounds: vec![Cubes(20, 8, 6), Cubes(4, 13, 0), Cubes(1, 5, 0)] },
            Game { id: 4, rounds: vec![Cubes(3, 1, 6), Cubes(6, 3, 0), Cubes(14, 3, 15)] },
            Game { id: 5, rounds: vec![Cubes(6, 3, 1), Cubes(1, 2, 2)] },
        ];

        assert_eq!(part2(&games), 2286);
    }
}
