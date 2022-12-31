use nom::{multi::many1, branch::alt, combinator::map, character::complete::{digit1, one_of}, IResult};

use super::{sym_logic::{parse_logic_6, Logic, parse_logic_5, parse_logic_3, parse_logic_2, parse_logic_7}, sym_rel::{parse_relation_6, Relation, parse_relation_2, parse_relation_1, parse_relation_3, parse_relation_4, parse_relation_5, parse_relation_8}, sym_op::{parse_op_6, Operator, parse_op_1, parse_op_3, parse_op_2, parse_op_8, parse_op_9, parse_op_5, parse_op_4}, sym_greek::{parse_greek_3, parse_greek_2, parse_greek_5, parse_greek_4, parse_greek_6, parse_greek_7, parse_greek_8, parse_greek_10}, sym_misc::{MiscOperator, parse_misc_2, parse_misc_3, parse_misc_4, parse_misc_5, parse_misc_6, parse_misc_7, parse_misc_8, parse_misc_9}, sym_arrow::{Arrow, parse_arrow_10, parse_arrow_9, parse_arrow_7, parse_arrow_6, parse_arrow_4, parse_arrow_3, parse_arrow_2, parse_arrow_21, parse_arrow_17, parse_arrow_14}};

#[derive(Debug)]
pub struct Symbol{
    pub payload: SymbolType,
    pub semantic: SymbolSemantic,
}

#[derive(Debug)]
pub enum SymbolSemantic{
    Operator,
    Numeric,
    /// Something like a variable
    Identifier,
    AnnotatedText
}

#[derive(Debug)]
pub enum SymbolType{
    Alpha(char),
    Greek(super::sym_greek::Greek),
    Number(String),
    Logic(Logic),
    Oper(Operator),
    Misc(MiscOperator),
    Relation(Relation),
    Arrow(Arrow)
}

impl From<&SymbolType> for String{
    fn from(val: &SymbolType) -> Self {
        match val{
            SymbolType::Alpha(char) => char.to_string(),
            SymbolType::Greek(greek) => greek.as_str().into(),
            SymbolType::Number(number) => number.clone(),
            SymbolType::Logic(logic) => logic.as_str().into(),
            SymbolType::Oper(op) => match op{
                Operator::Plus => "+",
                Operator::Minus => "-",
                Operator::Cdot => "&#x22C5;",
                Operator::Ast => "&#x2217;",
                Operator::Star => "&#x22C6;",
                Operator::Frontslash => "\\",
                Operator::Backslash => "/",
                Operator::Times => "&#xD7;",
                Operator::Ltimes => "&#x22C9;",
                Operator::Rtimes => "&#x22CA;",
                Operator::Bowtie => "&#x22C8;",
                Operator::Division => "&#xF7;",
                Operator::Circle => "&#x2218;",
                Operator::OPlus => "&#x2295;",
                Operator::OTimes => "&#x2297;",
                Operator::ODot => "&#x2299;",
                Operator::Sum => "&#x2211;",
                Operator::Product => "&#x220F;",
                Operator::Wedge => "&#x2227;",
                Operator::Bigwedge => "&#x22C0;",
                Operator::Vee => "&#x2228;",
                Operator::Bigvee => "&#x22C1;",
                Operator::Cap => "&#x2229;",
                Operator::BigCap => "&#x22C2;",
                Operator::Cup => "&#x222A;",
                Operator::BigCup => "&#x22C3;",
            }.into(),
            SymbolType::Relation(rel) => match rel {
                Relation::Equal => "=",
                Relation::NotEqual => "&#x2260;",
                Relation::LessThen => "&lt;",
                Relation::GreaterThen => "&gt;",
                Relation::LessEqual => "&#x2264;",
                Relation::GreaterEqual => "&#x2265;",
                Relation::MuchLess => todo!(),
                Relation::MuchGreater => todo!(),
                Relation::Prec => todo!(),
                Relation::PrecEq => todo!(),
                Relation::Succ => todo!(),
                Relation::SuccEq => todo!(),
                Relation::In => todo!(),
                Relation::NotIn => todo!(),
                Relation::SubSet => todo!(),
                Relation::SupSet => todo!(),
                Relation::SubEq => todo!(),
                Relation::SupEq => todo!(),
                Relation::Equiv => todo!(),
                Relation::Cong => todo!(),
                Relation::Approx => todo!(),
                Relation::Propor => todo!(),
            }.into(),
            SymbolType::Misc(misc) => todo!(),
            SymbolType::Arrow(arr) => todo!(),
        }
    }
}

