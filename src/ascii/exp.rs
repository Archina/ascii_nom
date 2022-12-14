use nom::{IResult, sequence::tuple, branch::alt, bytes::complete::tag, combinator::map};

use super::exp_inter::{Inter, parse_intermediate};

#[derive(Debug)]
pub enum Expression{
    Terminal(Inter),
    Sequence{
        content: Inter,
        next: Box<Expression>
    },
    Frac{
        over: Inter,
        under: Inter
    }
}

fn parse_e(i: &str) -> IResult<&str, Expression>{
    map(parse_intermediate, |t| Expression::Terminal(t))(i)
}

fn parse_ie(i: &str) -> IResult<&str, Expression>{
    map(tuple((parse_intermediate, parse_expression)), |(i, e)| Expression::Sequence{
        content: i,
        next: Box::new(e)
    })(i)
}

fn parse_ii(i: &str) -> IResult<&str, Expression>{
    map(tuple((parse_intermediate, tag("/"),parse_intermediate)), |(i_o, _, i_u)| Expression::Frac { over: i_o, under: i_u })(i)
}

pub fn parse_expression(i: &str) -> IResult<&str, Expression>{
    alt((
        parse_ii,
        parse_ie,
        parse_e,
    ))(i)
}