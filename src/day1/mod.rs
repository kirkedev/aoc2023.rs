use std::collections::HashMap;

pub fn get_numbers(line: String) -> Vec<u32> {
    line.chars()
        .filter_map(|char| char.to_digit(10))
        .collect()
}

pub fn get_text_numbers(line: String) -> Vec<u32> {
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

pub fn calibrate(numbers: Vec<u32>) -> u32 {
    match (numbers.first(), numbers.last()) {
        (Some(first), Some(last)) => first * 10 + last,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::{calibrate, get_numbers, get_text_numbers};

    #[test]
    pub fn test_get_numbers() {
        assert_eq!(get_numbers("1abc2".to_string()), vec![1, 2]);
        assert_eq!(get_numbers("pqr3stu8vwx".to_string()), vec![3, 8]);
        assert_eq!(get_numbers("a1b2c3d4e5f".to_string()), vec![1, 2, 3, 4, 5]);
        assert_eq!(get_numbers("treb7uchet".to_string()), vec![7]);
    }

    #[test]
    pub fn test_get_text_numbers() {
        assert_eq!(get_text_numbers("two1nine".to_string()), vec![2, 1, 9]);
        assert_eq!(get_text_numbers("eightwothree".to_string()), vec![8, 2, 3]);
        assert_eq!(get_text_numbers("abcone2threexyz".to_string()), vec![1, 2, 3]);
        assert_eq!(get_text_numbers("xtwone3four".to_string()), vec![2, 1, 3, 4]);
        assert_eq!(get_text_numbers("4nineeightseven2".to_string()), vec![4, 9, 8, 7, 2]);
        assert_eq!(get_text_numbers("zoneight234".to_string()), vec![1, 8, 2, 3, 4]);
        assert_eq!(get_text_numbers("7pqrstsixteen".to_string()), vec![7, 6]);
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
