use std::collections::HashMap;

use crate::token::Token;
pub struct Function {
    functions: Vec<Token>,
}

impl Function {
    pub fn new() -> Function {
        Function {
            functions: Vec::new(),
        }
    }

    pub fn add(&mut self, function: Token) {
        self.functions.push(function);
    }

    pub fn get(&self, function: Token) -> bool {
        if let Token::Function(name, args) = function {
            for f in &self.functions {
                if let Token::Function(n, a) = f {
                    return true;
                }
            }
        }
        false
    }

    pub fn can_use(&self, function: Token) -> bool {
        if let Token::Function(name, args) = function {
            for f in &self.functions {
                if let Token::Function(n, a) = f {
                    return a.len() == args.len();
                }
            }
        }
        false
    }
}

pub struct Variable {
    variables: HashMap<Token, Token>,
}

impl Variable {
    pub fn new() -> Variable {
        Variable {
            variables: HashMap::new(),
        }
    }

    pub fn add(&mut self, variable: Token, value: Token) {
        self.variables.insert(variable, value);
    }

    pub fn get(&self, variable: Token) -> Option<&Token> {
        self.variables.get(&variable)
    }

    pub fn can_use(&self, variable: Token) -> bool {
        self.variables.get(&variable).is_some()
    }
}
