extern crate strum;
extern crate strum_macros;

pub mod ascii;
pub mod mathml;

use std::io::stdin;

use crate::{ascii::exp::parse_expression, mathml::MathMl};

fn main() {
    // println!("{:?}", parse_expression("(123)"));
    // println!("{:?}", parse_expression("(3,4,50,x)"));
    // println!("{:?}", parse_expression("[(3,4),(3,3)]"));
    // println!("{:?}", parse_expression("text(Hey)"));
    // println!("{:?}", parse_expression("frac(50)(22:)"));
    // println!("{:?}", parse_expression("root(50){y}"));
    // println!("{:?}", parse_expression("(int) in A"));
    // println!("{:?}", parse_expression("[int:}"));
    // println!("{:?}", parse_expression("theta12huh13Theta"));
    println!("{:?}", parse_expression("thetaThetavarthetaetagamma"));
    println!("{:?}", parse_expression("alphagammaGamma/d"));

    if let Ok((_, exp)) = parse_expression("(alpha,beta,gamma)/d"){
        println!("{}", exp.to_math_ml());
    }

    if let Ok((_, exp)) = parse_expression("alpha"){
        println!("{}", exp.to_math_ml());
    }

    let mut s=String::new();
    while let Ok(_) = stdin().read_line(&mut s) {
        if let Ok((_, exp)) = parse_expression(s.trim()){
            println!("{}", exp.to_math_ml());
        } else {
            println!("Invalid input...");
        }
        s.clear();
    }
}
