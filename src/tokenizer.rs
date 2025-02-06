#[derive(Debug)]
pub enum Symbol {
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Token {
    Integer(isize),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Symbol(Symbol),
}
pub fn tokenizer(code: Vec<char>) -> Vec<Token> {
    let mut i = 0;
    let mut tokens = Vec::new();
    'main: loop {
        let c = code[i];
        if c == '\0' {
            break 'main;
        }
        if c == ' ' {
            i += 1;
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
                panic!("{num_str} is Invalid number");
            }
        }
        if c == '"' {
            let start = i + 1;
            'sub: loop {
                i += 1;
                let c = code[i];
                if c == '\0' {
                    panic!("Invalid string");
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
            let id_str: String = code[start..i].iter().collect();
            tokens.push(Token::Identifier(id_str));
        }
        if c == '=' {
            tokens.push(Token::Symbol(Symbol::Equal));
            i += 1;
        }
        if c == '+' {
            tokens.push(Token::Symbol(Symbol::Plus));
            i += 1;
        }
        if c == '-' {
            tokens.push(Token::Symbol(Symbol::Minus));
            i += 1;
        }
        if c == '*' {
            tokens.push(Token::Symbol(Symbol::Multiply));
            i += 1;
        }
        if c == '/' {
            tokens.push(Token::Symbol(Symbol::Divide));
            i += 1;
        }
    }
    tokens
}
