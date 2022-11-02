use nom::{branch::alt, combinator::map, bytes::complete::tag};

#[derive(Debug)]
pub enum GroupingBracket{
    Round,
    Square,
    Squirly,
    Angle,
    Ghost,
    LFloor,
    RFloor,
    LCeil,
    RCeil,
}

pub fn parse_bracket_start(i: &str) -> nom::IResult<&str, GroupingBracket>{
    alt((
        map(tag("("), |_| GroupingBracket::Round),
        map(tag("["), |_| GroupingBracket::Square),
        map(tag("{"), |_| GroupingBracket::Squirly),
        map(alt((tag("(:"), tag("<<"))), |_| GroupingBracket::Angle),
        map(tag("{:"), |_| GroupingBracket::Ghost),// Brackets
        map(tag("|__"), |_| GroupingBracket::LFloor),
        map(tag("lfloor"), |_| GroupingBracket::LFloor),
        map(tag("|~"), |_| GroupingBracket::LCeil),
        map(tag("lceiling"), |_| GroupingBracket::LCeil),
    ))(i)
}

pub fn parse_bracket_end(i: &str) -> nom::IResult<&str, GroupingBracket>{
    alt((
        map(tag(")"), |_| GroupingBracket::Round),
        map(tag("]"), |_| GroupingBracket::Square),
        map(tag("}"), |_| GroupingBracket::Squirly),
        map(alt((tag(":)"), tag(">>"))), |_| GroupingBracket::Angle),
        map(tag(":}"), |_| GroupingBracket::Ghost),
        map(tag("__|"), |_| GroupingBracket::RFloor),
        map(tag("rfloor"), |_| GroupingBracket::RFloor),
        map(tag("~|"), |_| GroupingBracket::RCeil),
        map(tag("rceiling"), |_| GroupingBracket::RCeil),
    ))(i)
}