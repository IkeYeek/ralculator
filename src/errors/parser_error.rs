use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParserError {
    message: String,
}

impl ParserError {
    pub fn new(message: String) -> ParserError {
        ParserError {
            message: String::from(message),
        }
    }
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}