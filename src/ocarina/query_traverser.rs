pub struct QueryTraverser {
    query: Vec<char>,
    query_length: usize,
    current_index: usize,
}

impl QueryTraverser {
    pub fn new(query: String) -> QueryTraverser {
        let mut traverser = QueryTraverser {
            query: query.chars().collect(),
            current_index: 0,
            query_length: 0,
        };
        traverser.query_length = traverser.query.len();
        return traverser;
    }

    ///
    /// return the next char in Vec<char>
    ///
    /// if current_index is set on the last index of query
    ///     return None
    /// else
    ///     return Some(query[current_index])
    ///
    pub fn next(&mut self) -> Option<char> {
        if self.has_next() == false {
            return None;
        }
        self.current_index += 1;
        return Some(self.query[self.current_index]);
    }

    pub fn current_index(&self) -> usize {
        return self.current_index;
    }

    /// checks if the query contains a forward character in the Vec
    pub fn has_next(&self) -> bool {
        return !self.current_index == self.query_length - 1;
    }

    //TODO: implement this method
    pub fn peek_till_next_occurence(&self, character_to_occure: char) -> Vec<char> {
        unimplemented!();
    }

    /// peek ahead of the current set index
    /// by a given count
    /// the result set is returned as a Vec<char>
    ///
    /// if given count to peek is greater then the remaining size
    /// the given count is overwritten with the remaining size of query:Vec<char>
    ///
    pub fn peek(&self, mut indexes: usize) -> Vec<char> {
        let mut peek_result_set = Vec::new();
        let remaining_size_of_query: usize = self.get_count_of_chars_forward();
        if indexes > remaining_size_of_query {
            indexes = remaining_size_of_query;
        }
        if indexes == 0 {
            return peek_result_set;
        }
        for peek_index in self.current_index..(self.current_index) + indexes {
            let value = self.query[peek_index];
            peek_result_set.push(value);
        }
        return peek_result_set;
    }

    fn get_count_of_chars_forward(&self) -> usize {
        return self.query_length - self.current_index;
    }
}
