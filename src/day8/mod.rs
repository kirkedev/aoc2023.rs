use std::collections::HashMap;

use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Eq)]
struct Location {
    id: String,
    left: String,
    right: String,
}

fn parse(input: &Vec<String>) -> Result<(String, HashMap<String, Location>)> {
    let (directions, locations) = input.split_at(1);

    let directions = directions.iter().next()
        .context("Failed to parse directions from {input}")?;

    let locations: HashMap<String, Location> = locations.iter()
        .skip(1)
        .map(|line| line.split_once(" = ")
            .context("Failed to split line")
            .and_then(|(id, locations)| locations.strip_prefix('(')
                .and_then(|location| location.strip_suffix(')'))
                .and_then(|location| location.split_once(", "))
                .context("Failed to parse {line}")
                .map(|(left, right)| (id.into(), Location {
                    id: id.into(),
                    left: left.into(),
                    right: right.into(),
                }))))
        .collect::<Result<HashMap<String, Location>>>()?;

    Ok((directions.into(), locations))
}

pub fn part1(input: &Vec<String>) -> Result<u32> {
    let (directions, locations) = parse(&input)?;

    directions.chars()
        .cycle()
        .scan(locations.get("AAA"), |location, direction| {
            *location = match direction {
                'L' => location.and_then(|location| locations.get(&location.left)),
                'R' => location.and_then(|location| locations.get(&location.right)),
                _ => None
            };

            return *location;
        })
        .zip(1u32..)
        .find(|(location, _)| location.id == "ZZZ")
        .map(|(_, steps)| steps)
        .context("Failed to find ZZZ")
}

#[cfg(test)]
mod tests {
    use super::{Location, parse, part1};

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_parse() {
        let input: Vec<String> = INPUT.lines().map(String::from).collect();
        let (directions, locations) = parse(&input).unwrap();

        assert_eq!(directions, "LLR".to_string());
        assert_eq!(locations.len(), 3);
        assert_eq!(locations.get("AAA").unwrap(), &Location { id: "AAA".into(), left: "BBB".into(), right: "BBB".into() });
        assert_eq!(locations.get("BBB").unwrap(), &Location { id: "BBB".into(), left: "AAA".into(), right: "ZZZ".into() });
        assert_eq!(locations.get("ZZZ").unwrap(), &Location { id: "ZZZ".into(), left: "ZZZ".into(), right: "ZZZ".into() });
    }

    #[test]
    fn test_part1() {
        let input: Vec<String> = INPUT.lines().map(String::from).collect();
        assert_eq!(part1(&input).unwrap(), 6)
    }
}
