use super::query_traverser;
use super::token;
use token::determinator::Determinator;
use token::token::Token;

pub struct OcarinaParser {
    traverser: query_traverser::QueryTraverser,
    optimize: bool,
    token_list: Vec<Vec<Token>>,
}

impl OcarinaParser {
    pub fn new(statement: &str) -> OcarinaParser {
        let parser = OcarinaParser {
            traverser: query_traverser::QueryTraverser::new(statement.to_owned()),
            optimize: false,
            token_list: Vec::new(),
        };
        return parser;
    }

    /// Iterate over the query string and cleanup syntax and formating as well as summarization if
    /// possible:
    ///     SELECT * FROM test WHERE x = 2 + 5;
    ///     
    ///     SELECT * FROM test WHERE x = 7;
    pub fn sanitize_query(&mut self) {}

    pub fn generate_token_list(&mut self) {
        let mut token_list: Vec<Vec<Token>> = Vec::new();
        let mut current_token_list_index: usize = 0;
        let BLANK: char = ' ';
        let mut current_token_buffer = String::new();
        let mut token_determinator = Determinator::new();
        while self.traverser.has_next() {
            // a call to unwrap is reasonable because the outer while loop would break if no next
            // characters was available in the query vec
            let current_character = self.traverser.next().unwrap();
            if current_character == BLANK && current_token_buffer.len() > 0 {
                //TODO: implement a state-machine that iterates over the given string value and
                //TODO: then determines the type of token
                // call the state-machine above with the content of current_token_buffer
                token_list[current_token_list_index].push(
                    token_determinator.determine_type_of_token(
                        current_token_buffer.clone(),
                        self.traverser.current_index() - current_token_buffer.len(),
                        self.traverser.current_index(),
                    ),
                );
                current_token_buffer.clear();
            }
        }
    }

    pub fn determine_type_of_token(&mut self) {}

    pub fn seperate_queries(self) -> Vec<String> {
        return Vec::new();
    }
}
