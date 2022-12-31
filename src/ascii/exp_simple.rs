use nom::{branch::alt, sequence::{tuple, preceded, delimited, terminated}, combinator::{map, opt, map_res}, bytes::complete::tag, IResult, multi::{many0, many1, separated_list1}, Err, Parser};

use super::{bracket::{BracketType, parse_bracket_start, parse_bracket_end}, sym_unary::{parse_unary, UnaryOperator, parse_fake_unary}, sym_binary::{BinaryOperator, parse_b_operator}, sym::parse_symbol, exp_inter::Inter};
use super::{sym::Symbol, exp::{parse_expression, Expression}};

#[derive(Debug)]
pub enum Simple{
    Binary{
        ops: BinaryOperator,
        first: Box<Simple>,
        second: Box<Simple>
    },
    Unary{
        ops: UnaryOperator,
        content: Box<Simple>,
    },
    Syms(Symbol),
    Grouping{
        content: Vec<Expression>,
        left: BracketType,
        right: BracketType
    },
    Matrix{
        content: Vec<Vec<Expression>>,
        left: BracketType,
        right: BracketType
    }
}

#[allow(non_snake_case)]
pub fn parse_bSS(i: &str) -> nom::IResult<&str, Simple> {
    map(tuple((parse_b_operator, parse_simple, parse_simple)), |(ops, first, second)| Simple::Binary { ops, first: Box::new(first), second: Box::new(second) })(i)
}

pub fn parse_v(i: &str) -> nom::IResult<&str, Simple>{
    map(parse_symbol, |out| Simple::Syms(out))(i)
}

#[allow(non_snake_case)]
pub fn parse_uS(i: &str) -> nom::IResult<&str, Simple>{
    // Second argument can only contains a single symbol or a lEr, if simple == lEr then discard brackets
    alt((
        map(tuple((parse_fake_unary, parse_simple)), |(fake, simple)| {
            let (left, right) = fake;
            Simple::Grouping { content: vec![Expression::Terminal(Inter::Mediate(simple))], left, right }
        }),
        map(tuple((parse_unary, parse_simple)), |(ops, content)| Simple::Unary { ops, content: Box::new(content) })
    ))(i)
}

#[allow(non_snake_case)]
pub fn parse_Es(i: &str) -> IResult<&str, Vec<Expression>>{
    map(tuple((
        parse_expression,
        many0(
            preceded(
                tag(","),
                parse_expression
            )
        )
    )), |(head, mut tail)| {
        tail.insert(0, head);
        tail
    })(i)
}

pub fn parse_matrix(input: &str) -> IResult<&str, Simple> {
    // Parse a single row of the matrix
    fn parse_row(input: &str) -> IResult<&str, Vec<Expression>> {
        delimited(parse_bracket_start, separated_list1(tag(","), parse_expression), parse_bracket_end)
        (input)
    }

    // Parse the entire matrix
    let matrix = tuple((
        parse_bracket_start,
        many1(terminated(parse_row, opt(tag(",")))),
        parse_bracket_end,
    ));

    map_res(matrix, |(left, content, right)| {
        // Check that all rows have the same number of elements
        let num_elements = content[0].len();
        // println!("{num_elements}");
        for column in content.iter(){
            if column.len() != num_elements {
                // println!("{} - {:?}", column.len(), column);
                return Err("Matrix rows have varying length of elements.");
            }
        }
        Ok(Simple::Matrix { content, left, right })
    })(input)
}

// Introduces mrow - might also pre and append bracket - not always
#[allow(non_snake_case)]
pub fn parse_lEr(i: &str) -> IResult<&str, Simple>{
    map(tuple((parse_bracket_start, parse_Es, parse_bracket_end)), |(left, content, right)| Simple::Grouping { content, left, right })(i)
}

pub fn parse_simple(i: &str) -> nom::IResult<&str, Simple>{
    alt((
        parse_bSS,
        parse_uS,
        parse_matrix,
        parse_lEr,
        parse_v,
    ))(i)
}