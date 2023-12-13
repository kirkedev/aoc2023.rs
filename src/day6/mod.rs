use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

fn get_values(line: &str) -> Result<Vec<u64>> {
    let (_, values) = line.split_once(':')
        .context("Failed to locate values in {line}")?;

    values.split_ascii_whitespace()
        .map(|value| value.parse::<u64>()
            .map_err(|err| anyhow!(err)))
        .collect()
}

fn parse(input: &Vec<String>) -> Result<Vec<(u64, u64)>> {
    let (time, distance) = input.iter()
        .take(2)
        .map(|line| get_values(line.as_str()))
        .collect::<Result<Vec<Vec<u64>>>>()?
        .into_iter()
        .collect_tuple()
        .context("Failed to parse races")?;

    Ok(time.into_iter().zip(distance.into_iter()).collect())
}

fn is_winner(rate: u64, time: u64, distance: u64) -> bool {
    rate * (time - rate) > distance
}

pub fn part1(input: &Vec<String>) -> Result<u64> {
    let result = parse(&input)?.into_iter()
        .map(|(time, distance)|
            (1..=time)
                .filter(|rate| is_winner(*rate, time, distance))
                .count() as u64)
        .product();

    Ok(result)
}

pub fn part2(input: &Vec<String>) -> Result<u64> {
    let input = input.iter()
        .map(|line| line.split_whitespace().collect::<String>())
        .collect_vec();

    return part1(&input);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::day6::{parse, part1, part2};

    #[test]
    fn test_parse() {
        let input: Vec<String> = "Time:      7  15   30
Distance:  9  40  200
"
            .lines()
            .map(String::from)
            .collect_vec();

        assert_eq!(parse(&input).unwrap(), vec![(7, 9), (15, 40), (30, 200)]);
    }

    #[test]
    fn test_part1() {
        let input: Vec<String> = "Time:      7  15   30
Distance:  9  40  200
"
            .lines()
            .map(String::from)
            .collect_vec();

        assert_eq!(part1(&input).unwrap(), 288);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = "Time:      7  15   30
Distance:  9  40  200
"
            .lines()
            .map(String::from)
            .collect_vec();

        assert_eq!(part2(&input).unwrap(), 71503);
    }
}
