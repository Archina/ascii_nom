use nom::{branch::alt, sequence::{tuple, preceded}, combinator::map, bytes::complete::tag, IResult, multi::many0};

use super::{bracket::{GroupingBracket, parse_bracket_start, parse_bracket_end}, sym_unary::{parse_unary, UnaryOperator}, sym_binary::{BinaryOperator, parse_b_operator}};
use super::{sym::{parse_symbols, Symbol}, exp::{parse_expression, Expression}};

#[derive(Debug)]
pub enum Simple{
    Binary{
        ops: BinaryOperator,
        first: Box<Simple>,
        second: Box<Simple>
    },
    Unary{
        ops: UnaryOperator,
        content: Box<Simple>,
    },
    Syms(Vec<Symbol>),
    Grouping{
        content: Vec<Expression>,
        left: GroupingBracket,
        right: GroupingBracket
    }
}

#[allow(non_camel_case_types)]
pub fn parse_bSS(i: &str) -> nom::IResult<&str, Simple> {
    map(tuple((parse_b_operator, parse_simple, parse_simple)), |(ops, first, second)| Simple::Binary { ops, first: Box::new(first), second: Box::new(second) })(i)
}

pub fn parse_v(i: &str) -> nom::IResult<&str, Simple>{
    map(parse_symbols, |out| Simple::Syms(out))(i)
}

pub fn parse_uS(i: &str) -> nom::IResult<&str, Simple>{
    map(tuple((parse_unary, parse_simple)), |(ops, content)| Simple::Unary { ops, content: Box::new(content) })(i)
}

pub fn parse_Es(i: &str) -> IResult<&str, Vec<Expression>>{
    map(tuple((
        parse_expression,
        many0(
            preceded(
                tag(","),
                parse_expression
            )
        )
    )), |(head, mut tail)| {
        tail.insert(0, head);
        tail
    })(i)
}

pub fn parse_lEr(i: &str) -> IResult<&str, Simple>{
    map(tuple((parse_bracket_start, parse_Es, parse_bracket_end)), |(left, content, right)| Simple::Grouping { content, left, right })(i)
}

pub fn parse_simple(i: &str) -> nom::IResult<&str, Simple>{
    alt((
        parse_bSS,
        parse_uS,
        parse_lEr,
        parse_v,
    ))(i)
}