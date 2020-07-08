use super::token;
use super::token::def_compressor;
use token::determinator::Determinator;
use token::token::Token;

pub struct OcarinaParser<T: Determinator> {
    optimize: bool,
    pub determinator: T,
    pub compressor: def_compressor::Compressor,
}

impl<T: Determinator> OcarinaParser<T> {
    pub fn new(determinator: T) -> Self {
        let parser = OcarinaParser {
            optimize: false,
            determinator: determinator,
            compressor: def_compressor::Compressor::new(),
        };
        return parser;
    }

    /// Iterate over the query string and cleanup syntax and formating as well as summarization if
    /// possible:
    /// SELECT * FROM test WHERE x = 2 + 5;
    ///     
    /// SELECT * FROM test WHERE x = 7;

    pub fn sanitize_query(&mut self) {
        unimplemented!();
    }

    pub fn generate_token_list(&mut self) {
        self.determinator
            .iterate_over_query_and_collect_token_list();
        //self.sanitize_query();
    }

    pub fn compress_token_list(mut self) -> Vec<Vec<Token>> {
        let mut token_list = self.determinator.get_token_list();
        for x in 0..token_list.len() {
            self.compressor.set_token_list(&token_list[x]);
            self.compressor.compress();
            token_list[x] = self.compressor.get_token_list();
        }
        return token_list;
    }
}
