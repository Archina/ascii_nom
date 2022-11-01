use nom::{multi::many1, branch::alt, combinator::map, character::complete::{alpha1, digit1}, bytes::complete::tag};

// use super::sym_greek::{parse_greek, Greek};

#[derive(Debug)]
pub enum Symbol{
    Alpha(String),
    Greek(super::sym_greek::Greek),
    Number(String)
}

// pub fn parse_greek(i: &str) -> nom::IResult<&str, Symbol> {
//     alt((
//         map(tag("theta"), |_| Symbol::Greek),
//         map(tag("Theta"), |_| Symbol::Greek)
//     ))(i)
// }

pub fn parse_symbol(i: &str) -> nom::IResult<&str, Symbol> {
    alt((
        super::sym_greek::parse_greek,
        map(alpha1, |val: &str| Symbol::Alpha(val.to_string())),
        map(digit1, |val: &str| Symbol::Number(val.to_string()))
    ))(i)
}

pub fn parse_symbols(i: &str) -> nom::IResult<&str, Vec<Symbol>>{
    many1(parse_symbol)(i)
}