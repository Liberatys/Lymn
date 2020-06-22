use super::token::Token;
pub struct Determinator {}

impl Determinator {
    pub fn new() -> Determinator {
        return Determinator {};
    }
    pub fn determine_type_of_token(
        &self,
        value: String,
        start_index: usize,
        end_index: usize,
    ) -> Token {
        unimplemented!();
    }
}
