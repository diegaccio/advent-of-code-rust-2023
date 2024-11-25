advent_of_code::solution!(6);
use std::iter::zip;

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap().to_string();
    let second_line = lines.next().unwrap().to_string();
    let res = zip(
        first_line
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok()),
        second_line
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok()),
    )
    .map(|(time, distance)| {
        let mut count = 0;
        for x in 1..time {
            let run_distance = x * (time - x);
            if run_distance > distance {
                count += 1;
            } else if count > 0 {
                break;
            }
        }

        count
    })
    .product();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_for_part_one = input
        .lines()
        .map(|s| s.split(' ').skip(1).collect::<Vec<&str>>().join(""))
        .collect::<Vec<String>>()
        .join("\n");

    println!("{input_for_part_one}");

    return part_one(input_for_part_one.as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
