use crate::calc::calc_with_variables;
use crate::{
    calc::{calc, calc_fi, calc_str},
    token::{Symbol, Token},
};
use std::{collections::HashMap, io::Write};

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
            Token::Symbol(s) => {
                if let Symbol::Equal = s {
                    let name = format!("{}", tokens[i - 1]);
                    i += 1;
                    if i < tokens.len() {
                        // 등호 다음의 한 토큰만 처리 (대부분 Block)
                        let value = eval(vec![tokens[i].clone()], variables.clone());
                        variables.insert(name.clone(), value.clone());
                        println!("Assigned {} = {:?}", name, value);
                    }
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
            Token::If(condition, _) => {
                println!("var {:?}", variables);
                // 조건 확인
                let condition_result = eval(condition.to_owned(), variables.clone());
                println!(
                    "condition {:?} ->{:?}",
                    condition.to_owned(),
                    condition_result
                );
                // 다음 토큰이 블록인지 확인
                i += 1;
                if i < tokens.len() {
                    if let Token::Block(block) = &tokens[i] {
                        if let Token::Boolean(true) = condition_result {
                            eval(block.to_owned(), variables.clone());
                        }
                    }
                }
                i += 1;
                continue;
            }
            Token::Block(tokens) => {
                println!("Processing Block: {:?}", tokens);
                if tokens.is_empty() {
                    return Token::Integer(0);
                }

                if let Token::String(_) = tokens[0] {
                    let calc_value = calc_str(tokens.to_owned(), variables.clone());
                    if let Token::None = calc_value {
                        return eval(tokens.to_owned(), variables.clone());
                    } else {
                        return calc_value;
                    }
                } else {
                    // calc_fi 대신 calc_with_variables 사용
                    let calc_value = calc_fi(tokens.to_owned(), variables.clone());
                    println!("calc_fi result: {:?}", calc_value);
                    if let Token::None = calc_value {
                        // calc가 None을 반환하면 일반 calc_with_variables 시도
                        let calc_with_vars =
                            calc_with_variables(tokens.to_owned(), variables.clone());
                        println!("calc_with_variables result: {:?}", calc_with_vars);
                        if let Token::None = calc_with_vars {
                            return eval(tokens.to_owned(), variables.clone());
                        } else {
                            return calc_with_vars;
                        }
                    } else {
                        return calc_value;
                    }
                }
            }
            _ => {
                // i += 1;
            }
        }
        i += 1;
    }
}
