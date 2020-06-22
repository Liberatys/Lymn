use super::super::types;

pub struct Token {
    token_type: TokenType,
    token_value: String,
    start: i64,
    end: i64,
}

impl Token {
    pub fn new(token_value: String, start: i64, end: i64) -> Token {
        let token = Token {
            token_value: token_value,
            start: start,
            end: end,
            token_type: TokenType::UNDETERMINED,
        };
        return token;
    }
}

pub enum TokenType {
    KEYWORD(types::keyword::Keyword),
    OPERATOR(types::operator::Operator),
    UNDETERMINED,
}
