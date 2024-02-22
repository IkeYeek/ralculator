use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SyntaxError {
    message: String,
}

impl SyntaxError {
    #[must_use]
    pub fn new(message: String) -> SyntaxError {
        SyntaxError {
            message,
        }
    }
}

impl Error for SyntaxError {}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}