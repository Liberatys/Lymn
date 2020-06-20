//Statements that are used in the decleration of 
pub enum Declarative{
    CREATE(String),
    UPDATE(String),
    DROP(String),
    DELETE(String),
    TRUNCATE(String),
}


pub enum StructureType{
    TABLE(String),
    DATABASE(String),
    VIEW(String),
}


// Maybe remove some of the restricting query words
//Statements that are used in queries
pub enum Altering{
    SELECT(String),
    INSERT(String),
    UPDATE(String),
    SET(String),
    FROM(String),
    WHERE(String),
    AND(String),
    OR(String),
    LIMIT(String),
    GROUP(String),
    HAVING(String),
    IN(String),
    JOIN(String),
    UNION(String),
    EXISTS(String),
    LIKE(String),
    //! CASE
}
