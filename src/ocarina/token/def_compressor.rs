use super::token;

pub struct Compressor {
    token_list: Vec<token::Token>,
}

impl Compressor {
    pub fn new() -> Compressor {
        let compressor: Compressor = Compressor {
            token_list: Vec::new(),
        };
        return compressor;
    }

    pub fn set_token_list(&mut self, tokens: &Vec<token::Token>) {
        self.token_list = tokens.to_owned().to_vec();
    }

    pub fn compress(&mut self) {}

    pub fn get_token_list(&self) -> Vec<token::Token> {
        return self.token_list.to_vec();
    }
}
