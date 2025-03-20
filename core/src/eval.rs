use std::{collections::HashMap, io::Write};

use crate::{
    calc::{calc, calc_fi},
    token::{Symbol, Token},
};

pub fn eval(tokens: Vec<Token>, mut variables: HashMap<String, Token>) -> Token {
    let mut i = 0;
    loop {
        if i >= tokens.len() {
            return Token::Integer(0);
        }
        let mut token = &tokens[i];
        if let Token::Identifier(name) = token {
            if let Some(value) = variables.get(name) {
                token = value;
            }
        }
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
            // Token::Identifier(i) => {
            //     return Token::Identifier(i.to_owned());
            // }
            Token::Symbol(s) => {
                if let Symbol::Equal = s {
                    let name = format!("{}", tokens[i - 1]);
                    i += 1;
                    let value = eval(vec![tokens[i].to_owned()], variables.clone());
                    variables.insert(name, value.clone());
                }
            }
            Token::Function(name, args) => {
                println!("var {:?}", variables);
                for arg in args {
                    if name == "prnt" {
                        let mut changed_arg = Vec::new();
                        for token in arg {
                            if let Token::Identifier(name) = token {
                                if let Some(value) = variables.get(name) {
                                    println!("{:?} -> {:?}", name, value);
                                    changed_arg.push(value.to_owned());
                                }
                            } else {
                                changed_arg.push(token.to_owned());
                            }
                        }
                        println!("{:?} -> {:?}", arg, changed_arg);
                        let calc_value = calc(changed_arg.to_owned());
                        println!("{:?}", calc_value);
                        if let Token::None = calc_value {
                            print!("{:?}", eval(changed_arg.to_owned(), variables.clone()));
                        } else {
                            print!("{:?}", calc_value);
                        }
                        std::io::stdout().flush().unwrap();
                    }
                }
                i += 1;
            }
            Token::Block(tokens) => {
                let calc_value = calc_fi(tokens.to_owned(), variables.clone());
                if let Token::None = calc_value {
                    return eval(tokens.to_owned(), variables.clone());
                } else {
                    return calc_value;
                }
            }
            _ => {
                // i += 1;
            }
        }
        i += 1;
    }
}
