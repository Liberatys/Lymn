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
