use super::super::types;
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    token_value: String,
}

impl Token {
    pub fn new(token_value: String) -> Token {
        let token = Token {
            token_value: token_value,
            token_type: TokenType::UNDETERMINED,
        };
        return token;
    }

    pub fn get_token_value(&self) -> String {
        return self.token_value.clone();
    }

    pub fn get_token_type(self) -> TokenType {
        return self.token_type;
    }

    pub fn set_token_type(&mut self, token_type: TokenType) {
        self.token_type = token_type;
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    KEYWORD(types::keyword::Keyword),
    OPERATOR(types::operator::Operator),
    DATA(types::data_type::DataType),
    UNDETERMINED,
}

impl TokenType {
    pub fn from_string(identifier: String, value: String) -> TokenType {
        let result = match identifier.as_ref() {
            "string" => TokenType::DATA(types::data_type::DataType::STRING(value)),
            "list" => TokenType::DATA(types::data_type::DataType::LIST(value)),
            _ => TokenType::UNDETERMINED,
        };
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn token_type_from_string_test() {
        let value: String = String::from("VALUE");
        let identifier: String = String::from("string");

        assert_eq!(
            TokenType::DATA(types::data_type::DataType::STRING(value.clone())),
            TokenType::from_string(identifier, value)
        );
    }
}
