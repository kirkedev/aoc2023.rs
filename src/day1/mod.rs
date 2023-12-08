use std::collections::HashMap;

fn get_numbers(line: &str) -> Vec<u32> {
    line.chars()
        .filter_map(|char| char.to_digit(10))
        .collect()
}

fn get_text_numbers(line: &str) -> Vec<u32> {
    let digits: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let mut numbers: Vec<u32> = Vec::new();

    for i in 0..line.len() {
        let string = &line[i..];

        let digit = string.chars().next()
            .and_then(|char| char.to_digit(10))
            .or(digits.keys()
                .find(|digit| string.starts_with(*digit))
                .and_then(|digit| digits.get(digit).copied()));

        if let Some(number) = digit {
            numbers.push(number);
        }
    }

    return numbers;
}

fn calibrate(numbers: Vec<u32>) -> u32 {
    match numbers.first().zip(numbers.last()) {
        Some((first, last)) => first * 10 + last,
        None => 0
    }
}

pub fn part1(lines: &Vec<String>) -> u32 {
    lines.iter()
        .map(|string| get_numbers(string.as_str()))
        .map(calibrate)
        .sum::<u32>()
}

pub fn part2(lines: &Vec<String>) -> u32 {
    lines.iter()
        .map(|string| get_text_numbers(string.as_str()))
        .map(calibrate)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::day1::{calibrate, get_numbers, get_text_numbers};

    #[test]
    pub fn test_get_numbers() {
        assert_eq!(get_numbers("1abc2"), vec![1, 2]);
        assert_eq!(get_numbers("pqr3stu8vwx"), vec![3, 8]);
        assert_eq!(get_numbers("a1b2c3d4e5f"), vec![1, 2, 3, 4, 5]);
        assert_eq!(get_numbers("treb7uchet"), vec![7]);
    }

    #[test]
    pub fn test_get_text_numbers() {
        assert_eq!(get_text_numbers("two1nine"), vec![2, 1, 9]);
        assert_eq!(get_text_numbers("eightwothree"), vec![8, 2, 3]);
        assert_eq!(get_text_numbers("abcone2threexyz"), vec![1, 2, 3]);
        assert_eq!(get_text_numbers("xtwone3four"), vec![2, 1, 3, 4]);
        assert_eq!(get_text_numbers("4nineeightseven2"), vec![4, 9, 8, 7, 2]);
        assert_eq!(get_text_numbers("zoneight234"), vec![1, 8, 2, 3, 4]);
        assert_eq!(get_text_numbers("7pqrstsixteen"), vec![7, 6]);
    }

    #[test]
    pub fn test_calibrate() {
        assert_eq!(calibrate(vec![1, 2]), 12);
        assert_eq!(calibrate(vec![3, 8]), 38);
        assert_eq!(calibrate(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(calibrate(vec![7]), 77);
        assert_eq!(calibrate(vec![]), 0);
    }
}
