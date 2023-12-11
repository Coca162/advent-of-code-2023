mod parsing;
mod solving;

use solving::{part1, part2};

static INPUT: &str = include_str!("input.txt");

fn main() {
    let parsed = parsing::parse_data(INPUT).unwrap();
    let part1 = part1(&parsed);

    println!("{part1}");

    let part2 = part2(&parsed);
    println!("{part2}");
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<(u32, u32)>,
    pub mappers: Vec<Vec<MapRange>>,
}

#[derive(Debug)]
pub struct MapRange {
    pub destination: u32,
    pub source: u32,
    pub length: u32,
}

impl MapRange {
    pub fn map(&self, source: u32) -> Option<u32> {
        source
            .checked_sub(self.source)
            .filter(|&n| n < self.length)
            .map(|n| self.destination + n)
    }
}
