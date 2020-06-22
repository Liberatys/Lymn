use super::super::query_traverser;
use super::token::Token;
pub struct Determinator {
    traverser: query_traverser::QueryTraverser,
    token_list: Vec<Vec<Token>>,
    current_token_buffer: String,
    current_token_list_index: usize,
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

    pub fn iterate_over_query_and_collect_token_list(&mut self) {
        while self.traverser.has_next() {
            // a call to unwrap is reasonable because the outer while loop would break if no next
            // characters was available in the query vec
            let current_character = self.traverser.next().unwrap();
            match current_character {
                '\'' => {
                    let string_vec: Vec<char> = self.traverser.peek_till_next_occurence('\'');
                }
                '0'..='9' => {
                    // Integer literal
                    self.current_token_buffer.push(current_character);
                }
                ';' => {
                    // End of a query
                    self.current_token_list_index += 1;
                }
                ' ' => {
                    // Blank == a token has ended
                    self.token_list[self.current_token_list_index].push(determine_type_of_token(
                        self.current_token_buffer.clone(),
                        self.traverser.current_index() - self.current_token_buffer.len(),
                        self.traverser.current_index(),
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

pub fn determine_type_of_token(value: String, start_index: usize, end_index: usize) -> Token {
    unimplemented!();
}

#[cfg(test)]
#[test]
fn name() {}
