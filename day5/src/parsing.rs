use nom::{
    bytes::complete::tag,
    character::{
        complete::{alpha1, digit1, newline, space1},
        streaming::space0,
    },
    combinator::{all_consuming, map_res},
    multi::{many1, separated_list1},
    sequence::{pair, preceded, tuple},
    IResult,
};

use crate::{Almanac, MapRange};

pub fn parse_data(input: &str) -> Result<Almanac, nom::Err<nom::error::Error<&str>>> {
    let (rest, seeds) = parse_seeds(input)?;

    let (_, mappers) = all_consuming(many1(preceded(pair(newline, newline), parse_mapper)))(rest)?;

    Ok(Almanac { seeds, mappers })
}

fn parse_seeds(i: &str) -> IResult<&str, Vec<(u32, u32)>> {
    preceded(
        tuple((tag("seeds:"), space0)),
        separated_list1(space1, number_pair),
    )(i)
}

fn parse_mapper(i: &str) -> IResult<&str, Vec<MapRange>> {
    let (rest, _) = tuple((alpha1, tag("-to-"), alpha1, space1, tag("map:"), newline))(i)?;

    separated_list1(newline, parse_mapper_range)(rest)
}

fn parse_mapper_range(i: &str) -> IResult<&str, MapRange> {
    let (rest, (destination, _, source, _, length)) =
        tuple((number, space1, number, space1, number))(i)?;

    Ok((
        rest,
        MapRange {
            destination,
            source,
            length,
        },
    ))
}

fn number_pair(i: &str) -> IResult<&str, (u32, u32)> {
    let (rest, (x, _, y)) = tuple((number, space1, number))(i)?;

    Ok((rest, (x, y)))
}

fn number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(i)
}
