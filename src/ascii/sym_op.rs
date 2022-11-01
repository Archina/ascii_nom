use nom::{branch::alt, IResult, combinator::map, bytes::complete::tag};

#[derive(Debug)]
pub enum MiscOperator{
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

pub fn parse_op_9(i: &str) -> IResult<&str, MiscOperator>{
    alt((
        map(tag("backslash"), |_| MiscOperator::Backslash),
    ))(i)
}

pub fn parse_op_8(i: &str) -> IResult<&str, MiscOperator>{
    alt((
        map(tag("setminus"), |_| MiscOperator::Backslash),
        map(tag("bigwedge"), |_| MiscOperator::Bigwedge),
    ))(i)
}

pub fn parse_op_6(i: &str) -> IResult<&str, MiscOperator>{
    alt((
        map(tag("ltimes"), |_| MiscOperator::Ltimes),
        map(tag("rtimes"), |_| MiscOperator::Rtimes),
        map(tag("bowtie"), |_| MiscOperator::Bowtie),
        map(tag("bigvee"), |_| MiscOperator::Bigvee),
        map(tag("bigcap"), |_| MiscOperator::BigCap),
        map(tag("bigcup"), |_| MiscOperator::BigCup),
    ))(i)
}

pub fn parse_op_5(i: &str) -> IResult<&str, MiscOperator>{
    alt((
        map(tag("times"), |_| MiscOperator::Times),
        map(tag("oplus"), |_| MiscOperator::OPlus),
        map(tag("oplus"), |_| MiscOperator::OTimes),
        map(tag("wedge"), |_| MiscOperator::Wedge),
    ))(i)
}

pub fn parse_op_4(i: &str) -> IResult<&str, MiscOperator>{
    alt((
        map(tag("cdot"), |_| MiscOperator::Cdot),
        map(tag("star"), |_| MiscOperator::Star),
        map(tag("|><|"), |_| MiscOperator::Bowtie),
        map(tag("circ"), |_| MiscOperator::Circle),
        map(tag("odot"), |_| MiscOperator::ODot),
        map(tag("prod"), |_| MiscOperator::Product),
    ))(i)
}

pub fn parse_op_3(i: &str) -> IResult<&str, MiscOperator>{
    alt((
        map(tag("ast"), |_| MiscOperator::Ast),
        map(tag("***"), |_| MiscOperator::Star),
        map(tag("div"), |_| MiscOperator::Division),
        map(tag("|><"), |_| MiscOperator::Ltimes),
        map(tag("><|"), |_| MiscOperator::Rtimes),
        map(tag("sum"), |_| MiscOperator::Sum),
        map(tag("^^^"), |_| MiscOperator::Bigwedge),
        map(tag("vee"), |_| MiscOperator::Vee),
        map(tag("vvv"), |_| MiscOperator::Bigvee),
        map(tag("cap"), |_| MiscOperator::Cap),
        map(tag("nnn"), |_| MiscOperator::BigCap),
        map(tag("cup"), |_| MiscOperator::Cup),
        map(tag("uuu"), |_| MiscOperator::BigCup),
    ))(i)
}

pub fn parse_op_2(i: &str) -> IResult<&str, MiscOperator>{
    alt((
        map(tag("**"), |_| MiscOperator::Ast),
        map(tag("//"), |_| MiscOperator::Frontslash),
        map(tag("\\\\"), |_| MiscOperator::Backslash),
        map(tag("xx"), |_| MiscOperator::Times),
        map(tag("-:"), |_| MiscOperator::Division),
        map(tag("o+"), |_| MiscOperator::OPlus),
        map(tag("ox"), |_| MiscOperator::OTimes),
        map(tag("o."), |_| MiscOperator::ODot),
        map(tag("^^"), |_| MiscOperator::Wedge),
        map(tag("vv"), |_| MiscOperator::Vee),
        map(tag("nn"), |_| MiscOperator::Cap),
        map(tag("uu"), |_| MiscOperator::Cup),
    ))(i)
}

pub fn parse_op_1(i: &str) -> IResult<&str, MiscOperator>{
    alt((
        map(tag("+"), |_| MiscOperator::Plus),
        map(tag("-"), |_| MiscOperator::Minus),
        map(tag("*"), |_| MiscOperator::Cdot),
        map(tag("@"), |_| MiscOperator::Circle),
    ))(i)
}

pub fn parse_op(i: &str) -> IResult<&str, MiscOperator>{
    alt((
        parse_op_9,
        parse_op_8,
        parse_op_6,
        parse_op_5,
        parse_op_4,
        parse_op_3,
        parse_op_2,
        parse_op_1,
    ))(i)
}