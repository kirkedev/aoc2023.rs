use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub contents: String,
    pub left: usize,
    pub top: usize,
}

impl Object {
    pub fn is_number(&self) -> bool {
        self.contents.chars().all(|char| char.is_digit(10))
    }

    pub fn is_adjacent(&self, other: &Object) -> bool {
        return self.top.abs_diff(other.top) <= 1 &&
            (self.left.abs_diff(other.left + other.contents.len() - 1) <= 1 ||
                other.left.abs_diff(self.left + self.contents.len() - 1) <= 1);
    }
}

fn get_objects(top: usize, line: &str) -> Vec<Object> {
    Regex::new(r"(\d+|[^\d\.])").unwrap()
        .find_iter(line)
        .map(|object| Object {
            top,
            left: object.start(),
            contents: object.as_str().into(),
        })
        .collect()
}

fn parse(input: &Vec<String>) -> Vec<Object> {
    input.iter()
        .enumerate()
        .flat_map(|(top, line)| get_objects(top, line))
        .collect()
}

pub fn part1(input: &Vec<String>) -> u32 {
    let (numbers, symbols): (Vec<Object>, Vec<Object>) = parse(input).into_iter()
        .partition(|object| object.is_number());

    numbers.iter()
        .filter(|number| symbols.iter().any(|symbol| number.is_adjacent(symbol)))
        .map_while(|number| number.contents.parse::<u32>().ok())
        .sum()
}

pub fn part2(input: &Vec<String>) -> u32 {
    let (numbers, symbols): (Vec<Object>, Vec<Object>) = parse(input).into_iter()
        .partition(|object| object.is_number());

    symbols.iter()
        .map(|symbol|
            numbers.iter()
                .filter(|number| number.is_adjacent(symbol))
                .map_while(|number| number.contents.parse::<u32>().ok())
                .collect::<Vec<u32>>())
        .fold(0, |total, gears|
            if gears.len() == 2 {
                total + gears.iter().product::<u32>()
            } else {
                total
            })
}

#[cfg(test)]
mod tests {
    use crate::day3::{get_objects, Object, parse, part1, part2};

    #[test]
    pub fn test_get_objects() {
        assert_eq!(get_objects(4, "617*......"), vec![
            Object { contents: "617".into(), top: 4, left: 0 },
            Object { contents: "*".into(), top: 4, left: 3 },
        ]);
    }

    #[test]
    pub fn test_parse() {
        let schematic: Vec<String> = vec![
            "467..114..".into(),
            "...*......".into(),
            "..35..633.".into(),
            "......#...".into(),
            "617*......".into(),
            ".....+.58.".into(),
            "..592.....".into(),
            "......755.".into(),
            "...$.*....".into(),
            ".664.598..".into(),
        ];

        let objects = parse(&schematic);
        assert_eq!(objects.iter().count(), 16);
        assert_eq!(objects.first(), Some(Object { contents: "467".into(), left: 0, top: 0 }).as_ref());
        assert_eq!(objects.last(), Some(Object { contents: "598".into(), left: 5, top: 9 }).as_ref());
        assert_eq!(objects.contains(&Object { contents: "$".into(), left: 3, top: 8 }), true);
        assert_eq!(objects.contains(&Object { contents: "*".into(), left: 5, top: 8 }), true);
    }

    #[test]
    pub fn test_is_adjacent() {
        let object = Object { contents: "664".into(), top: 9, left: 1 };
        let other = Object { contents: "$".into(), left: 3, top: 8 };
        assert_eq!(object.is_adjacent(&other), true);
        assert_eq!(other.is_adjacent(&object), true);

        assert_eq!(object.is_adjacent(&Object { contents: "*".into(), left: 0, top: 8 }), true);
        assert_eq!(object.is_adjacent(&Object { contents: "*".into(), left: 1, top: 8 }), true);
        assert_eq!(object.is_adjacent(&Object { contents: "*".into(), left: 2, top: 8 }), true);
        assert_eq!(object.is_adjacent(&Object { contents: "*".into(), left: 3, top: 8 }), true);
        assert_eq!(object.is_adjacent(&Object { contents: "*".into(), left: 4, top: 8 }), true);
        assert_eq!(object.is_adjacent(&Object { contents: "*".into(), left: 5, top: 8 }), false);

        assert_eq!(other.is_adjacent(&Object { contents: "*".into(), left: 0, top: 7 }), false);
        assert_eq!(other.is_adjacent(&Object { contents: "*".into(), left: 1, top: 7 }), false);
        assert_eq!(other.is_adjacent(&Object { contents: "*".into(), left: 2, top: 7 }), true);
        assert_eq!(other.is_adjacent(&Object { contents: "*".into(), left: 3, top: 7 }), true);
        assert_eq!(other.is_adjacent(&Object { contents: "*".into(), left: 4, top: 7 }), true);
        assert_eq!(other.is_adjacent(&Object { contents: "*".into(), left: 5, top: 7 }), false);
    }

    #[test]
    pub fn test_part_1() {
        let schematic = vec![
            "467..114..".into(),
            "...*......".into(),
            "..35..633.".into(),
            "......#...".into(),
            "617*......".into(),
            ".....+.58.".into(),
            "..592.....".into(),
            "......755.".into(),
            "...$.*....".into(),
            ".664.598..".into(),
        ];

        assert_eq!(part1(&schematic), 4361);
    }

    #[test]
    pub fn test_part_2() {
        let schematic = vec![
            "467..114..".into(),
            "...*......".into(),
            "..35..633.".into(),
            "......#...".into(),
            "617*......".into(),
            ".....+.58.".into(),
            "..592.....".into(),
            "......755.".into(),
            "...$.*....".into(),
            ".664.598..".into(),
        ];

        assert_eq!(part2(&schematic), 467835);
    }
}
