mod part1;
use part1::*;

use std::{
    fs::File,
    io::{self, BufReader},
};

pub static INPUT: &str = include_str!("input.txt");

fn main() {
    let part1 = part1_inlined_input(INPUT);
    let part1_file = part1_readbuf(BufReader::new(File::open("day1/src/input.txt").unwrap()));
    let part1_cursor = part1_readbuf(io::Cursor::new(INPUT));

    assert_eq!(part1, part1_file);
    assert_eq!(part1, part1_cursor);

    println!("{part1}");

    let part2 = part2_inlined_input(INPUT);

    println!("{part2}");
}

const MATCHERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part2_inlined_input(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = find_first(line);
            let last = find_last(line);

            first * 10 + last
        })
        .sum()
}

pub fn find_first(mut string: &str) -> usize {
    loop {
        let first = unsafe { string.as_bytes().first().unwrap_unchecked() };
        if first.is_ascii_digit() {
            return (first - b'0').into();
        }

        if let Some(first) = MATCHERS.into_iter().position(|x| string.starts_with(x)) {
            return first + 1;
        }

        string = &string[1..];
    }
}

pub fn find_last(mut string: &str) -> usize {
    loop {
        let first = unsafe { string.as_bytes().last().unwrap_unchecked() };
        if first.is_ascii_digit() {
            return (first - b'0').into();
        }

        if let Some(first) = MATCHERS.into_iter().position(|x| string.ends_with(x)) {
            return first + 1;
        }

        string = &string[..string.len() - 1];
    }
}
