use super::super::ocarina::token::token::Token;
use super::super::ocarina::token::token::TokenType;
use super::super::ocarina::types::keyword::Keyword;
pub enum QueryType {
    SELECT,
    UPDATE,
    INSERT,
    DELETE,
    NONE,
}

impl QueryType {
    pub fn from_primary_query_token(token: Token) -> QueryType {
        match token.get_token_type() {
            TokenType::KEYWORD(Keyword::SELECT) => QueryType::SELECT,
            TokenType::KEYWORD(Keyword::UPDATE) => QueryType::UPDATE,
            TokenType::KEYWORD(Keyword::INSERT) => QueryType::INSERT,
            TokenType::KEYWORD(Keyword::DELETE) => QueryType::DELETE,
            _ => QueryType::NONE,
        }
    }
}
