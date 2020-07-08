use crate::ocarina::token::token::Token;

pub trait Determinator {
    fn add_token_to_current_token_list(&mut self, expected_type: Option<String>);
    fn iterate_over_query_and_collect_token_list(&mut self);
    fn get_token_list(self) -> Vec<Vec<Token>>;
}
