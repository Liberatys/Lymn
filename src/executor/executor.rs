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
                return String::from(
                    "The given query was not able to be recognized as valid query",
                );
            }
            query_type::QueryType::SELECT => {
                let minimum_query_lenght = 4; // SELECT [col] FROM [table]
                let selector_keyword_index = 2;
                let table_name_index = 3;
                let query_column_index = 1;
                if self.query_plan.len() < minimum_query_lenght {
                    return String::from(
                        "Query length does not match the minimum length of SELECT query",
                    );
                }
                match self.query_plan[selector_keyword_index]
                    .clone()
                    .get_token_type()
                {
                    TokenType::KEYWORD(v) => {
                        if v != Keyword::FROM {
                            return String::from(
                                "Keywords found in given Query do not match the query plan",
                            );
                        }
                    }
                    _ => {}
                }
                let column_to_query = &self.query_plan[query_column_index];
                let mut is_multy_column_selection = false;
                let table_name = &self.query_plan[table_name_index];
                let table_name = table_name.get_token_value();
                self.table.name = table_name;
                self.table.read();
                match column_to_query.clone().get_token_type() {
                    TokenType::DATA(v) => match v {
                        types::data_type::DataType::LIST(_) => {
                            is_multy_column_selection = true;
                        }
                        _ => {}
                    },
                    _ => {}
                }
                if is_multy_column_selection {
                    match column_to_query.clone().get_token_type() {
                        TokenType::DATA(types::data_type::DataType::LIST(v)) => match v {
                            _ => {
                                let columns_to_query =
                                    convert_string_to_vec(column_to_query.get_token_value());
                                let column_vectors =
                                    self.query_column_values_for_multiple_columns(columns_to_query);
                                return format!("{:?}", column_vectors);
                            }
                        },
                        _ => {}
                    }
                }
                let _value: String = column_to_query.get_token_value();
                if _value == "*" {
                    let column_vectors =
                        self.query_column_values_for_multiple_columns(self.table.get_columns());
                    return format!("{:?}", column_vectors);
                }
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
                self.table.name = table_name.to_string();
                self.table.read();
                let index_number_of_columns = self.table.get_columns().len();
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
            query_type::QueryType::CREATE => {
                let creation_type_index = 1;
                let creation_type = self.query_plan[creation_type_index].clone();
                match creation_type.get_token_type() {
                    TokenType::KEYWORD(Keyword::DATABASE) => {
                        let database_name_index = 2;
                        let _database_name = self.query_plan[database_name_index]
                            .clone()
                            .get_token_value();
                    }
                    TokenType::KEYWORD(Keyword::TABLE) => {
                        let table_name_index = 2;
                        let table_column_definition_index = 3;
                        let table_name =
                            self.query_plan[table_name_index].clone().get_token_value();
                        let column_definition = self.query_plan[table_column_definition_index]
                            .clone()
                            .get_token_type();
                        match column_definition {
                            TokenType::DATA(types::data_type::DataType::LIST(v)) => {
                                let column_list = convert_string_to_vec(v);
                                if self.table.table_exist(&table_name) {
                                    return String::from(format!(
                                        "Table: {} already exists",
                                        table_name
                                    ));
                                }
                                self.table = InMemoryTabel::new(
                                    String::from(table_name.clone()),
                                    String::from("data"),
                                );
                                for column in column_list {
                                    let def_vec: Vec<&str> = column.trim().split(" ").collect();
                                    if def_vec.len() > 2 {
                                        return String::from("Invalid column definition");
                                    }
                                    self.table.insert_new_column(def_vec[0].to_owned());
                                }
                                self.table.write();
                                return String::from(format!("Table: {} created", table_name));
                            }
                            _ => return String::from("Invalid table definition"),
                        }
                    }
                    _ => {
                        return String::from("CREATE with given argument is not implemented yet");
                    }
                }
            }
            query_type::QueryType::DELETE => {}
            query_type::QueryType::UPDATE => {}
        }
        String::from("no error")
    }

    pub fn query_column_values_for_multiple_columns(
        &self,
        columns: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut column_vectors: Vec<Vec<String>> = Vec::new();
        for x in columns {
            column_vectors.push(self.table.get_colum(self.table.get_index_of_column(&x)));
        }
        column_vectors
    }
}

fn convert_string_to_vec(value: String) -> Vec<String> {
    let value_vec: Vec<&str> = value.split(",").collect();
    return Vec::from_iter(value_vec.iter().map(|v| v.to_string()));
}
