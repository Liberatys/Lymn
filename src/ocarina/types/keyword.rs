use super::super::token::token::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    TABLE,
    DATABASE,
    VIEW,
    CREATE,
    UPDATE,
    DROP,
    DELETE,
    TRUNCATE,
    SELECT,
    INSERT,
    SET,
    FROM,
    WHERE,
    AND,
    OR,
    LIMIT,
    GROUP,
    HAVING,
    IN,
    JOIN,
    UNION,
    EXISTS,
    LIKE,
    UNKNOWN,
    INTO,
}

pub fn is_keyword(value: &str) -> Keyword {
    let result_keyword = match value {
        "CREATE" => Keyword::CREATE,
        "INTO" => Keyword::INTO,
        "DROP" => Keyword::DROP,
        "SELECT" => Keyword::SELECT,
        "UPDATE" => Keyword::UPDATE,
        "FROM" => Keyword::FROM,
        "WHERE" => Keyword::WHERE,
        "IN" => Keyword::IN,
        "INSERT" => Keyword::INSERT,
        "SET" => Keyword::SET,
        "HAVING" => Keyword::HAVING,
        "JOIN" => Keyword::JOIN,
        "EXISTS" => Keyword::EXISTS,
        "TABLE" => Keyword::TABLE,
        "DATABASE" => Keyword::DATABASE,
        "VIEW" => Keyword::VIEW,
        _ => Keyword::UNKNOWN,
    };
    return result_keyword;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_keyword_detection() {
        let mut result_map: HashMap<&str, Keyword> = HashMap::new();
        result_map.insert("SELECT", Keyword::SELECT);
        result_map.insert("FROM", Keyword::FROM);
        result_map.insert("WHERE", Keyword::WHERE);
        result_map.insert("SET", Keyword::SET);
        result_map.insert("JOIN", Keyword::JOIN);
        result_map.insert("UPDATE", Keyword::UPDATE);
        result_map.insert("INTO", Keyword::INTO);
        result_map.insert("DROP", Keyword::DROP);
        result_map.insert("INSERT", Keyword::INSERT);
        for (key, value) in result_map {
            assert_eq!(is_keyword(key), value);
        }
    }
}
