use crate::ascii::{exp::Expression, exp_inter::Inter, exp_simple::Simple, sym::Symbol};

pub trait MathMl{
    fn to_math_ml(&self) -> String;
}

impl MathMl for Expression{
    fn to_math_ml(&self) -> String {
        match self {
            Expression::Terminal(inter) => inter.to_math_ml(),
            Expression::Sequence { content, next } => todo!(),
            Expression::Frac { over, under } => format!("<mfrac>{}{}</mfrac>", over.to_math_ml(), under.to_math_ml()),
        }
    }
}

impl MathMl for Inter{
    fn to_math_ml(&self) -> String {
        match self{
            Inter::SubSup { main, sub, sup } => format!("<msubsup>{}{}{}</msubsup>", main.to_math_ml(), sub.to_math_ml(), sup.to_math_ml()),
            Inter::Sub { main, sub } => format!("<msub>{}{}</msub>", main.to_math_ml(), sub.to_math_ml()),
            Inter::Sup { main, sup } => format!("<msup>{}{}</msup>", main.to_math_ml(), sup.to_math_ml()),
            Inter::Mediate(simple) => simple.to_math_ml(),
        }
    }
}

impl MathMl for Simple{
    fn to_math_ml(&self) -> String {
        match self{
            Simple::Binary { ops, first, second } => todo!(),
            Simple::Unary { ops, content } => todo!(),
            // We can't know here if we are mo or mi - it is different for different letters... This clearly shows the shortcomings of ascii math because we have less freedom so to say...
            Simple::Syms(symbols) => format!(
                "<mrow>{}</mrow>",
                symbols.iter().fold(String::new(), |a, b| a + &b.to_math_ml())
            ),
            Simple::Grouping { content, left, right } => todo!(),
        }
    }
}

impl MathMl for Symbol{
    fn to_math_ml(&self) -> String {
        let pay = &self.payload;
        let payload: String = pay.into();
        match self.semantic{
            crate::ascii::sym::SymbolSemantic::Operator => format!("<mo>{payload}</mo>"),
            crate::ascii::sym::SymbolSemantic::Numeric => format!("<mn>{payload}</mn>"),
            crate::ascii::sym::SymbolSemantic::Identifier => format!("<mi>{payload}</mi>"),
            crate::ascii::sym::SymbolSemantic::AnnotatedText => format!("<ms>{payload}</ms>"),
        }
    }
}

