use std::collections::HashMap;

use crate::{eval::eval, log, token::Token};
#[derive(Debug)]
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
        let log = log::Logging::new("CLASS".to_string());
        ::log::info!("클래스 '{}' 메서드 '{}' 호출 시도", self.name, name);
        ::log::info!(
            "사용 가능한 메서드: {:?}",
            self.methods.keys().collect::<Vec<_>>()
        );

        let function = match self.methods.get(&name) {
            Some(f) => f,
            None => {
                ::log::info!("메서드 '{}'를 찾을 수 없음", name);
                return Token::None;
            }
        };

        let mut merge_fields = self.fields.clone();
        merge_fields.extend(args);
        eval(vec![function.to_owned()], merge_fields).0
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
