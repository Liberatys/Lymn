use super::super::ocarina::token::token::Token;
use super::super::ocarina::token::token::TokenType;
use super::super::ocarina::types;
use super::super::ocarina::types::keyword::Keyword;
use super::super::storage::disk::in_memory_table::InMemoryTabel;
use super::super::storage::disk::io::StorageEntity;
use super::super::storage::disk::table::Table;
use super::query_type;
use super::sql_error::SQLError;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::Map;

pub struct Executor<'a> {
    query_plan: &'a Vec<Token>,
    table: InMemoryTabel,
}

static mut CURRENT_INDEX: usize = 0;

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
            query_type::QueryType::INSERT => {
                if self.query_plan.len() < 4 {
                    return String::from("To little arguments in the query to execute");
                }
                match self.query_plan[1].clone().get_token_type() {
                    TokenType::KEYWORD(_v) => {}
                    _ => {
                        return String::from("INSERT query was not able to be executed");
                    }
                }
                let table_name = &self.query_plan[2].get_token_value();
                let index_number_of_columns = self.table.get_columns().len();
                self.table.read();
                self.table.name = table_name.to_string();
                match &self.query_plan[3].clone().get_token_type() {
                    TokenType::DATA(types::data_type::DataType::LIST(v)) => {
                        let token_value = v;
                        let mut hash_map: HashMap<&str, String> = HashMap::new();
                        let column_list: Vec<&str> = token_value.split(",").collect();
                        if self.query_plan[4].clone().get_token_type()
                            == TokenType::KEYWORD(Keyword::VALUES)
                        {
                            let query_plan = self.query_plan.clone();
                            for t in 5..query_plan.len() {
                                let value_vec =
                                    convert_string_to_vec(query_plan[t].get_token_value());
                                for index in 0..value_vec.len() {
                                    hash_map.insert(column_list[index], value_vec[index].clone());
                                }
                                self.table.insert_row_by_column(hash_map.clone());
                            }
                        } else {
                            return String::from(
                                "Error in query: item at position 5 was expected to be 'VALUES'",
                            );
                        }
                    }
                    TokenType::KEYWORD(Keyword::VALUES) => {
                        for t in 4..self.query_plan.len() {
                            let item = self.query_plan[t].clone().get_token_value();
                            let list_value: Vec<&str> = item.split(",").collect();
                            if list_value.len() != index_number_of_columns {
                                return String::from("Given values do not match the column count");
                            }
                            self.table.insert_row(list_value);
                        }
                    }
                    _ => {}
                }
                self.table.write();
                return String::from(format!("Index: {}", 1));
            }
            query_type::QueryType::DELETE => {}
            query_type::QueryType::UPDATE => {}
        }
        String::from("no error")
    }
}

fn convert_string_to_vec(value: String) -> Vec<String> {
    let value_vec: Vec<&str> = value.split(",").collect();
    return Vec::from_iter(value_vec.iter().map(|v| v.to_string()));
}
