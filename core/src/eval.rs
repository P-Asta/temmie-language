use std::collections::HashMap;

use crate::token::Token;

pub fn eval(
    tokens: Vec<Token>,
    variables: HashMap<String, Token>,
    args: Option<Vec<Token>>,
) -> Token {
    let mut i = 0;
    loop {
        if i >= tokens.len() {
            return Token::Integer(0);
        }
        let token = &tokens[i];
        i += 1;
        match token {
            Token::Integer(i) => {
                return Token::Integer(*i);
            }
            Token::Float(f) => {
                return Token::Float(f.to_owned());
            }
            Token::String(s) => {
                return Token::String(s.to_owned());
            }
            Token::Boolean(b) => {
                return Token::Boolean(*b);
            }
            Token::Identifier(i) => {
                return Token::Identifier(i.to_owned());
            }
            Token::Function(name, args) => {
                for arg in args {
                    if name == "prnt" {
                        println!("{:?}", eval(arg.to_owned(), variables.clone(), None));
                    }
                }
            }
            _ => {}
        }
    }
}
