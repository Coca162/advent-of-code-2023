use core::panic;
use std::{collections::BTreeMap, mem, num::IntErrorKind};

pub static INPUT: &str = include_str!("input.txt");

fn main() {
    let (part1, part2) = part12(INPUT);

    println!("{part1}");
    println!("{part2}");
}

pub fn part12(input: &str) -> (usize, usize) {
    let mut lines = input.lines();

    let mut sum = 0;
    let mut gearbox_sum = 0;

    // Assuming first lines has no symbols
    let mut prev_line = lines.next().unwrap();

    let mut prev_line_symbols = Vec::<(usize, u8)>::new();
    let mut current_line_symbols = Vec::<(usize, u8)>::new();

    let mut prev_possible_gearsets = BTreeMap::<usize, Vec<usize>>::new();
    let mut current_possible_gearsets = BTreeMap::<usize, Vec<usize>>::new();

    for line in lines {
        let mut skip = 0;

        for (i, &c) in line.as_bytes().iter().enumerate() {
            if skip != 0 {
                skip -= 1;
                continue;
            }

            if c.is_ascii_punctuation() && c != b'.' {
                let (top_first, top_last) = parse_top_line(prev_line, i);

                sum += top_first.unwrap_or(0);
                sum += top_last.unwrap_or(0);

                current_line_symbols.push((i, c));

                let left_num = parse_3_digits_rev(line.split_at(i).0);

                sum += left_num.map_or(0, |(n, _)| n);

                let right_num = parse_3_digits(line.split_at(i + 1).1);

                skip = right_num.map_or(0, |(_, s)| s);

                sum += right_num.map_or(0, |(n, _)| n);

                if c == b'*' {
                    let gearsets: Vec<usize> = [
                        top_first,
                        top_last,
                        left_num.map(|(n, _)| n),
                        right_num.map(|(n, _)| n),
                    ]
                    .into_iter()
                    .flatten()
                    .collect();

                    if gearsets.len() <= 2 {
                        current_possible_gearsets.insert(i, gearsets);
                    }
                }
            }

            if c.is_ascii_digit() {
                if let Some(&(idx, c)) = prev_line_symbols
                    .iter()
                    .find(|(p_i, _)| (p_i - 1..=p_i + 1).contains(&i))
                {
                    let (num, skip_next) = parse_free_num(line, i);

                    skip = skip_next;

                    if c == b'*' {
                        prev_possible_gearsets.entry(idx).or_default().push(num);
                    }

                    sum += num;
                }
            }
        }

        prev_line_symbols.clear();
        mem::swap(&mut prev_line_symbols, &mut current_line_symbols);

        gearbox_sum += prev_possible_gearsets
            .values()
            .filter_map(|x| x.as_slice().try_into().ok())
            .map(|[x, y]: [usize; 2]| x * y)
            .sum::<usize>();

        prev_possible_gearsets.clear();

        mem::swap(&mut prev_possible_gearsets, &mut current_possible_gearsets);

        prev_line = line;
    }

    (sum, gearbox_sum)
}

fn parse_top_line(prev_line: &str, i: usize) -> (Option<usize>, Option<usize>) {
    let combined: [u8; 3] = prev_line.as_bytes()[i - 1..=i + 1].try_into().unwrap();
    let nice = combined.map(|x| x.is_ascii_digit());

    match nice {
        [true, false, true] => {
            let (left, right) = prev_line.split_at(i);
            (
                parse_3_digits_rev(left).map(|x| x.0),
                parse_3_digits(&right[1..]).map(|x| x.0),
            )
        }
        [true, true, true] => (
            Some(
                unsafe { std::str::from_utf8_unchecked(&combined) }
                    .parse()
                    .unwrap(),
            ),
            None,
        ),

        [false, true, false] => (Some((combined[1] - b'0') as usize), None),
        [false, false, false] => (None, None),

        [false, middle, true] => (
            None,
            parse_3_digits(prev_line.split_at(i + usize::from(!middle)).1).map(|x| x.0),
        ),
        [true, middle, false] => (
            parse_3_digits_rev(prev_line.split_at(i + usize::from(middle)).0).map(|x| x.0),
            None,
        ),
    }
}

fn parse_free_num(prev_line: &str, i: usize) -> (usize, usize) {
    let bytes = prev_line.as_bytes();
    match [bytes[i - 1].is_ascii_digit(), bytes[i + 1].is_ascii_digit()] {
        [true, true] => (
            unsafe { std::str::from_utf8_unchecked(&bytes[i - 1..=i + 1]) }
                .parse()
                .unwrap(),
            1,
        ),
        [true, false] => {
            let (n, _) = parse_3_digits_rev(prev_line.split_at(i + 1).0).unwrap();
            (n, 0)
        }
        [false, true] => {
            let (n, len) = parse_3_digits(prev_line.split_at(i).1).unwrap();

            (n, len - 1)
        }
        [false, false] => ((bytes[i] - b'0') as usize, 0),
    }
}

fn parse_3_digits_rev(string: &str) -> Option<(usize, usize)> {
    let possible_num = string
        .rsplit_once(|c: char| !c.is_ascii_digit())
        .map(|(_, x)| x)
        .unwrap_or(string);
    let len = possible_num.len();

    return match possible_num.parse() {
        Ok(n) => Some((n, len)),
        Err(e) if e.kind() == &IntErrorKind::Empty => None,
        Err(e) => panic!("{e}"),
    };
}

fn parse_3_digits(string: &str) -> Option<(usize, usize)> {
    let possible_num = string
        .split_once(|c: char| !c.is_ascii_digit())
        .map(|(x, _)| x)
        .unwrap_or(string);
    let len = possible_num.len();

    return match possible_num.parse() {
        Ok(n) => Some((n, len)),
        Err(e) if e.kind() == &IntErrorKind::Empty => None,
        Err(e) => panic!("{e}"),
    };
}
