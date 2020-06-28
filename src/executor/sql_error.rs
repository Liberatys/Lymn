use super::error;
use std::fmt;
#[derive(Debug)]
pub enum SQLError {
    Unknown_Query_Type(String),
}

impl error::Error for SQLError {
    fn to_string(&self) -> std::string::String {
        return format!("{}", self);
    }
}

impl fmt::Display for SQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
