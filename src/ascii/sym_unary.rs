use nom::{branch::alt, combinator::map, bytes::complete::tag};

#[derive(Debug)]
pub enum UnaryOperator{
    Sqrt, Text, BB, Cancel, Abs, Floor, Ceil, Norm, Underbrace, Overbrace, Hat, Vec,
    // Not added yet...
    Overline, Tilde, Underline, Dot, DDot,
}

pub fn parse_u_op_9(i: &str) -> nom::IResult<&str, UnaryOperator>{
    alt((
        map(tag("underline"), |_| UnaryOperator::Underline),
    ))(i)
}

pub fn parse_u_op_8(i: &str) -> nom::IResult<&str, UnaryOperator>{
    alt((
        map(tag("overline"), |_| UnaryOperator::Overline),
    ))(i)
}

pub fn parse_u_op_6(i: &str) -> nom::IResult<&str, UnaryOperator>{
    alt((
        map(tag("cancel"), |_| UnaryOperator::Cancel),
        map(tag("ubrace"), |_| UnaryOperator::Underbrace),
        map(tag("obrace"), |_| UnaryOperator::Overbrace),
    ))(i)
}

pub fn parse_u_op_5(i: &str) -> nom::IResult<&str, UnaryOperator>{
    alt((
        map(tag("floor"), |_| UnaryOperator::Floor),
        map(tag("tilde"), |_| UnaryOperator::Tilde),
    ))(i)
}

pub fn parse_u_op_4(i: &str) -> nom::IResult<&str, UnaryOperator>{
    alt((
        map(tag("sqrt"), |_| UnaryOperator::Sqrt),
        map(tag("text"), |_| UnaryOperator::Text),
        map(tag("ceil"), |_| UnaryOperator::Ceil),
        map(tag("norm"), |_| UnaryOperator::Norm),
        map(tag("ddot"), |_| UnaryOperator::DDot),
    ))(i)
}

pub fn parse_u_op_3(i: &str) -> nom::IResult<&str, UnaryOperator>{
    alt((
        map(tag("abs"), |_| UnaryOperator::Abs),
        map(tag("hat"), |_| UnaryOperator::Hat),
        map(tag("vec"), |_| UnaryOperator::Vec),
        map(tag("bar"), |_| UnaryOperator::Overline),
        map(tag("dot"), |_| UnaryOperator::Dot),
    ))(i)
}

pub fn parse_u_op_2(i: &str) -> nom::IResult<&str, UnaryOperator>{
    alt((
        map(tag("bb"), |_| UnaryOperator::BB),
        map(tag("ul"), |_| UnaryOperator::Underline),
    ))(i)
}

pub fn parse_unary(i: &str) -> nom::IResult<&str, UnaryOperator>{
    alt((
        parse_u_op_6,
        parse_u_op_5,
        parse_u_op_4,
        parse_u_op_3,
        parse_u_op_2,
    ))(i)
}