use std::collections::HashMap;
use std::ops::Index;

advent_of_code::solution!(8);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Coordinates {
    left: String,
    right: String,
}

type Map = HashMap<String, Coordinates>;
type DirectionVec = Vec<Direction>;

impl Index<&Direction> for Coordinates {
    type Output = String;

    fn index(&self, direction: &Direction) -> &Self::Output {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn parse_input(input: &str) -> (DirectionVec, Map) {
    let mut lines_iter = input.lines();
    let fist_line = lines_iter.next().unwrap();

    let instructions: Vec<Direction> = fist_line
        .chars()
        .map(|c| {
            if c == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect();

    let coordinates_map: Map = lines_iter.skip(1).fold(HashMap::new(), |mut acc, line| {
        let replaced = line
            .to_string()
            .replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "");
        let mut splitted_iter = replaced.split_whitespace();
        let current_key = splitted_iter.next().unwrap().to_string();

        let coord = Coordinates {
            left: splitted_iter.next().unwrap().to_string(),
            right: splitted_iter.next().unwrap().to_string(),
        };

        acc.insert(current_key.clone(), coord);
        acc
    });

    (instructions, coordinates_map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instructions, coordinates_map) = parse_input(input);

    //they are not the firs and la keys of the example
    let mut current_key = "AAA";
    let last_key = "ZZZ";

    let mut count: usize = 0;
    let mut idx: usize = 0;
    //
    while current_key != last_key {
        //println!(
        //    "Current Key: {} Counter: {} Idx: {}",
        //    current_key, count, idx
        //);
        let coord = coordinates_map.get(current_key).unwrap();

        let direction: &Direction = instructions.get(idx).unwrap();

        current_key = coord[direction].as_str();
        count += 1;
        idx += 1;
        //instructions array index check
        if idx == instructions.len() {
            idx = 0
        }
    }

    let res: u32 = u32::try_from(count).ok().unwrap();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_again() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
