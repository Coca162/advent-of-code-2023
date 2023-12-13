use nom::{
    bytes::complete::tag_no_case,
    character::complete::{digit1, multispace0, multispace1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::{pair, preceded},
    IResult,
};

pub fn parse(i: &str) -> Result<Vec<(u64, u64)>, nom::Err<nom::error::Error<&str>>> {
    let (rest, times) = preceded(pair(tag_no_case("Time:"), multispace0), numbers)(i)?;
    let (_, distances) = all_consuming(preceded(
        pair(tag_no_case("Distance:"), multispace0),
        numbers,
    ))(rest.trim_start())?;

    assert_eq!(
        times.len(),
        distances.len(),
        "Input has different amounts of races in times and distance rows"
    );

    Ok(times.into_iter().zip(distances).collect())
}

fn numbers(i: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(multispace1, number)(i)
}

fn number(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(i)
}
