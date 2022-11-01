use nom::{multi::many1, branch::alt, combinator::map, character::complete::{alpha1, digit1, anychar, one_of}, IResult};

use super::{sym_logic::{parse_logic_6, Logic, l_2_s, parse_logic_5, parse_logic_3, parse_logic_2, parse_logic_7}, sym_rel::parse_relation_6, sym_unary::parse_u_op_6, sym_op::parse_op_6};

#[derive(Debug)]
pub struct Symbol{
    pub payload: SymbolType,
    pub semantic: SymbolSemantic,
}

#[derive(Debug)]
pub enum SymbolSemantic{
    Operator,
    Numeric,
    // Something like a variable
    Identifier,
    AnnotatedText
}

#[derive(Debug)]
pub enum SymbolType{
    Alpha(char),
    Greek(super::sym_greek::Greek),
    Number(String),
    Logic(Logic),
}

impl From<&SymbolType> for String{
    fn from(val: &SymbolType) -> Self {
        match val{
            SymbolType::Alpha(char) => char.to_string(),
            SymbolType::Greek(greek) => greek.as_str().into(),
            SymbolType::Number(number) => number.clone(),
            SymbolType::Logic(logic) => logic.as_str().into(),
        }
    }
}

pub fn parse_symbol_7(i: &str) -> IResult<&str, Symbol>{
    alt((
        map(parse_logic_7, l_2_s),
        // parse_op_6,
        // parse_relation_6,
        // parse_u_op_6,
    ))(i)
}

pub fn parse_symbol_6(i: &str) -> IResult<&str, Symbol>{
    alt((
        map(parse_logic_6, l_2_s),
        // parse_op_6,
        // parse_relation_6,
        // parse_u_op_6,
    ))(i)
}

pub fn parse_symbol_5(i: &str) -> IResult<&str, Symbol>{
    alt((
        map(parse_logic_5, l_2_s),
        // parse_op_6,
        // parse_relation_6,
        // parse_u_op_6,
    ))(i)
}

pub fn parse_symbol_3(i: &str) -> IResult<&str, Symbol>{
    alt((
        map(parse_logic_3, l_2_s),
        // parse_op_6,
        // parse_relation_6,
        // parse_u_op_6,
    ))(i)
}

pub fn parse_symbol_2(i: &str) -> IResult<&str, Symbol>{
    alt((
        map(parse_logic_2, l_2_s),
        // parse_op_6,
        // parse_relation_6,
        // parse_u_op_6,
    ))(i)
}

// pub fn parse_symbol_1(i: &str) -> IResult<&str, Symbol>{
//     alt((
//         // parse_op_6,
//         // parse_relation_6,
//         // parse_u_op_6,
//     ))(i)
// }

pub fn parse_symbol(i: &str) -> nom::IResult<&str, Symbol> {
    alt((
        super::sym_greek::parse_greek,
        map(digit1, |val: &str| Symbol{
            payload: SymbolType::Number(val.to_string()),
            semantic: SymbolSemantic::Numeric
        }),
        map(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"), |val: char| Symbol{
            payload: SymbolType::Alpha(val),
            semantic: SymbolSemantic::Identifier
        }),
    ))(i)
}

pub fn parse_symbols(i: &str) -> nom::IResult<&str, Vec<Symbol>>{
    many1(parse_symbol)(i)
}