use std::{collections::BTreeMap, iter::Iterator};

use nom::{
    branch::alt,
    character::complete::{anychar, char, newline, space0},
    combinator::{self, all_consuming, map, verify},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    Err, IResult,
};

use crate::Direction;

type NomError<'a> = nom::Err<nom::error::Error<&'a str>>;

pub type Name = [u8; 3];
pub type LeftRight = (Name, Name);
pub type NodeRaw = (Name, LeftRight);

pub fn parse(i: &str) -> Result<(Vec<Direction>, Vec<NodeRaw>), NomError> {
    let (rest, directions) = parse_directions(i)?;

    let (_, nodes) = all_consuming(separated_list1(newline, parse_node))(rest.trim_start())?;

    Ok((directions, nodes))
}

fn parse_directions(i: &str) -> IResult<&str, Vec<Direction>> {
    many1(alt((
        map(char('L'), |_| Direction::Left),
        map(char('R'), |_| Direction::Right),
    )))(i)
}

fn parse_node(i: &str) -> IResult<&str, (Name, LeftRight)> {
    separated_pair(
        parse_element,
        delimited(space0, char('='), space0),
        separated_pair(
            preceded(char('('), parse_element),
            delimited(space0, char(','), space0),
            terminated(parse_element, char(')')),
        ),
    )(i)
}

fn parse_element(i: &str) -> IResult<&str, [u8; 3]> {
    let mut iter = combinator::iterator(i, verify(anychar, |c| c.is_ascii_alphanumeric()));

    let name: Option<[u8; 3]> = (|| {
        let mut test = iter.into_iter();
        Some([test.next()? as u8, test.next()? as u8, test.next()? as u8])
    })();

    if let Some(name) = name {
        let (rest, ()) = iter.finish()?;
        Ok((rest, name))
    } else {
        // Can be improved!!!
        Err(Err::Incomplete(nom::Needed::Unknown))
    }
}
