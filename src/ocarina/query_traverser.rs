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
        return Some(self.query[self.current_index - 1]);
    }

    pub fn current_index(&self) -> usize {
        return self.current_index;
    }

    /// checks if the query contains a forward character in the Vec
    pub fn has_next(&self) -> bool {
        if self.query_length == 0 {
            return false; // would result in an usize overflow if not checked
        }
        if self.current_index >= self.query_length - 1 {
            return false;
        }
        return true;
    }

    //TODO: implement this method
    pub fn peek_till_next_occurence(&self, character_to_occure: char) -> Vec<char> {
        let mut peek_result_set: Vec<char> = Vec::new();
        for index in self.current_index + 1..self.query_length {
            let current_value: char = self.query[index];
            if character_to_occure == current_value {
                return peek_result_set;
            } else {
                peek_result_set.push(current_value);
            }
        }
        return Vec::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn peek_next_character_test() {
        let mut traverser = QueryTraverser::new(String::from("SELECT * FROM Testing"));
        assert_eq!(traverser.next().unwrap(), 'S');
    }

    #[test]
    fn get_current_index_test() {
        let mut traverser = QueryTraverser::new(String::from("SELECT * FROM Testing"));
        for index in 0.."SELECT * FROM Testing".len() {
            assert_eq!(index, traverser.current_index);
            traverser.next();
        }
    }

    #[test]
    fn has_next_test() {
        let mut traverser = QueryTraverser::new(String::from("SELECT * FROM Testing"));
        assert_eq!(true, traverser.has_next());
    }

    #[test]
    fn peek_till_next_occurence_test() {
        let mut traverser = QueryTraverser::new(String::from("SELECT * FROM Testing"));
        traverser.next();
        assert_eq!(vec!['L'], traverser.peek_till_next_occurence('E'));
        traverser = QueryTraverser::new(String::from("'TESTING'"));
        let result_vec: Vec<char> = String::from("TESTING").chars().collect();
        assert_eq!(result_vec, traverser.peek_till_next_occurence('\''));
    }

    #[test]
    fn has_next_empty_query_input() {
        let mut traverser = QueryTraverser::new(String::from(""));
        assert_eq!(false, traverser.has_next());
    }
}
