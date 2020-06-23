use super::super::query_traverser;
use super::super::types;
use super::token::Token;
use super::token::TokenType;

pub struct Determinator {
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

impl Determinator {
    pub fn new(query: String) -> Determinator {
        return Determinator {
            traverser: query_traverser::QueryTraverser::new(query.to_owned()),
            token_list: Vec::new(),
            current_token_buffer: String::new(),
            current_token_list_index: 0,
        };
    }

    //TODO: refactor 'switch' case... extract methods
    //TODO: maybe extract the switch into a structure that has methods for each match case
    // like a state-machine
    pub fn iterate_over_query_and_collect_token_list(&mut self) {
        self.token_list.push(Vec::new());
        while self.traverser.has_next() {
            // a call to unwrap is reasonable because the outer while loop would break if no next
            // characters was available in the query vec
            let current_character = self.traverser.next().unwrap();
            match current_character {
                '\'' => {
                    let string_vec: Vec<char> = self.traverser.peek_till_next_occurence('\'');
                    self.current_token_buffer = string_vec.into_iter().collect();
                    self.traverser
                        .skip_next_n_indexes(self.current_token_buffer.len());
                    println!("{}", self.current_token_buffer);
                    self.token_list[self.current_token_list_index].push(determine_type_of_token(
                        self.current_token_buffer.clone(),
                        self.traverser.current_index() - self.current_token_buffer.len(),
                        self.traverser.current_index(),
                        Some(String::from("string")),
                    ));
                    self.current_token_buffer.clear();
                }
                '0'..='9' => {
                    // Integer literal
                    self.current_token_buffer.push(current_character);
                }
                ';' => {
                    // End of a query
                    if self.current_token_buffer.len() > 0 {
                        self.token_list[self.current_token_list_index].push(
                            determine_type_of_token(
                                self.current_token_buffer.clone(),
                                self.traverser.current_index() - self.current_token_buffer.len(),
                                self.traverser.current_index(),
                                None,
                            ),
                        );
                        self.current_token_buffer.clear();
                        self.token_list.push(Vec::new());
                    }
                    self.current_token_list_index += 1;
                }
                ' ' => {
                    // Blank == a token has ended
                    self.token_list[self.current_token_list_index].push(determine_type_of_token(
                        self.current_token_buffer.clone(),
                        self.traverser.current_index() - self.current_token_buffer.len(),
                        self.traverser.current_index(),
                        None,
                    ));
                    self.current_token_buffer.clear();
                }
                _ => {
                    unimplemented!();
                }
            }
        }
    }
}

pub fn determine_type_of_token(
    value: String,
    start_index: usize,
    end_index: usize,
    expected_type: Option<String>,
) -> Token {
    return match expected_type {
        Some(expected_type_literal) => {
            let mut token = Token::new(value.clone(), start_index, end_index);
            let token_type = TokenType::from_string(expected_type_literal, value);
            token.set_token_type(token_type);
            token
        }
        None => {
            let mut token = Token::new(value.clone(), start_index, end_index);
            let token_type = token_evaluator.invoke_method("is_keyword", value);
            token.set_token_type(token_type);
            token
        }
    };
}

#[cfg(test)]
mod tests {
    use super::types::keyword::Keyword;
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn iterate_over_query_and_collect_token_list_test() {
        let mut determinator = Determinator::new(String::from("'SELECT'"));
        let mut token = Token::new(String::from("SELECT"), 1, 1 + "SELECT".len());
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
            let result_token = determine_type_of_token(String::from(token_value), 0, 0, None);
            assert_eq!(result_token.get_token_type(), expected_result);
        }
    }
}
