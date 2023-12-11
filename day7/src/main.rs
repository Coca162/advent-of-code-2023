#![allow(clippy::wildcard_in_or_patterns)]

use std::{
    cell::OnceCell,
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    error::Error,
    fmt::{Display, Write},
    ops::Not,
    str::FromStr,
};

static INPUT: &str = include_str!("input.txt");

fn main() {
    let mut players: Vec<Player> = INPUT
        .lines()
        .map(Player::from_str)
        .collect::<Result<_, _>>()
        .unwrap();

    players.sort_by(Player::cmp_part1);

    let part1 = sum_players(&players);

    println!("{part1}");

    players.sort_by(Player::cmp_part2);

    let part2 = sum_players(&players);

    println!("{part2}");
}

fn sum_players(players: &[Player]) -> usize {
    players
        .iter()
        .enumerate()
        .map(|(i, p)| p.bid * (i + 1))
        .sum()
}

pub struct Player {
    hand: [Card; 5],
    hand_type_part1: OnceCell<HandType>,
    hand_type_part2: OnceCell<HandType>,
    pub bid: usize,
}

impl Player {
    pub fn cmp_part1(&self, other: &Self) -> Ordering {
        self.hand_type_part1()
            .cmp(other.hand_type_part1())
            .then_with(|| self.cmp_hands_with(other, Card::cmp_part1))
    }

    pub fn cmp_part2(&self, other: &Self) -> Ordering {
        self.hand_type_part2()
            .cmp(other.hand_type_part2())
            .then_with(|| self.cmp_hands_with(other, Card::cmp_part2))
    }

    fn cmp_hands_with(&self, other: &Player, cmp: impl Fn(&Card, &Card) -> Ordering) -> Ordering {
        self.hand
            .into_iter()
            .zip(other.hand)
            .find_map(|(x, y)| Some(cmp(&x, &y)).filter(|o| o.is_eq().not()))
            .unwrap_or(Ordering::Equal)
    }

    pub fn hand_type_part1(&self) -> &HandType {
        self.hand_type_part1.get_or_init(|| {
            let freq = self
                .hand
                .iter()
                .fold(HashMap::with_capacity(5), |mut acc, c| {
                    *acc.entry(c).or_insert(0) += 1;

                    acc
                });

            let values = {
                let mut values = freq.into_values().collect::<Vec<_>>();
                values.sort();
                values
            };

            use HandType as HT;

            match values.as_slice() {
                [5] => HT::FiveOfAKind,
                [1, 4] => HT::FourOfAKind,
                [2, 3] => HT::FullHouse,
                [1, 1, 3] => HT::ThreeOfAKind,
                [1, 2, 2] => HT::TwoPair,
                [1, 1, 1, 2] => HT::OnePair,
                [1, 1, 1, 1, 1] => HT::HighCard,
                x => unreachable!("{x:?}"),
            }
        })
    }

    pub fn hand_type_part2(&self) -> &HandType {
        self.hand_type_part2.get_or_init(|| {
            let (x1, x2) = (self.hand_type_part2_better(), self.hand_type_manual_part2());
            assert_eq!(x1, x2);

            x1
        })
    }

    fn hand_type_part2_better(&self) -> HandType {
        let mut freq = self
            .hand
            .iter()
            .fold(HashMap::with_capacity(5), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;

                acc
            });

        let joker = freq.remove(&Card::J);

        let freq = {
            let mut freq = freq.into_values().collect::<Vec<_>>();
            freq.sort();

            if let Some(joker) = joker {
                if let Some(last) = freq.last_mut() {
                    *last += joker;
                } else {
                    freq.push(joker);
                }
            }

            freq
        };

        use HandType as HT;

        match freq.as_slice() {
            [5] => HT::FiveOfAKind,
            [1, 4] => HT::FourOfAKind,
            [2, 3] => HT::FullHouse,
            [1, 1, 3] => HT::ThreeOfAKind,
            [1, 2, 2] => HT::TwoPair,
            [1, 1, 1, 2] => HT::OnePair,
            [1, 1, 1, 1, 1] => HT::HighCard,
            x => unreachable!("{x:?}"),
        }
    }

    fn hand_type_manual_part2(&self) -> HandType {
        let cards = self
            .hand
            .iter()
            .fold(HashMap::with_capacity(5), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });

        let freq_freq: Vec<(u32, (bool, u32))> = cards
            .into_iter()
            .fold(BTreeMap::new(), |mut acc, (c, n)| {
                let (has_j, n) = acc.entry(n).or_default();
                *has_j |= matches!(c, Card::J);
                *n += 1;
                acc
            })
            .into_iter()
            .collect();

        use HandType as HT;

        match freq_freq.as_slice() {
            [(5, (_, 1))]
            | [(2, (true, 1)), (3, (false, 1))]
            | [(1, (true, 1)), (4, (false, 1))]
            | [(1, (false, 1)), (4, (true, 1))]
            | [(2, (false, 1)), (3, (true, 1))] => HT::FiveOfAKind,
            [(1, (false, 1)), (4, (false, 1))]
            | [(1, (true, 2)), (3, (false, 1))]
            | [(1, (false, 2)), (3, (true, 1))]
            | [(1, (false, 1)), (2, (true, 2))] => HT::FourOfAKind,
            [(2, (false, 1)), (3, (false, 1))] | [(1, (true, 1)), (2, (false, 2))] => HT::FullHouse,
            [(1, (false, 2)), (3, (false, 1))]
            | [(1, (true, 3)), (2, (false, 1))]
            | [(1, (false, 3)), (2, (true, 1))] => HT::ThreeOfAKind,
            [(1, (false, 1)), (2, (false, 2))] => HT::TwoPair,
            [(1, (false, 3)), (2, (false, 1))] | [(1, (true, 5))] => HT::OnePair,
            [(1, (false, 5))] => HT::HighCard,
            x => unreachable!("{x:?}"),
        }
    }
}

impl FromStr for Player {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards = cards.as_bytes();
        assert_eq!(cards.len(), 5);

        let hand = cards
            .iter()
            .copied()
            .map(Card::from_ascii_char)
            .collect::<Option<Vec<_>>>()
            .unwrap()
            .try_into()
            .unwrap();

        Ok(Self {
            hand,
            bid: bid.parse()?,
            hand_type_part1: Default::default(),
            hand_type_part2: Default::default(),
        })
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Hash, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    pub fn cmp_part1(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    pub fn cmp_part2(&self, other: &Self) -> Ordering {
        if matches!(self, Self::J) || matches!(other, Self::J) {
            matches!(self, Self::J)
                .not()
                .cmp(&matches!(other, Self::J).not())
        } else {
            self.cmp_part1(other)
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::T => 'T',
            Self::J => 'J',
            Self::Q => 'Q',
            Self::K => 'K',
            Self::A => 'A',
        };
        f.write_char(a)
    }
}

impl Card {
    pub fn from_ascii_char(c: u8) -> Option<Card> {
        Some(match c.to_ascii_uppercase() {
            b'2' => Self::Two,
            b'3' => Self::Three,
            b'4' => Self::Four,
            b'5' => Self::Five,
            b'6' => Self::Six,
            b'7' => Self::Seven,
            b'8' => Self::Eight,
            b'9' => Self::Nine,
            b'T' => Self::T,
            b'J' => Self::J,
            b'Q' => Self::Q,
            b'K' => Self::K,
            b'A' => Self::A,
            _ => return None,
        })
    }
}
