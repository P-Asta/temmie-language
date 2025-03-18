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

    pub fn add_field(&mut self, name: String, value: Token) {
        self.fields.insert(name, value);
    }
    pub fn get_field(&self, name: String) -> Option<&Token> {
        self.fields.get(&name)
    }
    pub fn add_method(&mut self, name: String, function: Token) {
        self.methods.insert(name, function);
    }
    pub fn run_method(&self, name: String, args: HashMap<String, Token>) -> Token {
        let function = self.methods.get(&name).unwrap();
        let mut merge_fields = self.fields.clone();
        merge_fields.extend(args);
        eval(vec![function.to_owned()], merge_fields)
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "class {}",
            self.run_method("!!format!!".to_string(), HashMap::new())
        )
    }
}
