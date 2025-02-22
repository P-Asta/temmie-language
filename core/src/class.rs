use std::collections::HashMap;

use crate::{eval::eval, token::Token};

pub struct Class {
    pub name: String,
    pub fields: HashMap<String, Token>,
    pub methods: HashMap<String, Token>,
}

impl Class {
    pub fn new(
        name: String,
        fields: HashMap<String, Token>,
        methods: HashMap<String, Token>,
    ) -> Self {
        Self {
            name,
            fields,
            methods,
        }
    }

    pub fn get_field(&self, name: String) -> Option<&Token> {
        self.fields.get(&name)
    }
    pub fn run_function(&self, name: String, args: Vec<Token>) -> Token {
        let function = self.methods.get(&name).unwrap();
        eval(vec![function.to_owned()], self.fields.clone(), Some(args))
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "class {}",
            self.run_function("!!format!!".to_string(), Vec::new())
        )
    }
}
