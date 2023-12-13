use std::{collections::HashMap, ops::Not};

use parser::{LeftRight, Name};

use crate::parser::parse;

mod parser;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let (direction, noderaw) = parse(INPUT).unwrap();

    let part1 = part1(&direction, &noderaw);

    println!("{part1}");

    let part2 = part2(&direction, &noderaw);

    println!("{part2}");
}

pub fn part1(directions: &[Direction], node_map: &HashMap<Name, LeftRight>) -> usize {
    steps_to_byte_end::<b'Z'>(*b"AAA", node_map, directions)
}

pub fn part2(directions: &[Direction], node_map: &HashMap<Name, LeftRight>) -> usize {
    node_map
        .iter()
        .map(|(n, _)| n)
        .filter(|name| name.ends_with(&[b'A']))
        .map(|&node| steps_to_byte_end::<b'Z'>(node, node_map, directions))
        .reduce(num_integer::lcm)
        .unwrap()
}

pub fn steps_to_byte_end<const END: u8>(
    node: Name,
    map: &HashMap<Name, LeftRight>,
    directions: &[Direction],
) -> usize {
    directions
        .iter()
        .cycle()
        .scan(node, |node, d| {
            let ((Direction::Left, (next, _)) | (Direction::Right, (_, next))) = (d, map[node]);
            *node = next;
            next.ends_with(&[END]).not().then_some(next)
        })
        .count()
        + 1
}

pub fn part2_brute_force(directions: &[Direction], node_map: &HashMap<Name, LeftRight>) -> usize {
    let mut nodes_at = node_map
        .iter()
        .map(|(n, _)| n)
        .filter(|name| name.ends_with(&[b'A']))
        .copied()
        .collect::<Vec<_>>();

    let mut directions_cycle = directions.iter().cycle().enumerate();

    for (_, direction) in directions_cycle.by_ref() {
        for node in nodes_at.iter_mut() {
            *node = match (direction, node_map[node]) {
                (Direction::Left, (left, _)) => left,
                (Direction::Right, (_, right)) => right,
            }
        }

        if nodes_at.iter().all(|d| d.ends_with(&[b'Z'])) {
            break;
        }
    }

    let (steps, _) = directions_cycle.next().unwrap();

    steps
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}
