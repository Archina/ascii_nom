use nom::{combinator::map, bytes::complete::tag, IResult, branch::alt};

use super::sym::Symbol;

#[derive(Debug)]
pub enum MiscOperator{
    Integral,
    OIntegral,
    Partial,
    Nabla,
    PlusMinus,
    EmptySet,
    Infinity,
    Aleph,
    Therefore,
    Because,
    Dots,
    Cdots,
    Vdots,
    Ddots,
    Whitespace,
    Whitequad,
    Angle,
    Frown,
    Triangle,
    Diamond,
    Square,
    Complex,
    Natural,
    Rational,
    Real,
    Whole
}

pub fn parse_misc_9(i: &str) -> IResult<&str, Symbol>{
    map(tag("therefore"), |_| misc_2_sym(MiscOperator::Therefore))(i)
}

pub fn parse_misc_8(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("emptyset"), |_| MiscOperator::EmptySet),
        map(tag("triangle"), |_| MiscOperator::Triangle),
    )), misc_2_sym)(i)
}

pub fn parse_misc_7(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("partial"), |_| MiscOperator::Partial),
        map(tag("because"), |_| MiscOperator::Because),
        map(tag("diamond"), |_| MiscOperator::Diamond),
    )), misc_2_sym)(i)
}

pub fn parse_misc_6(i: &str) -> IResult<&str, Symbol>{
    map(tag("square"), |_| misc_2_sym(MiscOperator::Square))(i)
}

pub fn parse_misc_5(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("nabla"), |_| MiscOperator::Nabla),
        map(tag("infty"), |_| MiscOperator::Infinity),
        map(tag("aleph"), |_| MiscOperator::Aleph),
        map(tag("cdots"), |_| MiscOperator::Cdots),
        map(tag("vdots"), |_| MiscOperator::Vdots),
        map(tag("ddots"), |_| MiscOperator::Ddots),
        map(tag("angle"), |_| MiscOperator::Angle),
        map(tag("frown"), |_| MiscOperator::Frown),
    )), misc_2_sym)(i)
}

pub fn parse_misc_4(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("oint"), |_| MiscOperator::OIntegral),
        map(tag("grad"), |_| MiscOperator::Nabla),
        map(tag("dots"), |_| MiscOperator::Dots),
        map(tag("quad"), |_| MiscOperator::Whitequad),
    )), misc_2_sym)(i)
}

pub fn parse_misc_3(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("int"), |_| MiscOperator::Integral),
        map(tag("del"), |_| MiscOperator::Partial),
        map(tag("..."), |_| MiscOperator::Dots),
        map(tag("/_\\"), |_| MiscOperator::Triangle),
    )), misc_2_sym)(i)
}

pub fn parse_misc_2(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("\\ "), |_| MiscOperator::Whitespace),
        map(tag("+-"), |_| MiscOperator::PlusMinus),
        map(tag("pm"), |_| MiscOperator::PlusMinus),
        map(tag("O/"), |_| MiscOperator::EmptySet),
        map(tag("oo"), |_| MiscOperator::Infinity),
        map(tag(":."), |_| MiscOperator::Therefore),
        map(tag(":'"), |_| MiscOperator::Because),
        map(tag("/_"), |_| MiscOperator::Angle),
        map(tag("CC"), |_| MiscOperator::Complex),
        map(tag("NN"), |_| MiscOperator::Natural),
        map(tag("QQ"), |_| MiscOperator::Rational),
        map(tag("RR"), |_| MiscOperator::Real),
        map(tag("ZZ"), |_| MiscOperator::Whole),
    )), misc_2_sym)(i)
}

pub fn misc_2_sym(misc: MiscOperator) -> Symbol{
    Symbol { payload: super::sym::SymbolType::Misc(misc), semantic: super::sym::SymbolSemantic::Operator }
}