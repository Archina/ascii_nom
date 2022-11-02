use nom::{branch::alt, IResult, combinator::map, bytes::complete::tag};

use super::sym::Symbol;

#[derive(Debug)]
pub enum Operator{
    Plus,
    Minus,
    Cdot,
    Ast,
    Star,
    Frontslash,
    Backslash,
    Times,
    Ltimes,
    Rtimes,
    Bowtie,
    Division,
    Circle,
    OPlus,
    OTimes,
    ODot,
    Sum,
    Product,
    Wedge,
    Bigwedge,
    Vee,
    Bigvee,
    Cap,
    BigCap,
    Cup,
    BigCup
}

pub fn parse_op_9(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("backslash"), |_| Operator::Backslash),
    )), misc_2_sym)(i)
}

pub fn parse_op_8(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("setminus"), |_| Operator::Backslash),
        map(tag("bigwedge"), |_| Operator::Bigwedge),
    )), misc_2_sym)(i)
}

pub fn parse_op_6(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("ltimes"), |_| Operator::Ltimes),
        map(tag("rtimes"), |_| Operator::Rtimes),
        map(tag("bowtie"), |_| Operator::Bowtie),
        map(tag("bigvee"), |_| Operator::Bigvee),
        map(tag("bigcap"), |_| Operator::BigCap),
        map(tag("bigcup"), |_| Operator::BigCup),
    )), misc_2_sym)(i)
}

pub fn parse_op_5(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("times"), |_| Operator::Times),
        map(tag("oplus"), |_| Operator::OPlus),
        map(tag("oplus"), |_| Operator::OTimes),
        map(tag("wedge"), |_| Operator::Wedge),
    )), misc_2_sym)(i)
}

pub fn parse_op_4(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("cdot"), |_| Operator::Cdot),
        map(tag("star"), |_| Operator::Star),
        map(tag("|><|"), |_| Operator::Bowtie),
        map(tag("circ"), |_| Operator::Circle),
        map(tag("odot"), |_| Operator::ODot),
        map(tag("prod"), |_| Operator::Product),
    )), misc_2_sym)(i)
}

pub fn parse_op_3(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("ast"), |_| Operator::Ast),
        map(tag("***"), |_| Operator::Star),
        map(tag("div"), |_| Operator::Division),
        map(tag("|><"), |_| Operator::Ltimes),
        map(tag("><|"), |_| Operator::Rtimes),
        map(tag("sum"), |_| Operator::Sum),
        map(tag("^^^"), |_| Operator::Bigwedge),
        map(tag("vee"), |_| Operator::Vee),
        map(tag("vvv"), |_| Operator::Bigvee),
        map(tag("cap"), |_| Operator::Cap),
        map(tag("nnn"), |_| Operator::BigCap),
        map(tag("cup"), |_| Operator::Cup),
        map(tag("uuu"), |_| Operator::BigCup),
    )), misc_2_sym)(i)
}

pub fn parse_op_2(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("**"), |_| Operator::Ast),
        map(tag("//"), |_| Operator::Frontslash),
        map(tag("\\\\"), |_| Operator::Backslash),
        map(tag("xx"), |_| Operator::Times),
        map(tag("-:"), |_| Operator::Division),
        map(tag("o+"), |_| Operator::OPlus),
        map(tag("ox"), |_| Operator::OTimes),
        map(tag("o."), |_| Operator::ODot),
        map(tag("^^"), |_| Operator::Wedge),
        map(tag("vv"), |_| Operator::Vee),
        map(tag("nn"), |_| Operator::Cap),
        map(tag("uu"), |_| Operator::Cup),
    )), misc_2_sym)(i)
}

pub fn parse_op_1(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("+"), |_| Operator::Plus),
        map(tag("-"), |_| Operator::Minus),
        map(tag("*"), |_| Operator::Cdot),
        map(tag("@"), |_| Operator::Circle),
    )), misc_2_sym)(i)
}

pub fn misc_2_sym(misc: Operator) -> Symbol{
    Symbol { payload: super::sym::SymbolType::Oper(misc), semantic: super::sym::SymbolSemantic::Operator }
}
