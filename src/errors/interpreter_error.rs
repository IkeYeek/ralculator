use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InterpreterError {
    message: String,
}

impl InterpreterError {
    #[must_use]
    pub fn new(message: String) -> InterpreterError {
        InterpreterError {
            message
        }
    }
}

impl Error for InterpreterError {}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}