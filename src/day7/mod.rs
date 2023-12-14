use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn rank_hand(cards: [char; 5]) -> Hand {
    let counts = cards.iter().counts();

    match counts.len() {
        1 => Hand::FiveOfAKind,
        2 => if counts.into_values().any(|value| value == 4) {
            Hand::FourOfAKind
        } else {
            Hand::FullHouse
        },
        3 => if counts.into_values().any(|value| value == 3) {
            Hand::ThreeOfAKind
        } else {
            Hand::TwoPair
        },
        4 => Hand::OnePair,
        _ => Hand::HighCard
    }
}

fn rank_hand_with_jokers(cards: [char; 5]) -> Hand {
    if cards.contains(&'J') {
        let counts = cards.into_iter()
            .filter(|card| *card != 'J')
            .counts();

        let (best, _) = counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .unwrap_or(('J', 5));

        rank_hand(cards.map(|card| if card == 'J' { best } else { card }))
    } else {
        rank_hand(cards)
    }
}

fn parse(input: &Vec<String>) -> Result<Vec<([char; 5], u32)>> {
    input.iter()
        .map(|line| line.split_once(' ').context("Failed to split {line}"))
        .collect::<Result<Vec<(&str, &str)>>>()?
        .into_iter()
        .map(|(hand, bid)| {
            let bid = bid.parse::<u32>()?;

            let hand: [char; 5] = hand.chars()
                .take(5)
                .collect::<Vec<char>>()[..5]
                .try_into()?;

            Ok((hand, bid))
        })
        .collect()
}

fn card_ranker(cards: [char; 13]) -> impl Fn([char; 5]) -> [u8; 5] {
    move |hand| hand.map(|card|
        cards.iter().position(|&item| item == card).unwrap_or(0) as u8)
}

pub fn part1(input: &Vec<String>) -> Result<u32> {
    let rank_cards = card_ranker(['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']);

    let result = parse(input)?.iter()
        .sorted_by_key(|(hand, _)| (rank_hand(*hand), rank_cards(*hand)))
        .collect_vec()
        .into_iter()
        .map(|(_, bid)| bid)
        .zip(1u32..)
        .fold(0u32, |value, (bid, rank)| value + rank * bid);

    Ok(result)
}

pub fn part2(input: &Vec<String>) -> Result<u32> {
    let rank_cards = card_ranker(['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A']);

    let result = parse(input)?.iter()
        .sorted_by_key(|(hand, _)| (rank_hand_with_jokers(*hand), rank_cards(*hand)))
        .collect_vec()
        .into_iter()
        .map(|(_, bid)| bid)
        .zip(1u32..)
        .fold(0u32, |value, (bid, rank)| value + rank * bid);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{Hand, parse, part1, part2, rank_hand, rank_hand_with_jokers};

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_parse() {
        let input = INPUT.lines().map(String::from).collect();
        let hands = parse(&input).unwrap();

        assert_eq!(hands.len(), 5);
        assert_eq!(*hands.first().unwrap(), (['3', '2', 'T', '3', 'K'], 765));
        assert_eq!(*hands.iter().nth(1).unwrap(), (['T', '5', '5', 'J', '5'], 684));
        assert_eq!(*hands.last().unwrap(), (['Q', 'Q', 'Q', 'J', 'A'], 483));
    }

    #[test]
    fn test_rank_hand() {
        let input = INPUT.lines().map(String::from).collect();
        let hands = parse(&input).unwrap();

        assert_eq!(rank_hand(hands.iter().nth(0).unwrap().0), Hand::OnePair);
        assert_eq!(rank_hand(hands.iter().nth(1).unwrap().0), Hand::ThreeOfAKind);
        assert_eq!(rank_hand(hands.iter().nth(2).unwrap().0), Hand::TwoPair);
        assert_eq!(rank_hand(hands.iter().nth(3).unwrap().0), Hand::TwoPair);
        assert_eq!(rank_hand(hands.iter().nth(4).unwrap().0), Hand::ThreeOfAKind);
    }

    #[test]
    fn test_rank_hand_with_jokers() {
        let input = INPUT.lines().map(String::from).collect();
        let hands = parse(&input).unwrap();

        assert_eq!(rank_hand_with_jokers(hands.iter().nth(0).unwrap().0), Hand::OnePair);
        assert_eq!(rank_hand_with_jokers(hands.iter().nth(1).unwrap().0), Hand::FourOfAKind);
        assert_eq!(rank_hand_with_jokers(hands.iter().nth(2).unwrap().0), Hand::TwoPair);
        assert_eq!(rank_hand_with_jokers(hands.iter().nth(3).unwrap().0), Hand::FourOfAKind);
        assert_eq!(rank_hand_with_jokers(hands.iter().nth(4).unwrap().0), Hand::FourOfAKind);
    }

    #[test]
    fn test_part_1() {
        let input = INPUT.lines().map(String::from).collect();
        assert_eq!(part1(&input).unwrap(), 6440);
    }

    #[test]
    fn test_part_2() {
        let input = INPUT.lines().map(String::from).collect();
        assert_eq!(part2(&input).unwrap(), 5905);
    }
}