advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    //print!("{}", input);
    let res = input
        .lines()
        .map(|line| {
            let mut digits = line
                .chars()
                .filter_map(|c| c.to_string().parse::<u32>().ok());
            //.filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap_or(0);
            first * 10 + digits.last().unwrap_or(first)
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_string = input.to_string();
    let map = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    for (key, val) in map {
        input_string = input_string.replace(key, format!("{key}{val}{key}").as_str());
    }
    part_one(input_string.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
