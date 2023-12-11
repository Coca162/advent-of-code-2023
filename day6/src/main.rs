mod part1;
mod part2;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let part1: usize = part1::parse(INPUT)
        .unwrap()
        .into_iter()
        .map(count_farther_then_best)
        .product();

    println!("{part1}");

    let part2 = count_farther_then_best(part2::parse(INPUT).unwrap());
    println!("{part2}");
}

pub fn count_farther_then_best((time, best_distance): (u64, u64)) -> usize {
    (1..time)
        .map(|velocity| velocity * (time - velocity))
        .filter(|&distance| distance > best_distance)
        .count()
}
