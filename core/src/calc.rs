use crate::token::{FakeFloat, Symbol, Token};

/// return Token[Integer or Float]
pub fn calc(tokens: Vec<Token>) -> Token {
    let mut current_term = Vec::new();
    let mut terms = Vec::new();
    let mut operators = Vec::new();
    let mut has_float = false;

    // Check if there are any float values
    for token in &tokens {
        if let Token::Float(_) = token {
            has_float = true;
            break;
        }
        if !matches!(
            token,
            Token::Symbol(_) | Token::Float(_) | Token::Integer(_)
        ) {
            return Token::None;
        }
    }

    // Split into terms
    for token in tokens {
        match &token {
            Token::Symbol(Symbol::Plus) | Token::Symbol(Symbol::Minus) => {
                terms.push(current_term.clone());
                operators.push(token);
                current_term.clear();
            }
            _ => current_term.push(token),
        }
    }
    terms.push(current_term);

    // Process each term (multiplication and division)
    let mut processed_terms = Vec::new();
    for term in terms {
        if term.is_empty() {
            continue;
        }

        let mut result = match &term[0] {
            Token::Integer(n) => *n as f64,
            Token::Float(f) => f.0,
            _ => 0.0,
        };

        let mut i = 1;
        while i < term.len() {
            if let Token::Symbol(symbol) = &term[i] {
                let next_val = match &term[i + 1] {
                    Token::Integer(n) => *n as f64,
                    Token::Float(f) => f.0,
                    _ => 0.0,
                };

                match symbol {
                    Symbol::Multiply => result *= next_val,
                    Symbol::Divide => result /= next_val,
                    _ => (),
                }
                i += 2;
            }
        }
        processed_terms.push(result);
    }

    // Combine terms using addition and subtraction
    let mut final_result = processed_terms[0];
    for (i, op) in operators.iter().enumerate() {
        if let Token::Symbol(symbol) = op {
            match symbol {
                Symbol::Plus => final_result += processed_terms[i + 1],
                Symbol::Minus => final_result -= processed_terms[i + 1],
                _ => (),
            }
        }
    }

    // Return appropriate token type
    if has_float {
        Token::Float(FakeFloat(final_result))
    } else {
        Token::Integer(final_result as isize)
    }
}
