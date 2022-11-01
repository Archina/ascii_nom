use nom::{branch::alt, combinator::map, bytes::complete::tag, IResult};

use super::sym::{Symbol, SymbolType, SymbolSemantic};

#[derive(Debug)]
pub enum Logic{
    And,
    Or,
    Not,
    Implies,
    If,
    Iff,
    Forall,
    Exists,
    Bot,
    Top,
    Vdash,
    Models
}

pub fn parse_logic_7(i: &str) -> IResult<&str, Logic>{
    map(tag("implies"), |_| Logic::Implies)(i)
}

pub fn parse_logic_6(i: &str) -> IResult<&str, Logic>{
    alt((
        map(tag("forall"), |_| Logic::Forall),
        map(tag("exists"), |_| Logic::Exists),
        map(tag("models"), |_| Logic::Models),
    ))(i)
}

pub fn parse_logic_5(i: &str) -> IResult<&str, Logic>{
    map(tag("vdash"), |_| Logic::Vdash)(i)
}

pub fn parse_logic_3(i: &str) -> IResult<&str, Logic>{
    alt((
        map(tag("and"), |_| Logic::And),
        map(tag("not"), |_| Logic::Not),
        map(tag("neg"), |_| Logic::Not),
        map(tag("<=>"), |_| Logic::Iff),
        map(tag("iff"), |_| Logic::Iff),
        map(tag("_|_"), |_| Logic::Bot),
        map(tag("bot"), |_| Logic::Bot),
        map(tag("top"), |_| Logic::Top),
        map(tag("|--"), |_| Logic::Vdash),
        map(tag("|=="), |_| Logic::Models),
    ))(i)
}

pub fn parse_logic_2(i: &str) -> IResult<&str, Logic>{
    alt((
        map(tag("or"), |_| Logic::Or),
        map(tag("=>"), |_| Logic::Implies),
        map(tag("if"), |_| Logic::If),
        map(tag("AA"), |_| Logic::Forall),
        map(tag("EE"), |_| Logic::Exists),
        map(tag("TT"), |_| Logic::Top),
    ))(i)
}

pub fn l_2_s(logic: Logic) -> Symbol{
    Symbol{
        payload: SymbolType::Logic(logic),
        semantic: SymbolSemantic::Operator
    }
}

pub fn parse_logic(i: &str) -> IResult<&str, Logic>{
    alt((
        parse_logic_7,
        parse_logic_6,
        parse_logic_5,
        parse_logic_3,
        parse_logic_2,
    ))(i)
}

impl Logic{
    pub fn as_str(&self) -> &'static str {
        match self{
            // This block is handled differently
            Logic::And => todo!(),
            Logic::Or => todo!(),
            Logic::If => todo!(),
            Logic::Not => "&#xAC;",
            Logic::Implies => "&#x21D2;",
            Logic::Iff => "&#x21D4;",
            Logic::Forall => "&#x2200;",
            Logic::Exists => "&#x2203;",
            Logic::Bot => "&#x22A5;",
            Logic::Top => "&#x22A4;",
            Logic::Vdash => "&#x22A2;",
            Logic::Models => "&#x22A8;",
        }
    }
}