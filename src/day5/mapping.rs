use std::ops::Range;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum MappingType {
    Seeds = 0,
    Soil = 1,
    Fertilizer = 2,
    Water = 3,
    Light = 4,
    Temperature = 5,
    Humidity = 6,
    Location = 7,
}

#[derive(Debug, PartialEq)]
pub struct Mapping {
    pub source: Range<usize>,
    pub destination: Range<usize>,
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        Regex::new(r"(?<dest>\d+)\s(?<src>\d+)\s(?<len>\d+)").unwrap()
            .captures(input)
            .context("Failed to find mapping values in {input}")
            .and_then(|captures|
                Ok([
                    captures["src"].parse::<usize>()?,
                    captures["dest"].parse::<usize>()?,
                    captures["len"].parse::<usize>()?
                ]))
            .map(|[src, dest, len]|
                Mapping {
                    source: src..src + len,
                    destination: dest..dest + len,
                })
    }
}

pub fn parse(input: Vec<String>) -> Result<(Vec<usize>, Vec<Vec<Mapping>>)> {
    let pattern = Regex::new(r"(?<dest>\d+)\s(?<src>\d+)\s(?<len>\d+)").unwrap();
    let mut iterator = input.iter();

    let seeds: Vec<usize> = iterator.next()
        .context("Tried parsing empty input")
        .and_then(|seeds|
            seeds.strip_prefix("seeds: ")
                .context("Failed to remove seeds prefix from {seeds}"))
        .and_then(|seeds|
            seeds.split_ascii_whitespace()
                .map(|id| id.parse::<usize>().map_err(|err| anyhow!(err)))
                .collect::<Result<Vec<usize>>>())?;

    let maps = iterator
        .group_by(|line| pattern.is_match(line))
        .into_iter()
        .filter(|(key, _)| *key)
        .map(|(_, group)|
            group
                .into_iter()
                .map(|line| line.parse::<Mapping>()).collect())
        .collect::<Result<Vec<Vec<Mapping>>>>()?;

    Ok((seeds, maps))
}

#[cfg(test)]
mod tests {
    use super::{Mapping, parse};

    #[test]
    fn test_parse() {
        let input = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
".lines()
            .map(String::from)
            .collect::<Vec<String>>();

        let (seeds, mappings) = parse(input).unwrap();

        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(mappings.len(), 7);

        assert_eq!(*mappings.get(0).unwrap(), vec![
            Mapping {
                source: 98..100,
                destination: 50..52,
            }, Mapping {
                source: 50..98,
                destination: 52..100,
            }]);

        assert_eq!(*mappings.get(1).unwrap(), vec![
            Mapping {
                source: 15..52,
                destination: 0..37,
            }, Mapping {
                source: 52..54,
                destination: 37..39,
            }, Mapping {
                source: 0..15,
                destination: 39..54,
            }]);
    }

    #[test]
    fn test_parse_mapping() {
        assert_eq!("50 98 2".parse::<Mapping>().unwrap(), Mapping {
            source: 98..100,
            destination: 50..52,
        });
    }
}
