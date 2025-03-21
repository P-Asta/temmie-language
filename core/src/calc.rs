use std::collections::HashMap;

use crate::token::{FakeFloat, Symbol, Token};

pub fn calc(tokens: Vec<Token>) -> Token {
    if tokens.is_empty() {
        return Token::None;
    }

    // 먼저 곱셈/나눗셈 처리
    let mut result_tokens = Vec::new();
    let mut current = tokens[0].clone();
    let mut i = 1;

    while i < tokens.len() {
        if let Token::Symbol(op) = &tokens[i] {
            i += 1;
            if i >= tokens.len() {
                result_tokens.push(current.clone());
                break;
            }

            match op {
                Symbol::Multiply | Symbol::Divide => {
                    let next_token = tokens[i].clone();
                    let mut hash = HashMap::new();
                    hash.insert("rhs".to_string(), next_token);

                    current = match op {
                        Symbol::Multiply => current
                            .change2class()
                            .run_method("!!mul!!".to_string(), hash.clone()),
                        Symbol::Divide => current
                            .change2class()
                            .run_method("!!div!!".to_string(), hash.clone()),
                        _ => unreachable!(),
                    };
                }
                Symbol::Plus | Symbol::Minus => {
                    result_tokens.push(current);
                    result_tokens.push(Token::Symbol(op.clone()));
                    current = tokens[i].clone();
                }
                _ => {}
            }
        }
        i += 1;
    }
    result_tokens.push(current);

    // 그 다음 덧셈/뺄셈 처리
    let mut current = result_tokens[0].change2class();
    let mut i = 1;

    while i < result_tokens.len() {
        if let Token::Symbol(op) = &result_tokens[i] {
            i += 1;
            if i >= result_tokens.len() {
                break;
            }
            let next_token = result_tokens[i].clone();

            let mut hash = HashMap::new();
            hash.insert("rhs".to_string(), next_token);

            let result = match op {
                Symbol::Plus => current.run_method("!!add!!".to_string(), hash),
                Symbol::Minus => current.run_method("!!sub!!".to_string(), hash),
                _ => Token::None,
            };
            current = result.change2class();
        }
        i += 1;
    }

    match current.run_method("!!format!!".to_string(), HashMap::new()) {
        Token::String(s) => {
            if let Ok(i) = s.parse::<isize>() {
                Token::Integer(i)
            } else if let Ok(f) = s.parse::<f64>() {
                Token::Float(FakeFloat(f))
            } else {
                Token::String(s)
            }
        }
        t => t,
    }
}

pub fn calc_str(tokens: Vec<Token>, variables: HashMap<String, Token>) -> Token {
    // 토큰 유효성 검사 및 변수 치환
    let tokens: Vec<Token> = tokens
        .into_iter()
        .map(|token| match &token {
            Token::Identifier(name) => variables.get(name).cloned().unwrap_or(token),
            _ => token,
        })
        .collect();

    // 한개만있으면 반환
    if tokens.len() == 1 {
        return match &tokens[0] {
            Token::String(s) => Token::String(s.clone()),
            _ => Token::None,
        };
    }

    if tokens.len() == 3 {
        match (&tokens[0], &tokens[1], &tokens[2]) {
            // 문자열 곱셈 처리하는거
            (Token::String(s), Token::Symbol(Symbol::Multiply), Token::Integer(n)) => {
                if *n >= 0 {
                    return Token::String(s.repeat(*n as usize));
                }
                return Token::String(String::new());
            }
            // 문자열 덧셈 처리하는거
            (Token::String(s1), Token::Symbol(Symbol::Plus), Token::String(s2)) => {
                let mut result = s1.clone();
                result.push_str(s2);
                return Token::String(result);
            }
            _ => {}
        }
    }

    Token::None
}

pub fn calc_fi(tokens: Vec<Token>, variables: HashMap<String, Token>) -> Token {
    let mut current_term = Vec::new();
    let mut terms = Vec::new();
    let mut operators = Vec::new();
    let mut has_float = false;

    // float으로 할지 확인하는거
    for token in &tokens {
        let mut token = token.clone();
        if let Token::Identifier(name) = &token {
            if let Some(value) = variables.get(name) {
                token = value.to_owned();
            }
        }
        if let Token::Float(_) = token {
            has_float = true;
            break;
        }
        if !matches!(
            token,
            Token::Symbol(_) | Token::Float(_) | Token::Integer(_)
        ) {
            return Token::None;
        }
    }

    // 항으로 나누는거
    for token in tokens {
        let mut token = token.clone();
        if let Token::Identifier(name) = &token {
            if let Some(value) = variables.get(name) {
                token = value.to_owned();
            }
        }
        match &token {
            Token::Symbol(Symbol::Plus) | Token::Symbol(Symbol::Minus) => {
                terms.push(current_term.clone());
                operators.push(token);
                current_term.clear();
            }
            _ => current_term.push(token),
        }
    }
    terms.push(current_term);

    // 곱셈 나눔셈 하는거
    let mut processed_terms = Vec::new();
    for term in terms {
        if term.is_empty() {
            continue;
        }

        let mut result = match &term[0] {
            Token::Integer(n) => *n as f64,
            Token::Float(f) => f.0,
            _ => 0.0,
        };

        let mut i = 1;
        while i < term.len() {
            if let Token::Symbol(symbol) = &term[i] {
                let next_val = match &term[i + 1] {
                    Token::Integer(n) => *n as f64,
                    Token::Float(f) => f.0,
                    _ => 0.0,
                };

                match symbol {
                    Symbol::Multiply => result *= next_val,
                    Symbol::Divide => result /= next_val,
                    _ => (),
                }
                i += 2;
            }
        }
        processed_terms.push(result);
    }

    // 덧셈 뺄셈 하는거
    let mut final_result = processed_terms[0];
    for (i, op) in operators.iter().enumerate() {
        if let Token::Symbol(symbol) = op {
            match symbol {
                Symbol::Plus => final_result += processed_terms[i + 1],
                Symbol::Minus => final_result -= processed_terms[i + 1],
                _ => (),
            }
        }
    }

    // 타입 맞춰서 반환
    if has_float {
        Token::Float(FakeFloat(final_result))
    } else {
        Token::Integer(final_result as isize)
    }
}
