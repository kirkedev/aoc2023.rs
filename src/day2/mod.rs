use crate::day2::game::{Colors, Game};

pub mod game;

pub fn validate(red: u32, green: u32, blue: u32) -> impl Fn(&&Game) -> bool {
    move |game| game.rounds.iter().all(|round|
        round.0 <= red && round.1 <= green && round.2 <= blue)
}

pub fn get_minimums(game: &Game) -> Colors {
    let red = game.rounds.iter()
        .max_by_key(|round| round.0)
        .map_or(0, |round| round.0);

    let green = game.rounds.iter()
        .max_by_key(|round| round.1)
        .map_or(0, |round| round.1);

    let blue = game.rounds.iter()
        .max_by_key(|round| round.2)
        .map_or(0, |round| round.2);

    return Colors(red, green, blue);
}

pub fn cube(colors: Colors) -> u32 {
    colors.0 * colors.1 * colors.2
}

#[cfg(test)]
mod tests {
    use crate::day2::{cube, get_minimums, validate};
    use crate::day2::game::{Colors, Game};

    #[test]
    pub fn test_validate() {
        let is_valid = validate(12, 13, 14);

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
        assert_eq!(get_minimums(&game), Colors(4, 2, 6));

        game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".parse::<Game>().unwrap();
        assert_eq!(get_minimums(&game), Colors(1, 3, 4));

        game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".parse::<Game>().unwrap();
        assert_eq!(get_minimums(&game), Colors(20, 13, 6));

        game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".parse::<Game>().unwrap();
        assert_eq!(get_minimums(&game), Colors(14, 3, 15));

        game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".parse::<Game>().unwrap();
        assert_eq!(get_minimums(&game), Colors(6, 3, 2));
    }

    #[test]
    pub fn test_cube() {
        assert_eq!(cube(Colors(4, 2, 6)), 48);
        assert_eq!(cube(Colors(1, 3, 4)), 12);
        assert_eq!(cube(Colors(20, 13, 6)), 1560);
        assert_eq!(cube(Colors(14, 3, 15)), 630);
        assert_eq!(cube(Colors(6, 3, 2)), 36);
    }
}
