use std::ops::ControlFlow;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub(crate) enum TokenType {
    Symbol,
    Identifier,
    Literal,
}
#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) raw_value: String,
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
        Self::extract_from_regex(TokenType::Symbol, symbol_regex, chunk)
    }

    fn extract_literal_from_chunk(chunk: &str) -> Option<Token> {
        let literal_regex = Regex::new(r"^\d+").unwrap();
        Self::extract_from_regex(TokenType::Literal, literal_regex, chunk)
    }

    fn extract_identifier_from_chunk(chunk: &str) -> Option<Token> {
        let identifier_regex = Regex::new(r"^[a-zA-Z_]+").unwrap();
        Self::extract_from_regex(TokenType::Identifier, identifier_regex, chunk)
    }

    fn extract_next_token(chunk: &str) -> Result<Token, String> {
        if let Some(token) = Self::extract_symbol_from_chunk(chunk) {
            return Ok(token);
        } else if let Some(token) = Self::extract_literal_from_chunk(chunk) {
            return Ok(token);
        } else if let Some(token) = Self::extract_identifier_from_chunk(chunk) {
            return Ok(token);
        } else {
            Err(format!("Couldn't extract next token! current line analysis state: {}", chunk))
        }
    }


    fn tokenize_chunk(chunk: &str) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut remaining = chunk;
        while remaining.len() > 0 {
            match Self::extract_next_token(remaining) {
                Ok(token) => {
                    remaining = &remaining[token.raw_value.len()..];
                    tokens.push(token);
                }
                Err(e) => return Err(e)
            }
        }
        Ok(tokens)
    }

    pub(crate) fn tokenize_line(line: String) -> Result<Vec<Token>, String> {
        let fold_tokens = line.split_whitespace().into_iter().try_fold(Vec::new(), |mut acc, raw_chunk| {
            match Self::tokenize_chunk(raw_chunk).as_mut() {
                Ok(tokens) => {
                    acc.append(tokens);
                    ControlFlow::Continue(acc)
                },
                Err(e) => ControlFlow::Break(e.clone()),
            }
        });
        match fold_tokens {
            ControlFlow::Continue(tokens) => Ok(tokens),
            ControlFlow::Break(e) => Err(e)
        }
    }
}