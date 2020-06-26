#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    STRING(String),
    INTEGER(i64),
    CHARACTER(char),
}

impl DataType {
    pub fn convert_from_string_to_data_type(value: String) -> DataType {
        return DataType::INTEGER(0);
    }
}
