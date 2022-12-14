use nom::{branch::alt, combinator::map, bytes::complete::tag};

#[derive(Debug)]
pub enum BinaryOperator{
    Frac,
    Root,
    Overset,
    Underset,
    Color,
}

pub fn parse_b_operator(i: &str) -> nom::IResult<&str, BinaryOperator>{
    alt((
        map(tag("underset"), |_| BinaryOperator::Underset),
        map(tag("overset"), |_| BinaryOperator::Overset),
        map(tag("color"), |_| BinaryOperator::Color),
        map(tag("frac"), |_| BinaryOperator::Frac),
        map(tag("root"), |_| BinaryOperator::Root),
    ))(i)
}