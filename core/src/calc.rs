use std::collections::HashMap;

use crate::{
    log,
    token::{FakeFloat, Symbol, Token},
};

pub fn calc(tokens: Vec<Token>) -> Token {
    calc_with_variables(tokens, HashMap::new())
}

pub fn calc_with_variables(tokens: Vec<Token>, variables: HashMap<String, Token>) -> Token {
    if tokens.is_empty() {
        return Token::None;
    }

    // 변수 치환 먼저 처리
    let tokens: Vec<Token> = tokens
        .into_iter()
        .map(|token| match &token {
            Token::Identifier(name) => variables.get(name).cloned().unwrap_or(token),
            _ => token,
        })
        .collect();

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
                Symbol::Multiply | Symbol::Divide | Symbol::Mod => {
                    let next_token = tokens[i].clone();
                    let mut hash = HashMap::new();
                    hash.insert("rhs".to_string(), next_token);

                    current = match op {
                        Symbol::Multiply => current
                            .change2class()
                            .run_method("!!mul!!".to_string(), hash.clone()),
                        Symbol::Mod => current
                            .change2class()
                            .run_method("!!mod!!".to_string(), hash.clone()),
                        Symbol::Divide => current
                            .change2class()
                            .run_method("!!div!!".to_string(), hash.clone()),
                        _ => unreachable!(),
                    };
                }
                Symbol::Plus | Symbol::Minus | Symbol::Is | Symbol::Greater | Symbol::Less => {
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
    let mut result_tokens_is = Vec::new();
    let mut current = result_tokens[0].change2class();
    let mut i = 1;

    while i < result_tokens.len() {
        if let Token::Symbol(op) = &result_tokens[i] {
            i += 1;
            if i >= result_tokens.len() {
                break;
            }
            let next_token = result_tokens[i].clone();

            match op {
                Symbol::Plus | Symbol::Minus => {
                    let mut hash = HashMap::new();
                    hash.insert("rhs".to_string(), next_token);

                    let result = match op {
                        Symbol::Plus => current.run_method("!!add!!".to_string(), hash),
                        Symbol::Minus => current.run_method("!!sub!!".to_string(), hash),
                        _ => Token::None,
                    };
                    current = result.change2class();
                }
                Symbol::Is => {
                    result_tokens_is
                        .push(current.run_method("!!format!!".to_string(), HashMap::new()));
                    result_tokens_is.push(Token::Symbol(Symbol::Is));
                    current = next_token.change2class();
                }
                Symbol::Greater => {
                    result_tokens_is
                        .push(current.run_method("!!format!!".to_string(), HashMap::new()));
                    result_tokens_is.push(Token::Symbol(Symbol::Greater));
                    current = next_token.change2class();
                }
                Symbol::Less => {
                    result_tokens_is
                        .push(current.run_method("!!format!!".to_string(), HashMap::new()));
                    result_tokens_is.push(Token::Symbol(Symbol::Less));
                    current = next_token.change2class();
                }
                _ => {}
            }
        }
        i += 1;
    }

    let formatted = current.run_method("!!format!!".to_string(), HashMap::new());

    // is 연산 처리
    if !result_tokens_is.is_empty() {
        result_tokens_is.push(formatted);
        if result_tokens_is.len() == 3 {
            if let Token::Symbol(Symbol::Is) = result_tokens_is[1] {
                let left = &result_tokens_is[0];
                let right = &result_tokens_is[2];
                return Token::Boolean(left == right);
            }
            if let Token::Symbol(Symbol::Greater) = result_tokens_is[1] {
                let left = &result_tokens_is[0];
                let right = &result_tokens_is[2];
                if let (Token::String(l), Token::String(r)) = (left, right) {
                    if l.parse::<f64>().is_ok() && r.parse::<f64>().is_ok() {
                        return Token::Boolean(
                            l.parse::<f64>().unwrap() > r.parse::<f64>().unwrap(),
                        );
                    } else {
                        log::Logging::new("RUNTIME".to_string())
                            .error((0, 0), "> operator can only compare integers.".to_string());
                    }
                }
            }

            if let Token::Symbol(Symbol::Less) = result_tokens_is[1] {
                let left = &result_tokens_is[0];
                let right = &result_tokens_is[2];
                if let (Token::String(l), Token::String(r)) = (left, right) {
                    if l.parse::<f64>().is_ok() && r.parse::<f64>().is_ok() {
                        return Token::Boolean(
                            l.parse::<f64>().unwrap() < r.parse::<f64>().unwrap(),
                        );
                    } else {
                        log::Logging::new("RUNTIME".to_string())
                            .error((0, 0), "< operator can only compare integers.".to_string());
                    }
                }
            }
        }
        return Token::None;
    }

    // 결과 타입 변환
    match formatted {
        Token::String(s) => {
            if s == "tru" {
                return Token::Boolean(true);
            } else if s == "falz" {
                return Token::Boolean(false);
            }
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
    let mut has_is_operator = false;

    // float으로 할지 확인하는거 + is 연산자 확인
    for token in &tokens {
        let mut token = token.clone();
        if let Token::Identifier(name) = &token {
            if let Some(value) = variables.get(name) {
                token = value.to_owned();
            }
        }
        if let Token::Float(_) = token {
            has_float = true;
        }
        if let Token::Symbol(Symbol::Is) = token {
            has_is_operator = true;
        }
        if let Token::Symbol(Symbol::Greater) = token {
            has_is_operator = true;
        }
        if let Token::Symbol(Symbol::Less) = token {
            has_is_operator = true;
        }
        if !matches!(
            token,
            Token::Symbol(_) | Token::Float(_) | Token::Integer(_)
        ) {
            return Token::None;
        }
    }

    // is 연산자가 있으면 calc_with_variables 함수로 처리
    if has_is_operator {
        return calc_with_variables(tokens, variables);
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
                    Symbol::Mod => result %= next_val,
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
