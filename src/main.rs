extern crate strum;
#[macro_use] extern crate strum_macros;

pub mod ascii;

use ascii::{sym::Symbol, exp_simple::{parse_simple, Simple}};
use nom::{bytes::complete::tag, combinator::{self, map}, sequence::{delimited, self, tuple}, branch::alt, Parser, character::{complete::{digit1, alpha1}}, multi::many1};

use crate::ascii::{sym::parse_symbols, exp::parse_expression};

fn parse_int(i: &str) -> nom::IResult<&str, &str>{
    tag("int")(i)
}

fn parse_bracket(i: &str) -> nom::IResult<&str, (GroupingBracket, Simple, GroupingBracket)>{
    // Modifiers are abs, floor, ceil, norm
    tuple((
        parse_bracket_start, parse_simple, parse_bracket_end
    ))(i)
}

#[derive(Debug)]
pub enum GroupingBracket{
    Round,
    Square,
    Squirly,
    Angle,
    Ghost
}

#[derive(Debug)]
struct Grouping{
    open: GroupingBracket,
    close: GroupingBracket
}

fn parse_bracket_start(i: &str) -> nom::IResult<&str, GroupingBracket>{
    alt((
        map(tag("("), |_| GroupingBracket::Round),
        map(tag("["), |_| GroupingBracket::Square),
        map(tag("{"), |_| GroupingBracket::Squirly),
        map(alt((tag("(:"), tag("<<"))), |_| GroupingBracket::Angle),
        map(tag("{:"), |_| GroupingBracket::Ghost)
    ))(i)
}

fn parse_bracket_end(i: &str) -> nom::IResult<&str, GroupingBracket>{
    alt((
        map(tag(")"), |_| GroupingBracket::Round),
        map(tag("]"), |_| GroupingBracket::Square),
        map(tag("}"), |_| GroupingBracket::Squirly),
        map(alt((tag(":)"), tag(">>"))), |_| GroupingBracket::Angle),
        map(tag(":}"), |_| GroupingBracket::Ghost)
    ))(i)
}

fn main() {
    println!("{:?}", parse_expression("(123)"));
    println!("{:?}", parse_expression("(3,4,50,x)"));
    println!("{:?}", parse_expression("[(3,4),(3,3)]"));
    println!("{:?}", parse_expression("text(Hey)"));
    println!("{:?}", parse_expression("frac(50)(22:)"));
    println!("{:?}", parse_expression("root(50){y}"));
    println!("{:?}", parse_expression("(int) in A"));
    println!("{:?}", parse_expression("[int:}"));
    println!("{:?}", parse_expression("theta12huh13Theta"));
    println!("{:?}", parse_expression("thetaThetavarthetaetagamma"));
}
