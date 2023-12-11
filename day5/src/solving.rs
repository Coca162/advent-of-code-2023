use std::cmp::Reverse;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{Almanac, MapRange};

pub fn part1(input: &Almanac) -> u32 {
    generic_solver(
        &input.mappers,
        input.seeds.iter().flat_map(|(x, y)| [x, y]).copied(),
    )
}

pub fn part2(input: &Almanac) -> u32 {
    par_generic_solver(
        &input.mappers,
        input
            .seeds
            .par_iter()
            .flat_map(|&(start, len)| start..start + len),
    )
}

fn generic_solver(mappers: &[Vec<MapRange>], seeds: impl Iterator<Item = u32>) -> u32 {
    let lowest_location = seeds
        .map(|seed| map_over_maps(mappers, seed))
        .fold(None, |lowest_location, location| {
            lowest_location.max(Some(Reverse(location)))
        });

    lowest_location.unwrap().0
}

fn par_generic_solver(mappers: &[Vec<MapRange>], seeds: impl ParallelIterator<Item = u32>) -> u32 {
    seeds
        .map(|seed| map_over_maps(mappers, seed))
        .map(|n| Some(Reverse(n)))
        .reduce(|| None, |a, b| a.max(b))
        .unwrap()
        .0
}

fn map_over_maps(mappers: &[Vec<MapRange>], seed: u32) -> u32 {
    mappers.iter().fold(seed, |prev, map| {
        map.iter().find_map(|range| range.map(prev)).unwrap_or(prev)
    })
}
