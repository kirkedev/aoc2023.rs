use std::collections::HashMap;

pub fn get_numbers(string: String) -> Vec<u32> {
    string.chars()
        .filter_map(|char| char.to_digit(10))
        .collect()
}

pub fn get_text_numbers(string: String) -> Vec<u32> {
    let numbers: HashMap<&str, u32> = HashMap::from([
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

    let mut matches: Vec<u32> = Vec::new();

    for i in 0..string.len() {
        if let Some(number) = string.chars().nth(i).unwrap().to_digit(10) {
            matches.push(number);
        } else {
            let slice = &string[i..string.len()].to_string();

            if let Some(number) = numbers.keys().find(|number| slice.starts_with(*number)) {
                matches.push(*numbers.get(number).unwrap());
            }
        }
    }

    return matches;
}

pub fn calibrate(numbers: Vec<u32>) -> Option<u32> {
    let first = numbers.first();
    let last = numbers.last();
    return Some(first? * 10 + last?);
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
        assert_eq!(calibrate(vec!(1, 2)), Some(12));
        assert_eq!(calibrate(vec!(3, 8)), Some(38));
        assert_eq!(calibrate(vec!(1, 2, 3, 4, 5)), Some(15));
        assert_eq!(calibrate(vec!(7)), Some(77));
    }
}
