use crate::calc::calc_with_variables;
use crate::{
    calc::{calc, calc_fi, calc_str},
    token::{Symbol, Token},
};
use std::{collections::HashMap, io::Write};

pub fn eval(tokens: Vec<Token>, mut variables: HashMap<String, Token>) -> Token {
    let mut i = 0;
    let mut last_result = Token::Integer(0);

    loop {
        if i >= tokens.len() {
            return last_result;
        }
        let mut token = &tokens[i];
        if let Token::Identifier(name) = token {
            if let Some(value) = variables.get(name) {
                token = value;
            }
        }
        match token {
            Token::Integer(val) => {
                last_result = Token::Integer(*val);
                // 다음이 Symbol이 아니면 바로 반환
                if i + 1 >= tokens.len() || !matches!(tokens[i + 1], Token::Symbol(_)) {
                    return last_result;
                }
            }
            Token::Float(f) => {
                last_result = Token::Float(f.to_owned());
                if i + 1 >= tokens.len() || !matches!(tokens[i + 1], Token::Symbol(_)) {
                    return last_result;
                }
            }
            Token::String(s) => {
                last_result = Token::String(s.to_owned());
                if i + 1 >= tokens.len() || !matches!(tokens[i + 1], Token::Symbol(_)) {
                    return last_result;
                }
            }
            Token::Boolean(b) => {
                last_result = Token::Boolean(*b);
                if i + 1 >= tokens.len() || !matches!(tokens[i + 1], Token::Symbol(_)) {
                    return last_result;
                }
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
                        last_result = value;
                    }
                } else if let Symbol::Semicolon = s {
                    // 세미콜론은 그냥 넘어감
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
                last_result = Token::Integer(0); // 함수 실행 결과
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
                    last_result = Token::Integer(0);
                } else if let Token::String(_) = tokens[0] {
                    let calc_value = calc_str(tokens.to_owned(), variables.clone());
                    if let Token::None = calc_value {
                        last_result = eval(tokens.to_owned(), variables.clone());
                    } else {
                        last_result = calc_value;
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
                            last_result = eval(tokens.to_owned(), variables.clone());
                        } else {
                            last_result = calc_with_vars;
                        }
                    } else {
                        last_result = calc_value;
                    }
                }

                // 블록이 단독으로 있으면 결과 반환
                if tokens.len() == 1 {
                    return last_result;
                }
            }
            _ => {
                // 다른 토큰들은 그냥 넘어감
            }
        }
        i += 1;
    }
}
