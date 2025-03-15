use std::{collections::HashMap, hash::Hash};

use crate::class::Class;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol {
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    Comma,
    Semicolon,
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
    Include(String),
    If(Vec<Token>),
    None,
}

impl Token {
    fn change2class(&self) -> Class {
        if let Token::Integer(i) = self {
            let mut class = Class::new(
                "Integer".to_string(),
                Default::default(),
                Default::default(),
            );
            class.add_method("!!format!!".to_string(), Token::String(i.to_string()));
            class.add_method(
                "!!add!!".to_string(),
                Token::Block(vec![
                    Token::Identifier("x".to_string()),
                    Token::Symbol(Symbol::Plus),
                    Token::Identifier("rhs".to_string()),
                ]),
            );
            class.add_method(
                "!!sub!!".to_string(),
                Token::Block(vec![
                    Token::Identifier("x".to_string()),
                    Token::Symbol(Symbol::Minus),
                    Token::Identifier("rhs".to_string()),
                ]),
            );
            class.add_method(
                "!!mul!!".to_string(),
                Token::Block(vec![
                    Token::Identifier("x".to_string()),
                    Token::Symbol(Symbol::Multiply),
                    Token::Identifier("rhs".to_string()),
                ]),
            );
            class.add_method(
                "!!div!!".to_string(),
                Token::Block(vec![
                    Token::Identifier("x".to_string()),
                    Token::Symbol(Symbol::Divide),
                    Token::Identifier("rhs".to_string()),
                ]),
            );
        }
        Class::new("".to_string(), Default::default(), Default::default())
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FakeFloat(pub f64);

impl Eq for FakeFloat {}

impl Hash for FakeFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0.to_bits());
    }
}
impl std::fmt::Display for FakeFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
