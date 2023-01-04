use nom::{branch::alt, sequence::tuple, bytes::complete::tag, IResult, combinator::map};

use super::exp_simple::{Simple, parse_simple};

#[derive(Debug)]
pub enum Inter{
    Sub{
        main: Simple,
        /// Index
        sub: Simple,
    },
    Sup{
        main: Simple,
        /// Power
        sup: Simple,
    },
    SubSup{
        main: Simple,
        /// Power
        sup: Simple,
        /// Index
        sub: Simple,
    },
    Mediate(Simple)
}

pub fn parse_intermediate(i: &str) -> IResult<&str, Inter>{
    alt((
        map(tuple((parse_simple, tag("_"), parse_simple, tag("^"), parse_simple)), |(main, _, sub, _, sup)| Inter::SubSup { main, sup, sub }),
        map(tuple((parse_simple, tag("_"), parse_simple)), |(main, _, sub)| Inter::Sub { main, sub }),
        map(tuple((parse_simple, tag("^"), parse_simple)), |(main,_, sup)| Inter::Sup { main, sup }),
        map(parse_simple, |s| Inter::Mediate(s)),
    ))(i)
}