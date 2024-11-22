advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .to_string()
        .lines()
        .map(|line| {
            let (game_id, plays) = line.split_once(':').unwrap();
            let game_id = game_id.trim_start_matches("Game ").parse::<u32>().unwrap();

            if plays.split(';').all(|play| {
                let mut red = 0;
                let mut blue = 0;
                let mut green = 0;

                for color in play.splitn(3, ',') {
                    let (num, name) = color.trim().split_once(' ').unwrap();
                    let num = num.parse::<u32>().unwrap();
                    match name {
                        "red" => red += num,
                        "green" => green += num,
                        "blue" => blue += num,
                        _ => (),
                    }
                }
                red <= 12 && green <= 13 && blue <= 14
            }) {
                game_id
            } else {
                0
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .to_string()
        .lines()
        .map(|line| {
            let (_, plays) = line.split_once(':').unwrap();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for color in plays.replace(',', ";").split(';') {
                let (num, name) = color.trim().split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();
                match name {
                    "red" if num > red => red = num,
                    "green" if num > green => green = num,
                    "blue" if num > blue => blue = num,
                    _ => (),
                }
            }
            red * green * blue
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
