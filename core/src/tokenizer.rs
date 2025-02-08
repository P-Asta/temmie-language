use crate::log;
use crate::token::*;

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
                tokens.push(Token::Float(num_str.parse().unwrap()));
                continue 'main;
            } else {
                log.error(
                    (reading_y, reading_x),
                    format!("{num_str} is Invalid number"),
                );
            }
        }
        if c == '"' {
            let start = i + 1;
            'sub: loop {
                i += 1;
                let c = code[i];
                if c == '\0' {
                    log.error((reading_y, reading_x), format!("Invalid string"));
                }
                if c == '"' {
                    break 'sub;
                }
            }
            let str_value = code[start..i].iter().collect();
            tokens.push(Token::String(str_value));
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
                if c.is_alphabetic() {
                    continue 'sub;
                } else {
                    break 'sub;
                }
            }
            let bool_str: String = code[start..i].iter().collect();
            if bool_str == "true" {
                tokens.push(Token::Boolean(true));
            } else if bool_str == "false" {
                tokens.push(Token::Boolean(false));
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
                                "Invalid argument".to_string(),
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
                tokens.push(Token::Identifier(id_str));
            } else {
                let mut args = code[arg_start + 1..i - 1].to_vec();
                args.push('\0');
                let mut args = tokenizer(path.clone(), args);
                let mut remove_comma_count = 0;
                for i in 1..=args.len() / 2 {
                    if args[(i * 2 - 1) - remove_comma_count] != Token::Symbol(Symbol::Comma) {
                        log.error(
                            (reading_y, arg_start - start),
                            "Invalid argument".to_string(),
                        );
                    }
                    args.remove((i * 2 - 1) - remove_comma_count);
                    remove_comma_count += 1;
                }
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
