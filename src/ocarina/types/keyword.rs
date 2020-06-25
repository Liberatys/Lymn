#[derive(PartialEq, Debug)]
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
}

pub fn is_keyword(value: &str) -> Keyword {
    let is_query_keyword = is_query_keyword(value);
    if is_query_keyword != Keyword::UNKNOWN {
        return is_query_keyword;
    }
    let is_declarative_keyword = is_declarative_keyword(value);
    if is_declarative_keyword != Keyword::UNKNOWN {
        return is_declarative_keyword;
    }
    let is_structural_keyword = is_structural_keyword(value);
    if is_structural_keyword != Keyword::UNKNOWN {
        return is_structural_keyword;
    }
    return Keyword::UNKNOWN;
}

fn is_declarative_keyword(value: &str) -> Keyword {
    let result_keyword = match value {
        "CREATE" => Keyword::CREATE,
        "DROP" => Keyword::DROP,
        _ => Keyword::UNKNOWN,
    };
    return result_keyword;
}

fn is_query_keyword(value: &str) -> Keyword {
    let result_keyword = match value {
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
        _ => Keyword::UNKNOWN,
    };
    return result_keyword;
}

fn is_structural_keyword(value: &str) -> Keyword {
    let result_keyword = match value {
        "TABLE" => Keyword::TABLE,
        "DATABASE" => Keyword::DATABASE,
        "VIEW" => Keyword::VIEW,
        _ => Keyword::UNKNOWN,
    };
    return result_keyword;
}

