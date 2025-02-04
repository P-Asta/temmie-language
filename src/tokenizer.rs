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
    Number(isize),
    Float(f64),
    String(String),
    Identifier(String),
    Symbol(String),
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
            println!("{:?}", num_str);
            if dot_cnt == 0 {
                tokens.push(Token::Number(num_str.parse().unwrap()));
                continue 'main;
            }
            if dot_cnt == 1 {
                tokens.push(Token::Float(num_str.parse().unwrap()));
                continue 'main;
            } else {
                panic!("{num_str} is Invalid number");
            }
        }
    }
    tokens
}
