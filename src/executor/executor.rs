use super::super::ocarina::token::token::Token;
use super::query_type;
use super::sql_error::SQLError;
pub struct Executor<'a> {
    query_plan: &'a Vec<Token>,
}

impl<'a> Executor<'a> {
    pub fn new(query_plan: &'a std::vec::Vec<Token>) -> Self {
        let executor = Executor {
            query_plan: query_plan,
        };
        executor
    }

    pub fn evaluate_query(&self) -> String {
        let query_type =
            query_type::QueryType::from_primary_query_token(self.query_plan[0].clone());
        match query_type {
            query_type::QueryType::NONE => {
                return SQLError::Unknown_Query_Type(format!(
                    "Query could not be evaulated\n
                        first token doesn't seem to be valid starting token\n
                        Token: {:?}",
                    self.query_plan[0].clone()
                ))
                .to_string();
            }
            _ => {}
        }

        String::from("No error")
    }
}
