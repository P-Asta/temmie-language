use crate::calc::calc_with_variables;
use crate::{
    calc::{calc, calc_fi, calc_str},
    token::{Symbol, Token},
};
use crate::{get_to_variables, log};
use ::log::debug;
use core::num;
use std::hash::Hash;
use std::{collections::HashMap, io::Write};

// 변수를 재귀적으로 치환하는 헬퍼 함수
fn substitute_variables(tokens: Vec<Token>, variables: &HashMap<String, Token>) -> Vec<Token> {
    tokens
        .into_iter()
        .map(|token| match token {
            Token::Identifier(name) => {
                if let Some(value) = variables.get(&name) {
                    value.clone()
                } else {
                    Token::Identifier(name)
                }
            }
            Token::Block(inner_tokens) => {
                Token::Block(substitute_variables(inner_tokens, variables))
            }
            Token::If(condition) => Token::If(substitute_variables(condition, variables)),
            Token::Repeat(condition) => Token::Repeat(substitute_variables(condition, variables)),
            Token::Function(name, args) => {
                let substituted_args = args
                    .into_iter()
                    .map(|arg| substitute_variables(arg, variables))
                    .collect();
                Token::Function(name, substituted_args)
            }
            _ => token,
        })
        .collect()
}

pub fn eval(
    tokens: Vec<Token>,
    mut variables: HashMap<String, Token>,
) -> (Token, HashMap<String, Token>) {
    let mut i = 0;
    let mut last_result = Token::Integer(0);

    loop {
        if i >= tokens.len() {
            return (last_result, variables);
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
                    return (last_result, variables);
                }
            }
            Token::Float(f) => {
                last_result = Token::Float(f.to_owned());
                if i + 1 >= tokens.len() || !matches!(tokens[i + 1], Token::Symbol(_)) {
                    return (last_result, variables);
                }
            }
            Token::String(s) => {
                last_result = Token::String(s.to_owned());
                if i + 1 >= tokens.len() || !matches!(tokens[i + 1], Token::Symbol(_)) {
                    return (last_result, variables);
                }
            }
            Token::Boolean(b) => {
                last_result = Token::Boolean(*b);
                if i + 1 >= tokens.len() || !matches!(tokens[i + 1], Token::Symbol(_)) {
                    return (last_result, variables);
                }
            }
            Token::Symbol(s) => {
                if let Symbol::Equal = s {
                    let name = format!("{}", tokens[i - 1]);
                    i += 1;
                    if i < tokens.len() {
                        // 등호 다음의 한 토큰만 처리 (대부분 Block)
                        let (value, updated_vars) =
                            eval(vec![tokens[i].clone()], variables.clone());
                        variables = updated_vars;
                        variables.insert(name.clone(), value.clone());
                        last_result = value.clone();

                        // 디버깅을 위한 출력
                        debug!("변수 {} = {:?}", name, value);

                        // Block 내용이 계산식인 경우 추가 디버깅
                        if let Token::Block(ref block_tokens) = tokens[i] {
                            debug!("Block 내용: {:?}", block_tokens);
                            if block_tokens.len() >= 3
                                && matches!(block_tokens[1], Token::Symbol(Symbol::Mod))
                            {
                                debug!("모듈로 연산 감지");
                            }
                            if block_tokens.contains(&Token::Symbol(Symbol::Is)) {
                                debug!("비교 연산 감지");
                            }
                        }
                    }
                } else if let Symbol::Semicolon = s {
                    // 세미콜론은 그냥 넘어감
                }
            }
            Token::Function(name, args) => {
                if name == "prnt" {
                    for arg in args {
                        let mut changed_arg = Vec::new();
                        for token in arg {
                            if let Token::Identifier(name) = token {
                                if let Some(value) = variables.get(name) {
                                    changed_arg.push(value.to_owned());
                                }
                            } else {
                                changed_arg.push(token.to_owned());
                            }
                        }
                        let calc_value = calc(changed_arg.to_owned());
                        if let Token::None = calc_value {
                            let (result, _) = eval(changed_arg.to_owned(), variables.clone());
                            print!("{}", result);
                        } else {
                            print!("{}", calc_value);
                        }
                    }
                    println!(); // prnt 함수는 출력 후 줄바꿈
                } else {
                    i += 1;
                    if i < tokens.len() {
                        if let Token::Block(block) = &tokens[i] {
                            let mut blocks = block.clone();

                            let mut n = 0;
                            for arg in args {
                                blocks = blocks
                                    .into_iter()
                                    .map(|token| {
                                        if token == arg[0] {
                                            Token::Identifier(format!("${n}"))
                                        } else {
                                            token
                                        }
                                    })
                                    .collect();
                                n += 1;
                            }
                            variables.insert(name.clone(), Token::Block(blocks));
                        }
                    } else {
                        let function = variables.get(name);

                        if let Some(Token::Block(block)) = function {
                            let mut n = 0;
                            let mut variables = variables.clone();
                            for arg in args {
                                variables
                                    .insert(format!("${n}"), Token::Block(arg.clone().to_owned()));
                                n += 1;
                            }
                            return (Token::Block(block.clone()), variables);
                        } else {
                            let log = log::Logging::new("RUNTIME".to_string());
                            log.error((0, 0), format!("Function {} not found", name));
                        }
                    }
                }
                last_result = Token::Integer(0); // 함수 실행 결과
            }
            Token::If(condition) => {
                // 조건에서 변수 치환
                let substituted_condition = substitute_variables(condition.to_owned(), &variables);

                // 조건 평가
                let condition_result = if substituted_condition.len() == 1 {
                    // 단일 토큰인 경우 (예: Boolean 변수)
                    substituted_condition[0].clone()
                } else {
                    // 복합 조건인 경우 계산
                    let calc_result =
                        calc_with_variables(substituted_condition.clone(), variables.clone());
                    if let Token::None = calc_result {
                        let (result, updated_vars) = eval(substituted_condition, variables.clone());
                        variables = updated_vars;
                        result
                    } else {
                        calc_result
                    }
                };

                debug!("If 조건 결과: {:?}", condition_result);

                // 다음 토큰이 블록인지 확인
                i += 1;
                if i < tokens.len() {
                    if let Token::Block(block) = &tokens[i] {
                        // 조건이 true인지 확인 (Boolean(true) 또는 Integer가 아닌 0이 아닌 값)
                        let should_execute = match condition_result {
                            Token::Boolean(true) => true,
                            Token::Integer(n) if n != 0 => true, // 0이 아닌 정수는 true
                            _ => false,
                        };

                        if should_execute {
                            debug!("If 블록 실행");
                            let (_, block_vars) = eval(block.to_owned(), variables.clone());
                            variables = block_vars;
                        } else {
                            debug!("If 조건이 false, 블록 건너뜀");
                        }
                    }
                }
                i += 1;
                // Else 처리 로직도 수정
                let should_check_else = match condition_result {
                    Token::Boolean(false) => true,
                    Token::Integer(0) => true, // 0은 false로 간주
                    _ => false,
                };

                if should_check_else {
                    // Else 블록이 있는지 확인
                    if i < tokens.len() && matches!(tokens.get(i), Some(Token::Else)) {
                        i += 1; // Else 토큰을 건너뜀
                        if i < tokens.len() {
                            if let Token::Block(block) = &tokens[i] {
                                let (_, block_vars) = eval(block.to_owned(), variables.clone());
                                variables = block_vars;
                            }
                        }
                        i += 1; // Block 다음 토큰으로 이동
                    }
                }
                continue;
            }

            Token::Repeat(condition) => {
                let condition_result = calc_with_variables(condition.to_owned(), variables.clone());

                i += 1;
                if i < tokens.len() {
                    if let Token::Block(block) = &tokens[i] {
                        if let Token::Integer(num) = condition_result {
                            for iteration in 0..num {
                                debug!("Repeat 반복 {}", iteration);
                                // eval을 사용하여 블록 실행 및 변수 업데이트
                                // 중요: 이전 반복의 변수 상태를 유지해야 함
                                let (_, updated_vars) = eval(block.clone(), variables.clone());
                                variables = updated_vars; // 변수 상태 업데이트
                            }
                        } else {
                            let log = log::Logging::new("RUNTIME".to_string());
                            log.error(
                                (0, 0),
                                "ONLY NUMBRZ CAN G0 IN REP3T!! NO FUNNY STUFF ALLOWED!!!"
                                    .to_string(),
                            );
                        }
                    }
                }
                i += 1;
                continue;
            }
            Token::Block(tokens) => {
                let old_tokens = tokens.clone();
                let mut processed_tokens = Vec::new();

                // 함수 처리
                for token in old_tokens {
                    if let Token::Function(name, args) = token {
                        let (result, updated_vars) = eval(
                            vec![Token::Function(name.to_owned(), args.clone())],
                            variables.clone(),
                        );
                        variables = updated_vars;
                        processed_tokens.push(result);
                    } else {
                        processed_tokens.push(token);
                    }
                }

                if processed_tokens.is_empty() {
                    last_result = Token::Integer(0);
                } else if let Token::String(_) = processed_tokens[0] {
                    let calc_value = calc_str(processed_tokens.to_owned(), variables.clone());
                    if let Token::None = calc_value {
                        let (result, updated_vars) =
                            eval(processed_tokens.to_owned(), variables.clone());
                        variables = updated_vars;
                        last_result = result;
                    } else {
                        last_result = calc_value;
                    }
                } else {
                    // 먼저 변수 치환을 시도
                    let substituted_tokens =
                        substitute_variables(processed_tokens.clone(), &variables);
                    debug!("Block 처리 - 원본: {:?}", processed_tokens);
                    debug!("Block 처리 - 치환 후: {:?}", substituted_tokens);

                    // calc_fi 시도
                    let calc_value = calc_fi(substituted_tokens.clone(), variables.clone());
                    debug!("calc_fi 결과: {:?}", calc_value);

                    if let Token::None = calc_value {
                        // calc_with_variables 시도
                        let calc_with_vars =
                            calc_with_variables(substituted_tokens.clone(), variables.clone());
                        debug!("calc_with_variables 결과: {:?}", calc_with_vars);

                        if let Token::None = calc_with_vars {
                            // calc 시도
                            let basic_calc = calc(substituted_tokens.clone());
                            debug!("calc 결과: {:?}", basic_calc);

                            if let Token::None = basic_calc {
                                let (result, updated_vars) =
                                    eval(substituted_tokens.clone(), variables.clone());
                                variables = updated_vars;
                                last_result = result;
                            } else {
                                last_result = basic_calc;
                            }
                        } else {
                            last_result = calc_with_vars;
                        }
                    } else {
                        last_result = calc_value;
                    }
                }

                // 블록이 단독으로 있으면 결과 반환
                if processed_tokens.len() == 1 {
                    return (last_result, variables);
                }
            }
            _ => {
                // 다른 토큰들은 그냥 넘어감
            }
        }
        i += 1;
    }
}
