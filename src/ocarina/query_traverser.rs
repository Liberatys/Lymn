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

    pub fn skip_next_n_indexes(&mut self, indexes: usize) {
        self.current_index += indexes;
    }

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

    pub fn has_next(&self) -> bool {
        if self.query_length == 0 {
            return false; // would result in an usize overflow if not checked
        }
        if self.current_index >= self.query_length {
            return false;
        }
        return true;
    }

    pub fn peek_till_next_occurrence(&self, character_to_occure: char) -> Vec<char> {
        let mut peek_result_set: Vec<char> = Vec::new();
        for index in self.current_index..self.query_length {
            let current_value: char = self.query[index];
            if character_to_occure == current_value {
                return peek_result_set;
            } else {
                peek_result_set.push(current_value);
            }
        }
        return Vec::new();
    }

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
    fn has_next_negative_test() {
        let mut traverser = QueryTraverser::new(String::from(""));
        assert_eq!(false, traverser.has_next());
    }

    #[test]
    fn peek_till_next_occurence_test() {
        let mut traverser = QueryTraverser::new(String::from("SELECT * FROM Testing"));
        traverser.next();
        traverser.next();
        assert_eq!(vec!['L'], traverser.peek_till_next_occurrence('E'));
        traverser = QueryTraverser::new(String::from("'TESTING'"));
        traverser.next();
        let result_vec: Vec<char> = String::from("TESTING").chars().collect();
        assert_eq!(result_vec, traverser.peek_till_next_occurrence('\''));
    }

    #[test]
    fn has_next_empty_query_input() {
        let mut traverser = QueryTraverser::new(String::from(""));
        assert_eq!(false, traverser.has_next());
    }
}
