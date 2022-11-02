use nom::{branch::alt, bytes::complete::tag, IResult, combinator::map};

use super::sym::Symbol;

#[derive(Debug)]
pub enum Relation{
    Equal,
    NotEqual,
    LessThen,
    GreaterThen,
    LessEqual,
    GreaterEqual,
    MuchLess,
    MuchGreater,
    Prec,
    PrecEq,
    Succ,
    SuccEq,
    In,
    NotIn,
    SubSet,
    SupSet,
    SubEq,
    SupEq,
    Equiv,
    Cong,
    Approx,
    Propor
}

pub fn parse_relation_8(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("subseteq"), |_| Relation::SubEq),
        map(tag("supseteq"), |_| Relation::SupEq),
    )), rel_2_sym)(i)
}

pub fn parse_relation_6(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("preceq"), |_| Relation::PrecEq),
        map(tag("succeq"), |_| Relation::SuccEq),
        map(tag("subset"), |_| Relation::SubSet),
        map(tag("supset"), |_| Relation::SupSet),
        map(tag("approx"), |_| Relation::Approx),
        map(tag("propto"), |_| Relation::Propor),
    )), rel_2_sym)(i)
}

pub fn parse_relation_5(i: &str) -> IResult<&str, Symbol>{
    map(alt((       
        map(tag("notin"), |_| Relation::NotIn),
        map(tag("equiv"), |_| Relation::Equiv),
    )), rel_2_sym)(i)
}

pub fn parse_relation_4(i: &str) -> IResult<&str, Symbol>{
    map(alt((       
        map(tag("prec"), |_| Relation::Prec),
        map(tag("succ"), |_| Relation::Succ),
        map(tag("sube"), |_| Relation::SubEq),
        map(tag("supe"), |_| Relation::SupEq),
        map(tag("cong"), |_| Relation::Cong),
        map(tag("prop"), |_| Relation::Propor),
    )), rel_2_sym)(i)
}

pub fn parse_relation_3(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        map(tag("mlt"), |_| Relation::MuchLess), // Don't work in asciimath? - should render <<
        map(tag("mgt"), |_| Relation::MuchGreater), // Don't work in asciimath? - should render >>
        map(tag("-<="), |_| Relation::PrecEq),
        map(tag(">-="), |_| Relation::SuccEq),
        map(tag("!in"), |_| Relation::NotIn),
        map(tag("sub"), |_| Relation::SubSet),
        map(tag("sup"), |_| Relation::SupSet),
    )), rel_2_sym)(i)
}

pub fn parse_relation_2(i: &str) -> IResult<&str, Symbol>{
    map(alt((        
        map(tag("!="), |_| Relation::NotEqual),
        map(tag("ne"), |_| Relation::NotEqual),
        map(tag("lt"), |_| Relation::LessThen),
        map(tag("gt"), |_| Relation::GreaterThen),
        map(tag("<="), |_| Relation::LessEqual),
        map(tag("le"), |_| Relation::LessEqual),
        map(tag(">="), |_| Relation::GreaterEqual),
        map(tag("ge"), |_| Relation::GreaterEqual),
        map(tag("ll"), |_| Relation::MuchLess), // Don't work in asciimath? - should render << &#x226A;
        map(tag("gg"), |_| Relation::MuchGreater), // Don't work in asciimath? - should render >> &#x226B;
        map(tag("-<"), |_| Relation::Prec),
        map(tag(">-"), |_| Relation::Succ),
        map(tag("in"), |_| Relation::In),
        map(tag("-="), |_| Relation::Equiv),
        map(tag("~="), |_| Relation::Cong),
        map(tag("~~"), |_| Relation::Approx),
    )), rel_2_sym)(i)
}

pub fn parse_relation_1(i: &str) -> IResult<&str, Symbol>{
    map(alt((       
        map(tag("="), |_| Relation::Equal),
        map(tag("<"), |_| Relation::LessThen),
        map(tag(">"), |_| Relation::GreaterThen),
    )), rel_2_sym)(i)
}

pub fn rel_2_sym(rel: Relation) -> Symbol{
    Symbol { payload: super::sym::SymbolType::Relation(rel), semantic: super::sym::SymbolSemantic::Operator }
}
