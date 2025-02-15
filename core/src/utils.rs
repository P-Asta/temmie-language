use std::collections::HashMap;

use crate::token::Token;
pub struct Function {
    functions: HashMap<Token, Token>,
}

impl Function {
    pub fn new() -> Function {
        Function {
            functions: HashMap::new(),
        }
    }

    pub fn add(&mut self, function: Token, code: Token) {
        self.functions.insert(function, code);
    }

    pub fn get(&self, function: Token) -> Option<&Token> {
        self.functions.get(&function)
    }

    pub fn can_use(&self, function: Token) -> bool {
        self.functions.get(&function).is_some()
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