pub fn parse_symbol_10(i: &str) -> IResult<&str, Symbol>{
    alt((
        parse_greek_10,
        parse_arrow_10
    ))(i)
}

pub fn parse_symbol_9(i: &str) -> IResult<&str, Symbol>{
    alt((
        parse_op_9,
        parse_misc_9,
        parse_arrow_9
    ))(i)
}

pub fn parse_symbol_8(i: &str) -> IResult<&str, Symbol>{
    alt((
        parse_greek_8,
        parse_op_8,
        parse_relation_8,
        parse_misc_8,
    ))(i)
}

pub fn parse_symbol_7(i: &str) -> IResult<&str, Symbol>{
    alt((
        parse_logic_7,
        parse_greek_7,
        parse_misc_7,
        parse_arrow_7
    ))(i)
}

pub fn parse_symbol_6(i: &str) -> IResult<&str, Symbol>{
    alt((
        parse_logic_6,
        parse_greek_6,
        parse_op_6,
        parse_relation_6,
        parse_misc_6,
        parse_arrow_6
    ))(i)
}

pub fn parse_symbol_5(i: &str) -> IResult<&str, Symbol>{
    alt((
        parse_logic_5,
        parse_greek_5,
        parse_op_5,
        parse_relation_5,
        parse_misc_5,
    ))(i)
}

pub fn parse_symbol_4(i: &str) -> IResult<&str, Symbol>{
    alt((
        parse_greek_4,
        parse_op_4,
        parse_relation_4,
        parse_misc_4,
        parse_arrow_4
    ))(i)
}

pub fn parse_symbol_3(i: &str) -> IResult<&str, Symbol>{
    alt((
        parse_logic_3,
        parse_greek_3,
        parse_op_3,
        parse_relation_3,
        parse_misc_3,
        parse_arrow_3
    ))(i)
}

pub fn parse_symbol_2(i: &str) -> IResult<&str, Symbol>{
    alt((
        parse_logic_2,
        parse_greek_2,
        parse_op_2,
        parse_relation_2,
        parse_misc_2,
        parse_arrow_2
    ))(i)
}

pub fn parse_symbol_1(i: &str) -> IResult<&str, Symbol>{
    alt((
        parse_op_1,
        parse_relation_1,
    ))(i)
}

pub fn parse_symbol(i: &str) -> nom::IResult<&str, Symbol> {
    alt((
        map(digit1, |val: &str| Symbol{
            payload: SymbolType::Number(val.to_string()),
            semantic: SymbolSemantic::Numeric
        }),
        parse_arrow_21,
        parse_arrow_17,
        parse_arrow_14,
        parse_symbol_10,
        parse_symbol_9,
        parse_symbol_8,
        parse_symbol_7,
        parse_symbol_6,
        parse_symbol_5,
        parse_symbol_4,
        parse_symbol_3,
        parse_symbol_2,
        parse_symbol_1,
        map(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"), |val: char| Symbol{
            payload: SymbolType::Alpha(val),
            semantic: SymbolSemantic::Identifier
        }),
    ))(i)
}

pub fn parse_symbols(i: &str) -> nom::IResult<&str, Vec<Symbol>>{
    many1(parse_symbol)(i)
}