use super::super::query_traverser;
use super::super::types;
use super::determinator;
use super::token::Token;
use super::token::TokenType;
use types::operator::Operator;

pub struct DefDeterminator {
    traverser: query_traverser::QueryTraverser,
    token_list: Vec<Vec<Token>>,
    current_token_buffer: String,
    current_token_list_index: usize,
}

fn wrapped_is_keyword_query(value: String) -> TokenType {
    let result_value = types::keyword::is_keyword(&value);
    if result_value != types::keyword::Keyword::UNKNOWN {
        return TokenType::KEYWORD(result_value);
    }
    return TokenType::UNDETERMINED;
}

lazy_static! {
    static ref token_evaluator: super::token_evaluator::TokenEvaluator = {
        let mut evaluator = super::token_evaluator::TokenEvaluator::new();
        evaluator.add_method_to_map(String::from("is_keyword"), wrapped_is_keyword_query);
        evaluator
    };
}

impl DefDeterminator {
    pub fn new(query: String) -> DefDeterminator {
        return Self {
            traverser: query_traverser::QueryTraverser::new(query.to_owned()),
            token_list: Vec::new(),
            current_token_buffer: String::new(),
            current_token_list_index: 0,
        };
    }
}

impl determinator::Determinator for DefDeterminator {
    fn add_token_to_current_token_list(&mut self, expected_type: Option<String>) {
        self.token_list[self.current_token_list_index].push(determine_type_of_token(
            self.current_token_buffer.clone(),
            expected_type,
        ));
    }

    fn iterate_over_query_and_collect_token_list(&mut self) {
        self.token_list.push(Vec::new());
        while self.traverser.has_next() {
            // a call to unwrap is reasonable because the outer while loop would break if no next
            // characters was available in the query vec
            let current_character = self.traverser.next().unwrap();
            match current_character {
                '\'' => {
                    // ===
                    let string_vec: Vec<char> = self.traverser.peek_till_next_occurrence('\'');
                    self.current_token_buffer = string_vec.into_iter().collect();
                    if self.current_token_buffer.len() > 0 {
                        self.traverser
                            .skip_next_n_indexes(self.current_token_buffer.len());
                        self.add_token_to_current_token_list(Some(String::from("string")));
                        self.current_token_buffer.clear();
                    }
                }
                '(' => {
                    if self.current_token_buffer.len() > 0 {
                        self.add_token_to_current_token_list(None);
                        self.current_token_buffer.clear();
                    }
                    let string_vec: Vec<char> = self.traverser.peek_till_next_occurrence(')');
                    self.current_token_buffer = string_vec.into_iter().collect();
                    if self.current_token_buffer.len() > 0 {
                        self.traverser
                            .skip_next_n_indexes(self.current_token_buffer.len() + 1);
                        self.add_token_to_current_token_list(Some(String::from("list")));
                        self.current_token_buffer.clear();
                    }
                }
                ';' => {
                    // End of a query
                    if self.current_token_buffer.len() > 0 {
                        self.add_token_to_current_token_list(None);
                        self.current_token_buffer.clear();
                    }
                    self.token_list.push(Vec::new());
                    self.current_token_list_index += 1;
                }
                ' ' => {
                    if self.current_token_buffer.len() > 0 {
                        self.add_token_to_current_token_list(None);
                        self.current_token_buffer.clear();
                    }
                }
                '=' => {
                    if self.current_token_buffer.len() > 0 {
                        self.add_token_to_current_token_list(None);
                        self.current_token_buffer.clear();
                    }
                    self.current_token_buffer.push(current_character);
                    self.token_list[self.current_token_list_index].push(token_builder(
                        self.current_token_buffer.clone().as_ref(),
                        TokenType::OPERATOR(Operator::EQUAL),
                    ));
                    self.current_token_buffer.clear();
                }
                _ => {
                    self.current_token_buffer.push(current_character);
                }
            }
        }

        if self.current_token_buffer.len() > 0 {
            self.add_token_to_current_token_list(None);
            self.current_token_buffer.clear();
        }
    }

    fn get_token_list(self) -> Vec<Vec<Token>> {
        return self.token_list;
    }
}

pub fn determine_type_of_token(value: String, expected_type: Option<String>) -> Token {
    return match expected_type {
        Some(expected_type_literal) => {
            let mut token = Token::new(value.clone());
            let token_type = TokenType::from_string(expected_type_literal, value);
            token.set_token_type(token_type);
            token
        }
        None => {
            let mut token = Token::new(value.clone());
            let token_type = token_evaluator.invoke_method("is_keyword", value.to_uppercase());
            token.set_token_type(token_type);
            token
        }
    };
}

/// easy constructor for tokens when a TokenType is known at creation time
fn token_builder(value: &str, token_type: TokenType) -> Token {
    let mut token = Token::new(String::from(value));
    token.set_token_type(token_type);
    return token;
}

#[cfg(test)]
mod tests {
    use super::determinator::Determinator;
    use super::types::keyword::Keyword;
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn iterate_over_query_and_collect_token_list_test() {
        let mut determinator = DefDeterminator::new(String::from("'SELECT'"));
        let mut token = Token::new(String::from("SELECT"));
        determinator.iterate_over_query_and_collect_token_list();
        token.set_token_type(TokenType::DATA(types::data_type::DataType::STRING(
            String::from("SELECT"),
        )));
        assert_eq!(determinator.token_list[0], vec![token]);
    }

    //TODO: decide if these tests should be moved to the keywords.rs file - because it really just
    //would make more sense
    #[test]
    fn determine_type_of_token_test_keyword_checking() {
        let mut test_and_result_map: HashMap<&str, TokenType> = HashMap::new();
        test_and_result_map.insert("SELECT", TokenType::KEYWORD(Keyword::SELECT));
        test_and_result_map.insert("CREATE", TokenType::KEYWORD(Keyword::CREATE));
        test_and_result_map.insert("INSERT", TokenType::KEYWORD(Keyword::INSERT));
        test_and_result_map.insert("FROM", TokenType::KEYWORD(Keyword::FROM));
        test_and_result_map.insert("IN", TokenType::KEYWORD(Keyword::IN));
        test_and_result_map.insert("WHERE", TokenType::KEYWORD(Keyword::WHERE));
        for (token_value, expected_result) in test_and_result_map {
            let result_token = determine_type_of_token(String::from(token_value), None);
            assert_eq!(result_token.get_token_type(), expected_result);
        }
    }

    #[test]
    fn iterate_over_query_and_collect_token_list_integeration() {
        let query: String = String::from("SELECT * FROM tablet WHERE test = 'GOGO'");
        let mut determinator = DefDeterminator::new(query.clone());
        determinator.iterate_over_query_and_collect_token_list();
        let seperated: Vec<&str> = query.split(" ").collect();
        assert_eq!(
            determinator.token_list[0],
            vec![
                token_builder("SELECT", TokenType::KEYWORD(Keyword::SELECT)),
                token_builder("*", TokenType::UNDETERMINED),
                token_builder("FROM", TokenType::KEYWORD(Keyword::FROM)),
                token_builder("tablet", TokenType::UNDETERMINED),
                token_builder("WHERE", TokenType::KEYWORD(Keyword::WHERE)),
                token_builder("test", TokenType::UNDETERMINED),
                token_builder("=", TokenType::OPERATOR(Operator::EQUAL)),
                token_builder(
                    "GOGO",
                    TokenType::DATA(types::data_type::DataType::STRING(String::from("GOGO")))
                ),
            ]
        );
    }
}

