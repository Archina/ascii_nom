pub enum BinaryOperator{
    Frac,
    Root,
    Overset,
    Underset,
    Overbrace,
    Underbrace,
    Color
}

pub struct Binary{
    operator: BinaryOperator,
    first: Expression,
    second: Expression,
}