use nom::{branch::alt, combinator::map, bytes::complete::tag, IResult};

use super::sym::Symbol;

#[derive(Debug)]
pub enum Arrow{
    UpArrow,
    DownArrow,
    RightArrow,
    To,
    RightArrowTail,
    TwoHeadedRightArrow,
    TwoHeadedRightArrowTail,
    MapsTo,
    LeftArrow,
    HorizontalArrow,
    RRightArrow,
    LLeftArrow,
    HHorizontalArrow,
}

pub fn parse_arrow_21(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("twoheadrightarrowtail"), |_| Arrow::TwoHeadedRightArrowTail),
    )), arrow_2_sym)(i)
}

pub fn parse_arrow_17(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("twoheadrightarrow"), |_| Arrow::TwoHeadedRightArrow),
    )), arrow_2_sym)(i)
}

pub fn parse_arrow_14(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("rightarrowtail"), |_| Arrow::RightArrowTail),
        map(tag("leftrightarrow"), |_| Arrow::HorizontalArrow),
        map(tag("Leftrightarrow"), |_| Arrow::HHorizontalArrow),
    )), arrow_2_sym)(i)
}

pub fn parse_arrow_10(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("rightarrow"), |_| Arrow::RightArrow),
        map(tag("Rightarrow"), |_| Arrow::RRightArrow),
    )), arrow_2_sym)(i)
}

pub fn parse_arrow_9(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("downarrow"), |_| Arrow::DownArrow),
        map(tag("leftarrow"), |_| Arrow::LeftArrow),
        map(tag("Leftarrow"), |_| Arrow::LLeftArrow),
    )), arrow_2_sym)(i)
}

pub fn parse_arrow_7(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("uparrow"), |_| Arrow::UpArrow),
    )), arrow_2_sym)(i)
}

pub fn parse_arrow_6(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("mapsto"), |_| Arrow::MapsTo),
    )), arrow_2_sym)(i)
}

pub fn parse_arrow_4(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("uarr"), |_| Arrow::UpArrow),
        map(tag("darr"), |_| Arrow::DownArrow),
        map(tag("rarr"), |_| Arrow::RightArrow),
        map(tag(">->>"), |_| Arrow::TwoHeadedRightArrowTail),
        map(tag("larr"), |_| Arrow::LeftArrow),
        map(tag("harr"), |_| Arrow::HorizontalArrow),
        map(tag("rArr"), |_| Arrow::RRightArrow),
        map(tag("lArr"), |_| Arrow::LLeftArrow),
        map(tag("hArr"), |_| Arrow::HHorizontalArrow),
    )), arrow_2_sym)(i)
}

pub fn parse_arrow_3(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag(">->"), |_| Arrow::RightArrowTail),
        map(tag("->>"), |_| Arrow::TwoHeadedRightArrow),
        map(tag("|->"), |_| Arrow::MapsTo),
    )), arrow_2_sym)(i)
}

pub fn parse_arrow_2(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("->"), |_| Arrow::To),
        map(tag("to"), |_| Arrow::To),
    )), arrow_2_sym)(i)
}

fn arrow_2_sym(arr: Arrow) -> Symbol{
    Symbol { payload: super::sym::SymbolType::Arrow(arr), semantic: super::sym::SymbolSemantic::Operator }
}
