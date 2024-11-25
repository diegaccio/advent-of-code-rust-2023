use std::collections::HashMap;

advent_of_code::solution!(7);

static CARDS: [u8; 13] = [
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A',
];

fn get_hand_strength<T, A>(hand: T, jokers: &Option<A>) -> Vec<isize>
where
    T: AsRef<str>,
    A: AsRef<[usize]>,
{
    #[allow(clippy::map_unwrap_or)]
    let jokers = jokers.as_ref().map(AsRef::as_ref).unwrap_or_default();
    let hand = hand.as_ref();
    let mut counter = HashMap::with_capacity(hand.len());

    for c in hand.bytes() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    let mut strength = vec![match (counter.values().max(), counter.len()) {
        (Some(5), _) => 6,
        (Some(4), _) => 5,
        (Some(3), 2) => 4,
        (Some(3), 3) => 3,
        (Some(2), 3) => 2,
        (Some(2), 4) => 1,
        _ => 0,
    }];
    #[allow(clippy::cast_possible_wrap)]
    strength.extend(hand.bytes().enumerate().map(|(i, card)| {
        if jokers.contains(&i) {
            -1
        } else {
            CARDS.iter().position(|&r| r == card).unwrap_or_default() as isize
        }
    }));
    strength
}

fn get_hand_strength_joker<T>(hand: T) -> Vec<isize>
where
    T: AsRef<str>,
{
    let hand = hand.as_ref();
    if hand == "JJJJJ" {
        get_hand_strength("AAAAA", &Some((0..5).collect::<Vec<usize>>()))
    } else {
        get_hand_strength(
            hand.replace(
                'J',
                std::str::from_utf8(&[hand
                    .bytes()
                    .filter(|&card| card != b'J')
                    .max_by_key(|card| hand.bytes().filter(|c| c == card).count())
                    .unwrap()])
                .unwrap(),
            ),
            &Some(
                hand.bytes()
                    .enumerate()
                    .filter_map(|(i, card)| (card == b'J').then_some(i))
                    .collect::<Vec<usize>>(),
            ),
        )
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = input.to_string();

    let mut hands = hands
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<(&str, usize)>>();

    hands.sort_by_key(|(hand, _)| get_hand_strength(hand, &None::<&[usize]>));

    let res_usize: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum();

    let res = u32::try_from(res_usize).ok().unwrap();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands = input.to_string();
    let mut hands = hands
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (hand, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<(&str, usize)>>();

    hands.sort_by_key(|(hand, _)| get_hand_strength_joker(hand));
    let res_usize: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum();

    let res = u32::try_from(res_usize).ok().unwrap();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
