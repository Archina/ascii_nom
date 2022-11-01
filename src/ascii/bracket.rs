use nom::{branch::alt, combinator::map, bytes::complete::tag};

#[derive(Debug)]
pub enum GroupingBracket{
    Round,
    Square,
    Squirly,
    Angle,
    Ghost
}

pub fn parse_bracket_start(i: &str) -> nom::IResult<&str, GroupingBracket>{
    alt((
        map(tag("("), |_| GroupingBracket::Round),
        map(tag("["), |_| GroupingBracket::Square),
        map(tag("{"), |_| GroupingBracket::Squirly),
        map(alt((tag("(:"), tag("<<"))), |_| GroupingBracket::Angle),
        map(tag("{:"), |_| GroupingBracket::Ghost)
    ))(i)
}

pub fn parse_bracket_end(i: &str) -> nom::IResult<&str, GroupingBracket>{
    alt((
        map(tag(")"), |_| GroupingBracket::Round),
        map(tag("]"), |_| GroupingBracket::Square),
        map(tag("}"), |_| GroupingBracket::Squirly),
        map(alt((tag(":)"), tag(">>"))), |_| GroupingBracket::Angle),
        map(tag(":}"), |_| GroupingBracket::Ghost)
    ))(i)
}