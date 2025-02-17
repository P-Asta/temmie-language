use crate::token::Token;

pub fn parse(tokens: Vec<Token>) -> Token {
    let mut i = 0;
    loop {
        if i >= tokens.len() {
            return Token::Integer(0);
        }
        let token = &tokens[i];
        i += 1;
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
            Token::Identifier(i) => {
                return Token::Identifier(i.to_owned());
            }
            Token::Function(name, args) => {
                for arg in args {
                    if name == "prnt" {
                        println!("{:?}", parse(arg.to_owned()));
                    }
                }
            }
            _ => {}
        }
    }
}
