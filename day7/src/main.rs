use std::{
    cell::OnceCell,
    cmp::Ordering,
    collections::HashMap,
    error::Error,
    fmt::{Display, Write},
    ops::Not,
    str::FromStr,
};

static INPUT: &str = include_str!("input.txt");

fn main() {
    let part1 = part1(INPUT);
    println!("{part1}");
}

pub fn part1(input: &str) -> usize {
    let mut players: Vec<Player> = input
        .lines()
        .map(Player::from_str)
        .collect::<Result<_, _>>()
        .unwrap();

    players.sort_by(|x, y| x.part1_cmp(y));

    players.iter().enumerate().for_each(|(i, x)| {
        println!("{i} {x:?}");
    });

    players
        .into_iter()
        .enumerate()
        .map(|(i, p)| p.bid * (i + 1))
        .sum()

    // todo!()
}

pub struct Player {
    hand: [Card; 5],
    hand_type: OnceCell<HandType>,
    pub bid: usize,
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Player {
            hand: [x1, x2, x3, x4, x5],
            hand_type,
            bid,
        } = self;

        // let x1 = *x1 as usize;
        // let x2 = *x2 as usize;
        // let x3 = *x3 as usize;

        if let Some(hand_type) = hand_type.get() {
            write!(f, "{hand_type:?} [{x1} {x2} {x3} {x4}{x5}] {bid}")
        } else {
            write!(f, "[{x1} {x2} {x3} {x4}{x5}] {bid}")
        }
    }
}

impl Player {
    pub fn part1_cmp(&self, other: &Self) -> Ordering {
        self.hand_type().cmp(other.hand_type()).then_with(|| {
            self.hand
                .into_iter()
                .zip(other.hand)
                .find_map(|(x, y)| Some(x.cmp(&y)).filter(|o| o.is_eq().not()))
                .unwrap_or(Ordering::Equal)
        })
    }

    pub fn hand_type(&self) -> &HandType {
        self.hand_type.get_or_init(|| {
            let cards = self
                .hand
                .into_iter()
                .fold(HashMap::with_capacity(5), |mut map, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                });

            match cards.len() {
                1 => HandType::FiveOfAKind,
                2 => match cards.values().next().unwrap() {
                    4 | 1 => HandType::FourOfAKind,
                    3 | 2 => HandType::FullHouse,
                    _ => unreachable!(),
                },
                3 => {
                    let mut values = cards.values();

                    match values.next().unwrap() {
                        3 => HandType::ThreeOfAKind,
                        2 => HandType::TwoPair,
                        1 => match values.next().unwrap() {
                            2 => HandType::TwoPair,
                            1 | 3 => HandType::ThreeOfAKind,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                }
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => unreachable!(),
            }
        })
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
            hand_type: OnceCell::default(),
            bid: bid.parse()?,
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
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    T = 9,
    J = 10,
    Q = 11,
    K = 12,
    A = 13,
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
