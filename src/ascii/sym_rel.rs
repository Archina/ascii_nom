use nom::{branch::alt, bytes::complete::tag, IResult, combinator::map};

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

pub fn parse_relation_8(i: &str) -> IResult<&str, Relation>{
    alt((
        map(tag("subseteq"), |_| Relation::SubEq),
        map(tag("supseteq"), |_| Relation::SupEq),
    ))(i)
}

pub fn parse_relation_6(i: &str) -> IResult<&str, Relation>{
    alt((
        map(tag("preceq"), |_| Relation::PrecEq),
        map(tag("succeq"), |_| Relation::SuccEq),
        map(tag("subset"), |_| Relation::SubSet),
        map(tag("supset"), |_| Relation::SupSet),
        map(tag("approx"), |_| Relation::Approx),
        map(tag("propto"), |_| Relation::Propor),
    ))(i)
}

pub fn parse_relation_5(i: &str) -> IResult<&str, Relation>{
    alt((       
        map(tag("notin"), |_| Relation::NotIn),
        map(tag("equiv"), |_| Relation::Equiv),
    ))(i)
}

pub fn parse_relation_4(i: &str) -> IResult<&str, Relation>{
    alt((       
        map(tag("prec"), |_| Relation::Prec),
        map(tag("succ"), |_| Relation::Succ),
        map(tag("sube"), |_| Relation::SubEq),
        map(tag("supe"), |_| Relation::SupEq),
        map(tag("cong"), |_| Relation::Cong),
        map(tag("prop"), |_| Relation::Propor),
    ))(i)
}

pub fn parse_relation_3(i: &str) -> IResult<&str, Relation>{
    alt((
        map(tag("mlt"), |_| Relation::MuchLess),
        map(tag("lgt"), |_| Relation::MuchGreater),
        map(tag("-<="), |_| Relation::PrecEq),
        map(tag(">-="), |_| Relation::SuccEq),
        map(tag("!in"), |_| Relation::NotIn),
        map(tag("sub"), |_| Relation::SubSet),
        map(tag("sup"), |_| Relation::SupSet),
    ))(i)
}

pub fn parse_relation_2(i: &str) -> IResult<&str, Relation>{
    alt((        
        map(tag("!="), |_| Relation::NotEqual),
        map(tag("ne"), |_| Relation::NotEqual),
        map(tag("lt"), |_| Relation::LessThen),
        map(tag("gt"), |_| Relation::GreaterThen),
        map(tag("<="), |_| Relation::LessEqual),
        map(tag("le"), |_| Relation::LessEqual),
        map(tag(">="), |_| Relation::GreaterEqual),
        map(tag("ge"), |_| Relation::GreaterEqual),
        map(tag("ll"), |_| Relation::MuchLess),
        map(tag("gg"), |_| Relation::MuchGreater),
        map(tag("-<"), |_| Relation::Prec),
        map(tag(">-"), |_| Relation::Succ),
        map(tag("in"), |_| Relation::In),
        map(tag("-="), |_| Relation::Equiv),
        map(tag("~="), |_| Relation::Cong),
        map(tag("~~"), |_| Relation::Approx),
    ))(i)
}

pub fn parse_relation_1(i: &str) -> IResult<&str, Relation>{
    alt((       
        map(tag("="), |_| Relation::Equal),
        map(tag("<"), |_| Relation::LessThen),
        map(tag(">"), |_| Relation::GreaterThen),
    ))(i)
}

pub fn parse_relation(i: &str) -> IResult<&str, Relation>{
    alt((
        parse_relation_8,
        parse_relation_6,
        parse_relation_5,
        parse_relation_4,
        parse_relation_3,
        parse_relation_2,
        parse_relation_1,
    ))(i)
}