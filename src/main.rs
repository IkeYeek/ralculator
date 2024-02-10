use regex::Regex;
use crate::TokenType::{Identifier, Literal, Symbol};

mod test;

#[derive(Debug, PartialEq)]
pub(crate) enum TokenType {
    Symbol,
    Identifier,
    Literal,
}
#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    token_type: TokenType,
    raw_value: String,
}


pub(crate) struct Tokenizer {}
impl Tokenizer {
    fn extract_from_regex(token_type: TokenType, regex: Regex, chunk: &str) -> Option<Token> {
        match regex.find(chunk) {
            Some(matched) => Some(Token {
                token_type,
                raw_value: matched.as_str().into()
            }),
            None => None,
        }
    }
    fn extract_symbol_from_chunk(chunk: &str) -> Option<Token> {
        let symbol_regex = Regex::new(r"^[+-/*)(^]").unwrap();
        Self::extract_from_regex(Symbol, symbol_regex, chunk)
    }

    fn extract_literal_from_chunk(chunk: &str) -> Option<Token> {
        let literal_regex = Regex::new(r"^\d+").unwrap();
        Self::extract_from_regex(Literal, literal_regex, chunk)
    }

    fn extract_identifier_from_chunk(chunk: &str) -> Option<Token> {
        let identifier_regex = Regex::new(r"^[a-zA-Z_]+").unwrap();
        Self::extract_from_regex(Identifier, identifier_regex, chunk)
    }

    fn extract_next_token(chunk: &str) -> Token {
        if let Some(token) = Self::extract_symbol_from_chunk(chunk) {
            return token;
        } else if let Some(token) = Self::extract_literal_from_chunk(chunk) {
            return token;
        } else if let Some(token) = Self::extract_identifier_from_chunk(chunk) {
            return token;
        } else {
            panic!("Couldn't extract next token! current line analysis state: {}", chunk)
        }
    }


    fn tokenize_chunk(chunk: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut remaining = chunk;
        while remaining.len() > 0 {
            let token = Self::extract_next_token(remaining);
            remaining = &remaining[token.raw_value.len()..];
            tokens.push(token);
        }
        tokens
    }

    pub(crate) fn tokenize_line(line: String) -> Vec<Token> {
        line.split_whitespace().into_iter().fold(Vec::new(), |mut acc: Vec<Token>, raw_chunk| {
            acc.append(Self::tokenize_chunk(raw_chunk).as_mut());
            acc
        })
    }
}

fn main() {
    println!("Hello, world!");
}
