use std::str::FromStr;

pub static INPUT: &str = include_str!("input.txt");

fn main() {
    let part1 = part1(INPUT);
    let part2 = part2(INPUT);

    println!("{part1}");
    println!("{part2}");
}

pub enum Cube {
    Red,
    Green,
    Blue,
}

impl Cube {
    pub fn over_limit(&self, amount: u8) -> bool {
        match self {
            Cube::Red => amount > 12,
            Cube::Green => amount > 13,
            Cube::Blue => amount > 14,
        }
    }
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => Cube::Red,
            "green" => Cube::Green,
            "blue" => Cube::Blue,
            _ => return Err(()),
        })
    }
}

pub fn part1(input: &str) -> usize {
    let mut total = 0;

    'b: for line in input.lines() {
        let (id, rest) = parse_id(line).unwrap();
        for subset in rest.split(';') {
            for cube in subset.split(',') {
                let (amount, color) = cube[1..].split_once(' ').unwrap();
                let amount = amount.parse().unwrap();
                let over = Cube::from_str(color).unwrap().over_limit(amount);

                if over {
                    continue 'b;
                }
            }
        }

        let id: usize = id.parse().unwrap();
        total += id;
    }

    total
}

pub fn parse_id(line: &str) -> Option<(&str, &str)> {
    line.split_once(' ')?.1.split_once(':')
}

pub fn part2(input: &str) -> usize {
    let mut power = 0;

    for line in input.lines() {
        let (_, rest) = line.split_once(':').unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for subset in rest.split(';') {
            for cube in subset.split(',') {
                let (amount, color) = cube[1..].split_once(' ').unwrap();
                let amount: usize = amount.parse().unwrap();
                match Cube::from_str(color).unwrap() {
                    Cube::Red => red = red.max(amount),
                    Cube::Green => green = green.max(amount),
                    Cube::Blue => blue = blue.max(amount),
                }
            }
        }

        power += red * green * blue;
    }

    power
}
