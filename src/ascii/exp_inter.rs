use nom::{branch::alt, sequence::tuple, bytes::complete::tag, IResult, combinator::map};

use super::exp_simple::{Simple, parse_simple};

#[derive(Debug)]
pub enum Intermediate{
    Mediate{
        main: Simple,
        pow: Option<Simple>,
        under: Option<Simple>,
    },
    Simple(Simple)
}

pub fn parse_intermediate(i: &str) -> IResult<&str, Intermediate>{
    alt((
        map(parse_simple, |s| Intermediate::Simple(s)),
        map(tuple((parse_simple, tag("_"), parse_simple)), |(main, _, under)| Intermediate::Mediate { main, under: Some(under), pow: None }),
        map(tuple((parse_simple, tag("^"), parse_simple)), |(main,_, pow)| Intermediate::Mediate { main, pow: Some(pow), under: None }),
        map(tuple((parse_simple, tag("_"), parse_simple, tag("^"), parse_simple)), |(main, _, under, _, pow)| Intermediate::Mediate { main, pow: Some(pow), under: Some(under) })
    ))(i)
}