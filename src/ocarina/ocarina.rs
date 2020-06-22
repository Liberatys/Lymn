use super::query_traverser;
use super::token;
use token::determinator::Determinator;
use token::token::Token;

pub struct OcarinaParser {
    optimize: bool,
    token_list: Vec<Vec<Token>>,
    determinator: Determinator,
}

impl OcarinaParser {
    pub fn new(statement: &str) -> OcarinaParser {
        let parser = OcarinaParser {
            optimize: false,
            token_list: Vec::new(),
            determinator: Determinator::new(statement.to_owned()),
        };
        return parser;
    }

    /// Iterate over the query string and cleanup syntax and formating as well as summarization if
    /// possible:
    ///     SELECT * FROM test WHERE x = 2 + 5;
    ///     
    ///     SELECT * FROM test WHERE x = 7;
    pub fn sanitize_query(&mut self) {
        unimplemented!();
    }

    pub fn generate_token_list(&mut self) {
        self.determinator
            .iterate_over_query_and_collect_token_list();
    }

    pub fn determine_type_of_token(&mut self) {
        unimplemented!();
    }
}
