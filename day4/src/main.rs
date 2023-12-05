use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

static INPUT: &str = include_str!("input.txt");

fn main() {
    let part1 = part1(INPUT);
    let part2 = part2(INPUT);

    println!("{part1}");
    println!("{part2}");
}

pub fn part1(input: &str) -> usize {
    let mut point_sum = 0;

    for line in input.lines() {
        let (_, numbers) = line.split_once(':').unwrap();

        let (winning_numbers, numbers) = numbers.split_once('|').unwrap();

        let winning_numbers: HashSet<u8> = winning_numbers
            .split_ascii_whitespace()
            .map(FromStr::from_str)
            .collect::<Result<_, _>>()
            .unwrap();

        let winning_numbers_gotten = numbers
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .filter(|n| winning_numbers.contains(n))
            .count();

        let points = match winning_numbers_gotten {
            0 => 0,
            n => 2_usize.pow(n as u32 - 1),
        };

        point_sum += points;
    }

    point_sum
}

pub fn part2(input: &str) -> usize {
    let mut all_cards = HashMap::with_capacity(220);

    let mut winning_numbers_set = HashSet::with_capacity(10);

    for (i, line) in input.lines().enumerate() {
        let scratchcard_id = i + 1;

        let &mut amount = all_cards.entry(scratchcard_id).or_insert(1);

        let (_, numbers) = line.split_once(':').unwrap();

        let (winning_numbers, numbers) = numbers.split_once('|').unwrap();

        winning_numbers_set.extend(
            winning_numbers
                .split_ascii_whitespace()
                .map(|n| n.parse::<u8>().unwrap()),
        );

        let winning_numbers_gotten = numbers
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .filter(|n| winning_numbers_set.contains(n))
            .count();

        winning_numbers_set.clear();

        for scratchcard in scratchcard_id + 1..=scratchcard_id + winning_numbers_gotten {
            *all_cards.entry(scratchcard).or_insert(1) += amount;
        }
    }

    all_cards.into_values().sum()
}
