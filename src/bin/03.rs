use std::collections::HashMap;

advent_of_code::solution!(3);

fn symbol_adjacent<A, C, F>(arr: A, coordinates: C, condition: F) -> Vec<(usize, usize)>
where
    F: Fn(char) -> bool,
    A: AsRef<[Vec<char>]>,
    C: AsRef<[(usize, usize)]>,
{
    let arr = arr.as_ref();
    let coordinates = coordinates.as_ref();
    let n_coords = coordinates.len() - 1;

    coordinates
        .iter()
        .enumerate()
        .flat_map(|(i, &(row, col))| {
            let mut indices = vec![(row.wrapping_sub(1), col), (row + 1, col)];
            if i == 0 {
                indices.extend([
                    (row, col.wrapping_sub(1)),
                    (row.wrapping_sub(1), col.wrapping_sub(1)),
                    (row + 1, col.wrapping_sub(1)),
                ]);
            }
            if i == n_coords {
                indices.extend([
                    (row, col + 1),
                    (row.wrapping_sub(1), col + 1),
                    (row + 1, col + 1),
                ]);
            }
            indices.into_iter().filter_map(|coord @ (row, col)| {
                arr.get(row)
                    .and_then(|row| row.get(col).and_then(|&c| condition(c).then_some(coord)))
            })
        })
        .collect::<Vec<(usize, usize)>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let arr = input
        .to_string()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut total = 0;

    let mut curr_indices = Vec::new();
    let mut curr_num = String::new();
    for (y, row) in arr.iter().enumerate() {
        for (x, chr) in row.iter().enumerate() {
            if chr.is_numeric() {
                curr_indices.push((y, x));
                curr_num.push(*chr);
            } else {
                if !curr_indices.is_empty()
                    && !symbol_adjacent(&arr, &curr_indices, |c| !c.is_numeric() && c != '.')
                        .is_empty()
                {
                    total += curr_num.parse::<u32>().unwrap();
                }
                curr_indices.clear();
                curr_num.clear();
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let arr = input
        .to_string()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut total = 0;

    for (y, row) in arr.iter().enumerate() {
        for (x, chr) in row.iter().enumerate() {
            if *chr == '*' {
                let nums = symbol_adjacent(&arr, [(y, x)], char::is_numeric);
                if !nums.is_empty() {
                    let mut num_map = HashMap::new();
                    for (y, mut x) in nums {
                        let mut curr_num = String::new();

                        while arr
                            .get(y)
                            .and_then(|row| row.get(x.wrapping_sub(1)).filter(|c| c.is_numeric()))
                            .is_some()
                        {
                            x -= 1;
                        }

                        while let Some(chr) = arr
                            .get(y)
                            .and_then(|row| row.get(x).filter(|c| c.is_numeric()))
                        {
                            curr_num.push(*chr);
                            x += 1;
                        }
                        num_map.insert((y, x), curr_num);
                    }
                    if num_map.len() == 2 {
                        let mut values = num_map.values();
                        total += values.next().and_then(|a| a.parse::<u32>().ok()).unwrap()
                            * values.next().and_then(|b| b.parse::<u32>().ok()).unwrap();
                    }
                }
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
