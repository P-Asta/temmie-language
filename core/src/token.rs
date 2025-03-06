use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol {
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    Comma,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Integer(isize),
    Float(FakeFloat),
    String(String),
    Function(String, Vec<Vec<Token>>),
    Boolean(bool),
    Identifier(String),
    Symbol(Symbol),
    Block(Vec<Token>),
    Array(Vec<Vec<Token>>),
    Repeat(Box<Token>),
    Return(Vec<Token>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FakeFloat(pub f64);

impl Eq for FakeFloat {}

impl Hash for FakeFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0.to_bits());
    }
}
