use crate::ascii::{exp::Expression, exp_inter::Inter, exp_simple::Simple, sym::Symbol, sym_unary::UnaryOperator, sym_binary::BinaryOperator, bracket::{BracketType, Bracket, BracketState}};

pub trait MathMl{
    fn to_math_ml(&self) -> String;
}

impl MathMl for Expression{
    fn to_math_ml(&self) -> String {
        match self {
            Self::Terminal(inter) => inter.to_math_ml(),
            Self::Sequence { content, next } => format!("{}{}", content.to_math_ml(), next.to_math_ml()),
            Self::Frac { over, under } => format!("<mfrac>{}{}</mfrac>", over.to_math_ml(), under.to_math_ml()),
        }
    }
}

impl MathMl for Inter{
    fn to_math_ml(&self) -> String {
        use crate::ascii::sym::SymbolType::Oper;
        match self{
            Self::SubSup { main, sub, sup } => {
                if let Simple::Syms(Symbol { payload: Oper(operator), .. })= main {
                    match operator{
                        crate::ascii::sym_op::Operator::Sum | crate::ascii::sym_op::Operator::Product =>
                            return format!("<munderover>{}{}{}</munderover>", main.to_math_ml(), sub.to_math_ml(), sup.to_math_ml()),
                        _ => ()
                    }
                }
                format!("<msubsup>{}{}{}</msubsup>", main.to_math_ml(), sub.to_math_ml(), sup.to_math_ml())
            },
            Self::Sub { main, sub } => {
                if let Simple::Syms(Symbol { payload: Oper(operator), .. })= main {
                    match operator{
                        crate::ascii::sym_op::Operator::Sum | crate::ascii::sym_op::Operator::Product =>
                            return format!("<munder>{}{}</munder>", main.to_math_ml(), sub.to_math_ml()),
                        _ => ()
                    }
                }
                format!("<msub>{}{}</msub>", main.to_math_ml(), sub.to_math_ml())
            },
            Self::Sup { main, sup } => {
                if let Simple::Syms(Symbol { payload: Oper(operator), .. })= main {
                    match operator{
                        crate::ascii::sym_op::Operator::Sum | crate::ascii::sym_op::Operator::Product =>
                            return format!("<mover>{}{}</mover>", main.to_math_ml(), sup.to_math_ml()),
                        _ => ()
                    }
                }
                format!("<msup>{}{}</msup>", main.to_math_ml(), sup.to_math_ml())
            },
            Self::Mediate(simple) => simple.to_math_ml(),
        }
    }
}

impl MathMl for Simple{
    fn to_math_ml(&self) -> String {
        match self{
            Self::Binary { ops, first, second } => format!("{}{}{}{}{}", ops.open(), second.to_math_ml(), first.to_math_ml(), ops.prepend().unwrap_or_default(), ops.close()),
            Self::Unary { ops, content } => format!("{}{}{}{}", ops.open(), content.to_math_ml(), ops.prepend().unwrap_or_default(), ops.close()),
            // We can't know here if we are mo or mi - it is different for different letters... This clearly shows the shortcomings of ascii math because we have less freedom so to say...
            Self::Syms(symbol) => symbol.to_math_ml(),
            Self::Grouping { content, left, right } => {
                format!("<mrow>{}{}{}</mrow>", Bracket{bracket: *left, state: BracketState::Open}.to_math_ml(), content.to_math_ml(), Bracket{bracket: *right, state: BracketState::Close}.to_math_ml())
            },
            Self::Matrix { content, left, right } => {
                let tags = (0..content[0].len()).map(|_| "none").collect::<Vec<_>>().join(" ");
                let mut column_content = String::new();
                for clm_idx in 0..content.len() {
                    let mut row_content = String::new();
                    for row_idx in 0..content[0].len(){
                        row_content = row_content + &format!("<mtd>{}</mtd>", content[clm_idx][row_idx].to_math_ml());
                    }
                    column_content = column_content + &format!("<mtr>{}</mtr>", row_content);
                }
                format!(
                    "<mrow>{}<mtable columnlines=\"{}\">{}</mtable>{}</mrow>",
                    Bracket{bracket: *left, state: BracketState::Open}.to_math_ml(),
                    tags,
                    column_content,
                    Bracket{bracket: *right, state: BracketState::Close}.to_math_ml()
                )
            }
        }
    }
}

