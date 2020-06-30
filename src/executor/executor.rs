use super::super::ocarina::token::token::Token;
use super::super::ocarina::token::token::TokenType;
use super::super::ocarina::types::keyword::Keyword;
use super::super::storage::disk::in_memory_table::InMemoryTabel;
use super::super::storage::disk::io::StorageEntity;
use super::super::storage::disk::table::Table;

use super::query_type;
use super::sql_error::SQLError;

pub struct Executor<'a> {
    query_plan: &'a Vec<Token>,
    table: InMemoryTabel,
}

impl<'a> Executor<'a> {
    pub fn new(query_plan: &'a std::vec::Vec<Token>, table: InMemoryTabel) -> Self {
        let executor = Executor {
            query_plan: query_plan,
            table: table,
        };
        executor
    }

    pub fn evaluate_query(&mut self) -> String {
        let query_type =
            query_type::QueryType::from_primary_query_token(self.query_plan[0].clone());
        match query_type {
            query_type::QueryType::NONE => {
                return SQLError::UnknownQueryType(format!(
                    "Query could not be evaulated
                    first token doesn't seem to be valid starting token
                    Token: {:?}",
                    self.query_plan[0].clone()
                ))
                .to_string();
            }
            query_type::QueryType::SELECT => {
                if self.query_plan.len() < 4 {
                    return SQLError::UnknownQueryType(format!(
                        "{}",
                        "Query {} is to small to be a select query"
                    ))
                    .to_string();
                }
                match self.query_plan[2].clone().get_token_type() {
                    TokenType::KEYWORD(v) => {
                        if v != Keyword::FROM {
                            return SQLError::UnknownQueryType(format!(
                                "{}",
                                "second keyword was not FROM therefore the
                            SELECT query could not be executed"
                            ))
                            .to_string();
                        }
                    }
                    _ => {}
                }
                let column_to_query = &self.query_plan[1];
                let _value: String = column_to_query.get_token_value();
                let from = &self.query_plan[3];
                let table_name = from.get_token_value();
                self.table.name = table_name;
                self.table.read();
                let result_cols = self
                    .table
                    .get_colum(self.table.get_index_of_column(&_value));
                return format!("{:?}", result_cols);
            }
            query_type::QueryType::INSERT => {}
            query_type::QueryType::DELETE => {}
            query_type::QueryType::UPDATE => {}
        }
        String::from("no error")
    }
}
