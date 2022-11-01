pub fn parse_bracket_start(i: &str) -> nom::IResult<&str, GroupingBracket>{
    alt((
        map(tag("("), |_| GroupingBracket::Round),
        map(tag("["), |_| GroupingBracket::Square),
        map(alt((tag("(:"), tag("<<"))), |_| GroupingBracket::Angle),
        map(tag("{:"), |_| GroupingBracket::Ghost)
    ))(i)
}

pub fn parse_bracket_end(i: &str) -> nom::IResult<&str, GroupingBracket>{
    alt((
        map(tag(")"), |_| GroupingBracket::Round),
        map(tag("]"), |_| GroupingBracket::Square),
        map(alt((tag(":)"), tag(">>"))), |_| GroupingBracket::Angle),
        map(tag(":}"), |_| GroupingBracket::Ghost)
    ))(i)
}