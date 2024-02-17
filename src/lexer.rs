use regex::Regex;
use crate::lexer::token::Kind::{Identifier, Literal, Operator, Separator};
use crate::lexer::token::{ Token};

pub(crate) mod token {
    #[derive(Debug, PartialEq, Clone)]
    pub(crate) enum Kind {
        Identifier,
        Operator,
        Separator,
        Literal,
    }
    #[derive(Debug, PartialEq, Clone)]
    pub(crate) struct Token {
        pub(crate) kind: Kind,
        pub(crate) raw_value: String,
        pub(crate) position: usize,
    }

    impl Token {
        pub(crate) fn new(kind: Kind, raw_value: String, position: usize) -> Self {
            Token {
                kind,
                raw_value,
                position
            }
        }
    }
}

// function is agnostic to the current real buffer. start_offset is used to compute token start
fn next_token_in_buff(buffer: &str, buffer_start_offset: usize) -> Result<Token, String> {
    let trimmed_start_whitespaces = buffer.trim_start();
    let delta = buffer_start_offset + buffer.len() - trimmed_start_whitespaces.len();
    let regexs = [
        (Identifier, Regex::new(r"^[a-zA-Z_]+").unwrap()),
        (Literal, Regex::new(r"^\d+").unwrap()),
        (Operator, Regex::new(r"^[+-/*^=]").unwrap()),
        (Separator, Regex::new(r"^[()]").unwrap())
    ];
    for r in regexs {
        if let Some(res) = r.1.find(trimmed_start_whitespaces) {
            return Ok(Token::new(r.0, res.as_str().into(), delta));
        }
    }
    Err(format!("unknown token at position {delta}"))
}

pub(crate) fn lex(buffer: &str) -> Result<Vec<Token>, String> {
    let mut token_vector: Vec<Token> = Vec::new();
    let mut cursor: usize = 0;
    let mut buffer = buffer.trim();
    while cursor < buffer.len() {
        match next_token_in_buff(&buffer[cursor..], cursor) {
            Ok(token) => {
                cursor = token.position + token.raw_value.len();
                token_vector.push(token);
            },
            Err(e) => return Err(e)
        }
    }
    Ok(token_vector)
}

