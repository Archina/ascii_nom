extern crate strum;
extern crate strum_macros;

pub mod ascii;
pub mod mathml;

use crate::{ascii::{exp::parse_expression, sym::{Symbol, parse_symbol}, sym_greek::{Greek, parse_greek}}, mathml::MathMl};

fn main() {
    println!("{:?}", parse_expression("(123)"));
    println!("{:?}", parse_expression("(3,4,50,x)"));
    println!("{:?}", parse_expression("[(3,4),(3,3)]"));
    println!("{:?}", parse_expression("text(Hey)"));
    println!("{:?}", parse_expression("frac(50)(22:)"));
    println!("{:?}", parse_expression("root(50){y}"));
    println!("{:?}", parse_expression("(int) in A"));
    println!("{:?}", parse_expression("[int:}"));
    println!("{:?}", parse_expression("theta12huh13Theta"));
    println!("{:?}", parse_expression("thetaThetavarthetaetagamma"));
    println!("{:?}", parse_expression("alphagammaGamma/d"));

    if let Ok((_, exp)) = parse_expression("(alpha)gammaGamma/d"){
        println!("{}", exp.to_math_ml());
    }

    if let Ok((_, exp)) = parse_symbol("alpha"){
        println!("{}", exp.to_math_ml());
    }
}
