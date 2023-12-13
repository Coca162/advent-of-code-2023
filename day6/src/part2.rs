use nom::{
    bytes::complete::tag_no_case,
    character::complete::{digit1, multispace0},
    combinator::{all_consuming, map_res},
    multi::fold_many1,
    sequence::{pair, preceded, terminated},
    IResult,
};

pub fn parse(i: &str) -> Result<(u64, u64), nom::Err<nom::error::Error<&str>>> {
    let (rest, times) = preceded(pair(tag_no_case("Time:"), multispace0), number)(i)?;
    let (_, distances) = all_consuming(preceded(
        pair(tag_no_case("Distance:"), multispace0),
        number,
    ))(rest.trim_start())?;

    Ok((times, distances))
}

fn number(i: &str) -> IResult<&str, u64> {
    map_res(
        fold_many1(
            terminated(digit1, multispace0),
            String::new,
            |parts, part| parts + part,
        ),
        |n| n.parse(),
    )(i)
}
