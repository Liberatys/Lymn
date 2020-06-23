use super::token;
use std::collections::HashMap;
pub struct TokenEvaluator {
    method_map: HashMap<String, fn(value: String) -> token::TokenType>,
}

impl TokenEvaluator {
    pub fn new() -> TokenEvaluator {
        let evaluator = TokenEvaluator {
            method_map: HashMap::new(),
        };
        return evaluator;
    }

    pub fn set_method_map(
        &mut self,
        method_hash_map: HashMap<String, fn(value: String) -> token::TokenType>,
    ) {
        self.method_map = method_hash_map;
    }

    pub fn add_method_to_map(
        &mut self,
        method_ident: String,
        method: fn(value: String) -> token::TokenType,
    ) {
        self.method_map.insert(method_ident, method);
    }

    pub fn invoke_method(&self, method_ident: &str, value: String) -> token::TokenType {
        let method = self.method_map.get(method_ident);
        let token_type = match method {
            None => return token::TokenType::UNDETERMINED,
            Some(v) => {
                return v(value);
            }
        };
        return token_type;
    }
}
