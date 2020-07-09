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

impl Operator {
    pub fn evaluates_to_true(&self, values: Vec<&str>) -> bool {
        match self {
            Operator::EQUAL => {
                if values.len() < 2 {
                    return false;
                }
                return values[0] == values[1];
            }
            Operator::LESS => {
                if values.len() < 2 {
                    return false;
                }
                return values[0] < values[1];
            }
            Operator::GREATER => {
                if values.len() < 2 {
                    return false;
                }
                return values[0] > values[1];
            }
            Operator::GREATER_OR_EQUAL => {
                if values.len() < 2 {
                    return false;
                }
                return values[0] >= values[1];
            }
            Operator::LESS_OR_EQUAL => {
                if values.len() < 2 {
                    return false;
                }
                return values[0] <= values[1];
            }
            Operator::NOT_EQUAL => {
                if values.len() < 2 {
                    return false;
                }
                return !(values[0] == values[1]);
            }
            _ => {}
        }
        println!("{}", true);
        true
    }
}

//TODO: write tests
