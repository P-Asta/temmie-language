#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    Comma,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(isize),
    Float(f64),
    String(String),
    Function(String, Vec<Token>),
    Boolean(bool),
    Identifier(String),
    Symbol(Symbol),
    Block(Vec<Token>),
    Repeat(isize),
}