impl MathMl for Vec<Expression>{
    fn to_math_ml(&self) -> String {
        self.iter().map(|c| c.to_math_ml()).collect::<Vec<String>>().join("<mo>,</mo>")
    }
}

impl MathMl for Box<Simple>{
    fn to_math_ml(&self) -> String {
        match self.as_ref(){
            Simple::Grouping { content, .. } => {
                format!("<mrow>{}</mrow>", content.to_math_ml())
            },
            value => value.to_math_ml()
        }
    }
}

impl BinaryOperator{
    fn open(&self) -> String{
        format!("<{}{}>", self.node(), self.attributes().map(|a| format!(" {a}")).unwrap_or_default())
    }
    fn close(&self) -> String{
        format!("</{}>", self.node())
    }
    fn node(&self) -> &'static str{
        match self{
            Self::Frac => todo!(),
            Self::Root => todo!(),
            Self::Overset => todo!(),
            Self::Underset => todo!(),
            Self::Color => todo!(),
        }
    }
    fn attributes(&self) -> Option<&'static str>{
        match self{
            _ => None
        }
    }
    fn prepend(&self) -> Option<&'static str>{
        match self {
            _ => None
        }
    }
}

impl UnaryOperator{
    fn open(&self) -> String{
        format!("<{}{}>", self.node(), self.attributes().map(|a| format!(" {a}")).unwrap_or_default())
    }
    fn close(&self) -> String{
        format!("</{}>", self.node())
    }
    fn node(&self) -> &'static str{
        match self{
            Self::Sqrt => "msqrt",
            Self::Text => "mtext",
            Self::BB => "mstyle",
            Self::Cancel => "menclose",
            Self::Underbrace | Self::Underline => "munder",
            Self::Overbrace | Self::Hat | Self::Vec | Self::Tilde | Self::Overline | Self::DDot | Self::Dot => "mover",
        }
    }
    fn attributes(&self) -> Option<&'static str>{
        match self{
            UnaryOperator::BB => Some(r#"mathvariant="bold""#),
            UnaryOperator::Cancel => Some(r#"notation="updiagonalstrike""#),
            _ => None
        }
    }
    fn prepend(&self) -> Option<&'static str>{
        match self {
            UnaryOperator::Underbrace => Some("<mo>&#x23DF;</mo>"),
            UnaryOperator::Overbrace => Some("<mo>&#x23DE;</mo>"),
            UnaryOperator::Hat => Some("<mo>^</mo>"),
            UnaryOperator::Vec => Some("<mo>&#x2192;</mo>"),
            UnaryOperator::Overline => Some("<mo>&#xAF;</mo>"),
            UnaryOperator::Tilde => Some("<mo>~</mo>"),
            UnaryOperator::Underline => Some("<mo>&#x332;</mo>"),
            UnaryOperator::Dot => Some("<mo>.</mo>"),
            UnaryOperator::DDot => Some("<mo>..</mo>"),
            _ => None
        }
    }
}

impl MathMl for Bracket{
    fn to_math_ml(&self) -> String {
        let sym = match self.bracket{
            BracketType::Round => if self.state == BracketState::Open {"("} else {")"},
            BracketType::Square => if self.state == BracketState::Open {"["} else {"]"},
            BracketType::Squirly => if self.state == BracketState::Open {"{"} else {"}"},
            BracketType::Angle => if self.state == BracketState::Open {"&#x2329;"} else {"&#x232A;"},
            BracketType::Ghost => return format!(""),
            BracketType::LFloor => "&#x230A;",
            BracketType::RFloor => "&#x230B;",
            BracketType::LCeil => "&#x2308;",
            BracketType::RCeil => "&#x2309;",
            BracketType::Norm => "&#x2225;",
            BracketType::Abs => "|",
        };
        format!("<mo>{sym}</mo>")
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

