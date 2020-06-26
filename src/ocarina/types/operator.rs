#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    EQUAL,
    AND,
    OR,
    NOT_EQUAL,
    GREATER,
    LESS,
    GREATER_OR_EQUAL,
    LESS_OR_EQUAL,
    BETWEEN,
    LIKE,
    // in holds a conditional to determine if it is inverted or not
    IN(bool),
    IS_NULL(bool),
    // IS holds two bools that determine whether it is (inverted) true or (inverted) false
    IS(bool, bool),
    AS,
}
