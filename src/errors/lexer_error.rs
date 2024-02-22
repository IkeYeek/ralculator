use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct LexerError {
    message: String,
}

impl LexerError {
    #[must_use]
    pub fn new(message: String) -> LexerError {
        LexerError {
            message,
        }
    }
}

impl Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}