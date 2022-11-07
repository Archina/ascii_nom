use nom::{branch::alt, combinator::map, bytes::complete::tag};

#[derive(Debug, Clone, Copy)]
pub enum BracketType{
    Round,
    Square,
    Squirly,
    Angle,
    Ghost,
    LFloor,
    RFloor,
    LCeil,
    RCeil,
    Norm,
    Abs,
}

#[derive(PartialEq)]
pub enum BracketState{
    Open, Close
}

pub struct Bracket{
    pub bracket: BracketType,
    pub state: BracketState
}

pub fn parse_bracket_start(i: &str) -> nom::IResult<&str, BracketType>{
    alt((
        map(tag("lceiling"), |_| BracketType::LCeil),
        map(tag("lfloor"), |_| BracketType::LFloor),
        map(tag("|__"), |_| BracketType::LFloor),
        map(tag("|~"), |_| BracketType::LCeil),
        map(alt((tag("(:"), tag("<<"))), |_| BracketType::Angle),
        map(tag("{:"), |_| BracketType::Ghost),// Brackets
        map(tag("("), |_| BracketType::Round),
        map(tag("["), |_| BracketType::Square),
        map(tag("{"), |_| BracketType::Squirly),
    ))(i)
}

pub fn parse_bracket_end(i: &str) -> nom::IResult<&str, BracketType>{
    alt((
        map(tag("rceiling"), |_| BracketType::RCeil),
        map(tag("rfloor"), |_| BracketType::RFloor),
        map(tag("__|"), |_| BracketType::RFloor),
        map(tag("~|"), |_| BracketType::RCeil),
        map(alt((tag(":)"), tag(">>"))), |_| BracketType::Angle),
        map(tag(":}"), |_| BracketType::Ghost),
        map(tag(")"), |_| BracketType::Round),
        map(tag("]"), |_| BracketType::Square),
        map(tag("}"), |_| BracketType::Squirly),
    ))(i)
}
