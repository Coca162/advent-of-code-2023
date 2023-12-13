use std::{
    collections::{BTreeSet, HashMap, VecDeque, BTreeMap},
    iter, mem,
    ops::Not,
    rc::{Rc, Weak},
    sync::{mpsc, Arc},
    thread,
};

use parser::{LeftRight, Name, NodeRaw};

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

pub fn part1(directions: &[Direction], nodes_raw: &[NodeRaw]) -> usize {
    let map = nodes_raw.iter().copied().collect::<HashMap<_, _>>();

    let mut left_right = map[b"AAA"];

    directions
        .iter()
        .cycle()
        .map(|d| {
            let node = match (d, left_right) {
                (Direction::Left, (left, _)) => left,
                (Direction::Right, (_, right)) => right,
            };

            left_right = map[&node];
            node
        })
        .take_while(|node| node != b"ZZZ")
        .count()
        + 1
}

pub fn part1_alternate(directions: &[Direction], nodes_raw: &[NodeRaw]) -> usize {
    let map = nodes_raw.iter().copied().collect::<HashMap<_, _>>();

    let mut directions_cycle = directions.iter().copied().cycle().enumerate();

    let (steps, _) = find_z(*b"AAA", &map, directions_cycle.by_ref());

    steps
}

pub fn part2_smarter(directions: &[Direction], nodes_raw: &[NodeRaw]) -> usize {
    let map = nodes_raw.iter().copied().collect::<HashMap<_, _>>();

    let mut nodes_at = nodes_raw
        .iter()
        .map(|(n, _)| n)
        .filter(|name| name.ends_with(&[b'A']))
        .map(|&node| get_z_cycle(node, &map, directions))
        .collect::<Vec<_>>();

    todo!()
}

pub fn get_z_cycle(
    mut node: Name,
    map: &HashMap<Name, LeftRight>,
    directions: &[Direction]
) -> (usize, Vec<usize>) {
    let initial = node;
    let mut z_found = Vec::new();

    let directions = directions.iter().copied().cycle().enumerate().map(|(i, d)| (i+1, d));

    for (step, direction) in directions {
        node = match (direction, map[&node]) {
            (Direction::Left, (left, _)) => left,
            (Direction::Right, (_, right)) => right,
        };

        if node.ends_with(&[b'Z']) {
            z_found.push(step)
        }

        if node == initial {
            return (step, z_found);
        }
    };

    unreachable!("Directions is infinite")
}

pub fn part2(directions: &[Direction], nodes_raw: &[NodeRaw]) -> usize {
    let map = nodes_raw.iter().copied().collect::<HashMap<_, _>>();

    let mut nodes_at = nodes_raw
        .iter()
        .map(|(n, _)| n)
        .filter(|name| name.ends_with(&[b'A']))
        .copied()
        .inspect(|n| println!("{:?}", unsafe { std::str::from_utf8_unchecked(n) }))
        .collect::<Vec<_>>();

    // let mut steppers = nodes_at.into_iter()
    //     .map(|mut node| {
    //         let (tx, rx) = mpsc::sync_channel(100);

    //         let map = map.clone();

    //         let directions = directions.to_owned();

    //         thread::spawn(move || {
    //             let mut directions_cycle = directions.into_iter().cycle().enumerate();

    //             loop {
    //                 let (steps, next) = find_z(node, map.as_ref(), directions_cycle.by_ref());
    //                 tx.send(steps).unwrap();
    //                 node = next;
    //             }
    //         });

    //         (rx, BTreeSet::new())
    //     })
    //     .collect::<Vec<_>>();

    // loop {
    //     let mut smallest = usize::MAX;

    //     for (rx, prev) in steppers.iter_mut() {
    //         let first = rx.recv().unwrap();

    //         smallest = smallest.min(first);

    //         prev.extend(rx.try_iter());
    //     }

    //     for (_, prev) in steppers.iter_mut() {
    //         prev.split_off(&smallest);
    //     }

    //     let intersection = steppers
    //         .iter()
    //         .map(|(_, set)| set)
    //         .cloned()
    //         .reduce(|set1, set2| &set1 & &set2).unwrap();

    //     if intersection.is_empty().not() {
    //         println!("{intersection:?}");
    //     }
    // }

    let mut directions_cycle = directions.iter().cycle().enumerate();

    // for node in nodes_at.iter_mut() {

    //     let d = find_z(*node, &map, directions_cycle.by_ref().map(|(_, d)| *d));

    //     let found = directions_cycle.next().unwrap().0;

    //     println!("{} {found}", unsafe { std::str::from_utf8_unchecked(&d) });
    // }

    for (_, direction) in directions_cycle.by_ref() {
        for node in nodes_at.iter_mut() {
            *node = match (direction, map[node]) {
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

pub fn find_z<>(
    mut node: Name,
    map: &HashMap<Name, LeftRight>,
    mut directions: impl Iterator<Item = (usize, Direction)>,
) -> (usize, Name) {
    debug_assert_eq!(directions.size_hint(), (usize::MAX, None));

    for (index, d) in directions.by_ref() {
        match (d, map[&node]) {
            (Direction::Left, (left, _)) => node = left,
            (Direction::Right, (_, right)) => node = right,
        };

        if node.ends_with(&[b'Z']) {
            return (index + 1, node);
        }
    }

    unreachable!("Directions iterator should be infinite")
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}
