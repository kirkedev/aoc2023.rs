use std::collections::HashSet;

pub fn get_numbers(numbers: &str) -> HashSet<u32> {
    numbers.split_ascii_whitespace()
        .map_while(|number| number.parse::<u32>().ok())
        .collect::<HashSet<u32>>()
}

pub fn get_winners(input: &Vec<String>) -> Vec<usize> {
    input.iter()
        .map_while(|line|
            line.split_once(':')
                .and_then(|(_, numbers)|
                    numbers.split_once(" | "))
                .map(|(winners, numbers)|
                    get_numbers(numbers).intersection(&get_numbers(winners)).count()))
        .collect()
}

pub fn part1(input: &Vec<String>) -> u32 {
    get_winners(input).iter()
        .filter(|matches| **matches > 0)
        .map(|matches| 1 << matches - 1)
        .sum()
}

pub fn part2(input: &Vec<String>) -> u32 {
    let mut cards = vec![1u32; input.len()];

    let winners = get_winners(input).into_iter()
        .enumerate()
        .filter(|(_, winners)| *winners > 0);

    for (i, winners) in winners {
        let count = cards[i];

        for card in &mut cards[(i + 1)..=(i + winners)] {
            *card += count;
        }
    }

    return cards.iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::day4::{part1, part2};

    #[test]
    pub fn test_part_1() {
        let input: Vec<String> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".into(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".into(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".into(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".into(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".into(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".into(),
        ];

        assert_eq!(part1(&input), 13);
    }

    #[test]
    pub fn test_part_2() {
        let input: Vec<String> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".into(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".into(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".into(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".into(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".into(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".into(),
        ];

        assert_eq!(part2(&input), 30);
    }
}
