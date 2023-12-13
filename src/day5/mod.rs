use std::ops::Range;

use anyhow::{Context, Result};
use itertools::Itertools;

use crate::day5::mapping::{Mapping, MappingType, parse};
use crate::day5::mapping::MappingType::{Location, Seeds};

mod mapping;

fn map(maps: &Vec<Vec<Mapping>>, id: usize, from: MappingType, to: MappingType) -> usize {
    if from > to {
        maps[to as usize..from as usize].iter().rev().fold(id, |id, maps|
            maps.iter()
                .find(|map| map.destination.contains(&id))
                .map(|map| map.source.start + id - map.destination.start)
                .unwrap_or(id))
    } else {
        maps[from as usize..to as usize].iter().fold(id, |id, maps|
            maps.iter()
                .find(|map| map.source.contains(&id))
                .map(|map| map.destination.start + id - map.source.start)
                .unwrap_or(id))
    }
}

pub fn part1(input: &Vec<String>) -> Result<usize> {
    let (seeds, maps) = parse(input)?;

    seeds.iter()
        .map(|seed| map(&maps, *seed, Seeds, Location))
        .min()
        .context("Failed to find any locations for {seeds}")
}

pub fn part2(input: &Vec<String>) -> Result<usize> {
    let (seeds, maps) = parse(input)?;

    let ranges = seeds.into_iter().tuples()
        .map(|(start, len)| start..start + len)
        .collect::<Vec<Range<usize>>>();

    (1..)
        .map(|location| (location, map(&maps, location, Location, Seeds)))
        .find(|(_, seed)| ranges.iter().any(|range| range.contains(seed)))
        .map(|(location, _)| location)
        .context("Failed to find nearest location for {seeds}")
}

#[cfg(test)]
mod tests {
    use super::{map, parse, part1, part2};
    use super::MappingType::{Fertilizer, Humidity, Light, Location, Seeds, Soil, Temperature, Water};

    const INPUT: &str = "seeds: 79 14 55 13

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
";

    #[test]
    fn test_map() {
        let input = INPUT.lines()
            .map(String::from)
            .collect::<Vec<String>>();

        let (_, maps) = parse(&input).unwrap();

        assert_eq!(map(&maps, 79, Seeds, Soil), 81);
        assert_eq!(map(&maps, 79, Seeds, Fertilizer), 81);
        assert_eq!(map(&maps, 79, Seeds, Water), 81);
        assert_eq!(map(&maps, 79, Seeds, Light), 74);
        assert_eq!(map(&maps, 79, Seeds, Temperature), 78);
        assert_eq!(map(&maps, 79, Seeds, Humidity), 78);
        assert_eq!(map(&maps, 79, Seeds, Location), 82);

        assert_eq!(map(&maps, 14, Seeds, Location), 43);
        assert_eq!(map(&maps, 55, Seeds, Location), 86);
        assert_eq!(map(&maps, 13, Seeds, Location), 35);

        assert_eq!(map(&maps, 35, Location, Seeds), 13);
    }

    #[test]
    fn test_part1() {
        let input = INPUT.lines()
            .map(String::from)
            .collect::<Vec<String>>();

        assert_eq!(part1(&input).unwrap(), 35);
    }

    #[test]
    fn test_part2() {
        let input = INPUT.lines()
            .map(String::from)
            .collect::<Vec<String>>();

        assert_eq!(part2(&input).unwrap(), 46);
    }
}
