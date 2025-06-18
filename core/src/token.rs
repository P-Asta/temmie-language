use std::hash::Hash;

use crate::class::Class;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol {
    Equal,
    Is,
    Greater,
    Less,
    Plus,
    Minus,
    Multiply,
    Divide,
    Comma,
    Semicolon,
    Mod,
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
    Repeat(Vec<Token>),
    Return(Vec<Token>),
    Include(String),
    If(Vec<Token>),
    Else,

    None,
}

impl Token {
    pub fn change2class(&self) -> Class {
        match self {
            Token::Integer(i) => {
                let mut class = Class::new(
                    "Integer".to_string(),
                    Default::default(),
                    Default::default(),
                );
                class.add_method("!!format!!".to_string(), Token::String(i.to_string()));
                class.add_method(
                    "!!add!!".to_string(),
                    Token::Block(vec![
                        Token::Integer(i.to_owned()),
                        Token::Symbol(Symbol::Plus),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class.add_method(
                    "!!sub!!".to_string(),
                    Token::Block(vec![
                        Token::Integer(i.to_owned()),
                        Token::Symbol(Symbol::Minus),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class.add_method(
                    "!!mul!!".to_string(),
                    Token::Block(vec![
                        Token::Integer(i.to_owned()),
                        Token::Symbol(Symbol::Multiply),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class.add_method(
                    "!!div!!".to_string(),
                    Token::Block(vec![
                        Token::Integer(i.to_owned()),
                        Token::Symbol(Symbol::Divide),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class.add_method(
                    "!!mod!!".to_string(),
                    Token::Block(vec![
                        Token::Integer(i.to_owned()),
                        Token::Symbol(Symbol::Mod),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class
            }
            Token::Float(f) => {
                let mut class =
                    Class::new("Float".to_string(), Default::default(), Default::default());
                class.add_method("!!format!!".to_string(), Token::String(f.to_string()));
                class.add_method(
                    "!!add!!".to_string(),
                    Token::Block(vec![
                        Token::Float(f.to_owned()),
                        Token::Symbol(Symbol::Plus),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class.add_method(
                    "!!sub!!".to_string(),
                    Token::Block(vec![
                        Token::Float(f.to_owned()),
                        Token::Symbol(Symbol::Minus),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class.add_method(
                    "!!mul!!".to_string(),
                    Token::Block(vec![
                        Token::Float(f.to_owned()),
                        Token::Symbol(Symbol::Multiply),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class.add_method(
                    "!!div!!".to_string(),
                    Token::Block(vec![
                        Token::Float(f.to_owned()),
                        Token::Symbol(Symbol::Divide),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class.add_method(
                    "!!mod!!".to_string(),
                    Token::Block(vec![
                        Token::Float(f.to_owned()),
                        Token::Symbol(Symbol::Mod),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class
            }
            Token::String(s) => {
                let mut class =
                    Class::new("String".to_string(), Default::default(), Default::default());
                class.add_method("!!format!!".to_string(), Token::String(s.to_string()));
                class.add_method(
                    "!!add!!".to_string(),
                    Token::Block(vec![
                        Token::String(s.to_owned()),
                        Token::Symbol(Symbol::Plus),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class.add_method(
                    "!!mul!!".to_string(),
                    Token::Block(vec![
                        Token::String(s.to_owned()),
                        Token::Symbol(Symbol::Multiply),
                        Token::Identifier("rhs".to_string()),
                    ]),
                );
                class
            }
            Token::Boolean(b) => {
                let mut class = Class::new(
                    "Boolean".to_string(),
                    Default::default(),
                    Default::default(),
                );
                class.add_method(
                    "!!format!!".to_string(),
                    Token::String(
                        b.to_string()
                            .replace("true", "tru")
                            .replace("false", "falz"),
                    ),
                );
                class
            }
            _ => Class::new("".to_string(), Default::default(), Default::default()),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Token::String(s) = self {
            return write!(f, "{}", s);
        }
        if let Token::Integer(i) = self {
            return write!(f, "{}", i);
        }
        if let Token::Float(ff) = self {
            return write!(f, "{}", ff);
        }
        if let Token::Boolean(b) = self {
            if *b {
                return write!(f, "tru");
            }
            return write!(f, "falz");
        }
        if let Token::Identifier(i) = self {
            return write!(f, "{}", i);
        }
        write!(f, "{:?}", self)
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
