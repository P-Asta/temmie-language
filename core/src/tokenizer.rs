use crate::log;
use crate::token;
use crate::token::*;

fn remove_comma(
    args: Vec<Token>,
    reading_y: usize,
    arg_start: usize,
    start: usize,
    log: &log::Logging,
) -> Vec<Vec<Token>> {
    let mut result_args = vec![];
    let mut remove_comma_count = 0;
    let mut arg = vec![];
    for i in 0..args.len() {
        if let Token::Symbol(Symbol::Comma) = args[i] {
            if arg.len() == 0 {
                log.error(
                    (reading_y, arg_start - start + remove_comma_count),
                    "INVAL!D C0MMA: UNEXPEKTED C0MMA!! H0W DID DIS HAPPEN??".to_string(),
                );
            }
            result_args.push(arg);
            arg = vec![];
            remove_comma_count += 1;
        } else {
            arg.push(args[i].clone());
        }
    }
    result_args.push(arg);
    result_args
}

pub fn tokenizer(path: String, code: Vec<char>) -> Vec<Token> {
    let mut i = 0;
    let mut reading_x = 1;
    let mut reading_y = 1;
    let mut tokens = Vec::new();
    let log = log::Logging::new(path.clone());
    'main: loop {
        let c = code[i];
        reading_x += 1;
        if c == '\0' {
            break 'main;
        }
        if c == ' ' {
            i += 1;
            continue;
        }
        if c == ';' {
            i += 1;
            tokens.push(Token::Symbol(Symbol::Semicolon));
            continue;
        }
        if c == '\n' {
            i += 1;
            reading_y += 1;
            reading_x = 1;
            continue;
        }
        if c.is_numeric() {
            let start = i;
            let mut dot_cnt = 0;
            'sub: loop {
                let c = code[i];
                if c == '\0' {
                    break 'sub;
                }
                if c == '.' {
                    dot_cnt += 1;
                    i += 1;
                    continue 'sub;
                } else if c.is_numeric() {
                    i += 1;
                    continue 'sub;
                } else {
                    break 'sub;
                }
            }

            let num_str: String = code[start..i].iter().collect();
            if dot_cnt == 0 {
                tokens.push(Token::Integer(num_str.parse().unwrap()));
                continue 'main;
            }
            if dot_cnt == 1 {
                tokens.push(Token::Float(FakeFloat(num_str.parse().unwrap())));
                continue 'main;
            } else {
                log.error(
                    (reading_y, reading_x),
                    format!(
                        "INVAL!D NUMBR FORM@T: '{num_str}' H@S MULTIPLE DECIM@L P0INTS!! WHAT??"
                    ),
                );
            }
        }
        if c == '"' {
            let start = i + 1;
            'sub: loop {
                i += 1;
                let c = code[i];
                if c == '\0' {
                    log.error(
                        (reading_y, reading_x),
                        "UNT3RM!N@TED STR!NG L!TER@L: M!SS!NG CLOS!NG QU0TE!! OH NOO!!".to_string(),
                    );
                }
                if c == '"' {
                    break 'sub;
                }
            }
            let str_value = code[start..i].iter().collect();
            tokens.push(Token::String(str_value));
            i += 1;
        }

        if c == '{' {
            let block_start = i + 1;
            let block_end;
            let mut block_count = 1;
            'sub: loop {
                i += 1;
                let c = code[i];
                if c == '\0' {
                    log.error(
                        (reading_y, reading_x),
                        "UNT3RM!N@TED BL0CK: M!SS!NG CLOS!NG BR@C3 '}'!! OOPSIE!!".to_string(),
                    );
                }
                if c == '{' {
                    block_count += 1;
                }
                if c == '}' {
                    block_count -= 1;
                    if block_count == 0 {
                        block_end = i;
                        break 'sub;
                    }
                }
            }
            let mut block_code = code[block_start..block_end].to_vec();
            block_code.push('\0');
            let block_token = tokenizer(path.clone(), block_code);
            tokens.push(Token::Block(block_token));
            i += 1;
        }
        if c == '[' {
            let array_start = i + 1;
            let array_end;
            let mut array_count = 1;
            'sub: loop {
                i += 1;
                let c = code[i];
                if c == '\0' {
                    log.error(
                        (reading_y, reading_x),
                        "UNT3RM!N@TED @RR@Y: M!SS!NG CLOS!NG BR@CKET ']'!! NOO!!".to_string(),
                    );
                }
                if c == '[' {
                    array_count += 1;
                }
                if c == ']' {
                    array_count -= 1;
                    if array_count == 0 {
                        array_end = i;
                        break 'sub;
                    }
                }
            }
            let mut array_code = code[array_start..array_end].to_vec();
            array_code.push('\0');
            let array_token = remove_comma(
                tokenizer(path.clone(), array_code),
                reading_y,
                array_start,
                0,
                &log,
            );
            tokens.push(Token::Array(array_token));
            i += 1;
        }

        if c == 't' || c == 'f' {
            let start = i;
            'sub: loop {
                i += 1;
                let c = code[i];
                if c == '\0' {
                    break 'sub;
                }
                if c.is_alphabetic() || c == '_' {
                    continue 'sub;
                } else {
                    break 'sub;
                }
            }
            let code_str: String = code[start..i].iter().collect();
            if code_str == "tru" {
                tokens.push(Token::Boolean(true));
            } else if code_str == "falz" {
                tokens.push(Token::Boolean(false));
            } else {
                i = start;
            }
        }

        if c == 'r' {
            let start = i;
            'sub: loop {
                i += 1;
                let c = code[i];
                if c == '\0' {
                    break 'sub;
                }
                if c.is_alphabetic() {
                    continue 'sub;
                } else {
                    break 'sub;
                }
            }
            let code_str: String = code[start..i].iter().collect();
            if code_str == "retun" {
                let start = i + 1;
                'sub: loop {
                    i += 1;
                    let c = code[i];
                    if c == '\0' {
                        break 'sub;
                    }
                    if c == '\n' {
                        break 'sub;
                    }
                }
                let mut return_code = code[start..i].to_vec();
                return_code.push('\0');
                let return_token = tokenizer(path.clone(), return_code);
                tokens.push(Token::Return(return_token));
            } else if code_str == "repet" {
                let start = i + 1;
                'sub: loop {
                    i += 1;
                    let c = code[i];
                    if c == '\0' {
                        break 'sub;
                    }
                    if c == '{' {
                        break 'sub;
                    }
                }
                let mut repeat_code = code[start..i].to_vec();
                i -= 1;
                repeat_code.push('\0');
                let repeat_token = tokenizer(path.clone(), repeat_code);
                match &repeat_token[0] {
                    Token::Integer(i) => {
                        if i <= &0 {
                            log.error(
                                (reading_y, start - 1),
                                format!(
                                    "INVAL!D REP3T C0UNT: MUST BE GR@TER THAN 0, G0T -1!! W@HAT??"
                                ),
                            );
                        }
                        tokens.push(Token::Repeat(Box::new(Token::Integer(*i))));
                    }
                    Token::Identifier(i) => {
                        tokens.push(Token::Repeat(Box::new(Token::Identifier(i.to_owned()))));
                    }
                    _ => {
                        log.error(
                            (reading_y, start - 1),
                            "INVAL!D REP3T EXPRESSION: MUST BE @N !NT3G3R OR !D3NT!F!3R!!"
                                .to_string(),
                        );
                    }
                }
            } else {
                i = start;
            }
        }

        if c.is_alphabetic() {
            let start = i;
            let mut arg_start = 0;
            let id_str: String;
            'sub: loop {
                i += 1;
                let c = code[i];

                if c == '\0' {
                    id_str = code[start..i].iter().collect();
                    break 'sub;
                } else if c.is_alphabetic() {
                    continue 'sub;
                } else if c == '(' {
                    id_str = code[start..i].iter().collect();
                    arg_start = i;
                    loop {
                        i += 1;
                        let c = code[i];
                        if c == '\0' {
                            log.error(
                                (reading_y, arg_start - start),
                                "INVAL!D FUNCT!ON C@LL: M!SS!NG CLOS!NG P@RENTHESES ')'!! OH NO!!"
                                    .to_string(),
                            );
                        } else if c == ')' {
                            i += 1;
                            break 'sub;
                        }
                    }
                } else {
                    id_str = code[start..i].iter().collect();
                    break 'sub;
                }
            }
            if arg_start == 0 {
                if id_str == "\n" || id_str == " " {
                    continue;
                }
                tokens.push(Token::Identifier(id_str));
            } else {
                let mut args = code[arg_start + 1..i - 1].to_vec();
                args.push('\0');
                let args = remove_comma(
                    tokenizer(path.clone(), args),
                    reading_y,
                    arg_start,
                    start,
                    &log,
                );

                tokens.push(Token::Function(id_str, args));
            }
        }
        match c {
            ',' => {
                tokens.push(Token::Symbol(Symbol::Comma));
                i += 1;
            }
            '=' => {
                tokens.push(Token::Symbol(Symbol::Equal));
                i += 1;
            }
            '+' => {
                tokens.push(Token::Symbol(Symbol::Plus));
                i += 1;
            }
            '-' => {
                tokens.push(Token::Symbol(Symbol::Minus));
                i += 1;
            }
            '*' => {
                tokens.push(Token::Symbol(Symbol::Multiply));
                i += 1;
            }
            '/' => {
                tokens.push(Token::Symbol(Symbol::Divide));
                i += 1;
            }
            _ => {}
        }
    }
    tokens
}
