use regex::Regex;
use crate::expressions::lexer::tokens::{Kind, Token};

pub mod tokens {
    #[derive(Debug, PartialEq, Clone)]
    pub enum Kind {
        Identifier,
        Operator,
        Separator,
        Literal,
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct Token {
        pub(crate) kind: Kind,
        pub(crate) raw_value: String,
        pub(crate) position: usize,
    }

    impl Token {
        pub fn new(kind: Kind, raw_value: String, position: usize) -> Self {
            Token {
                kind,
                raw_value,
                position,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct TokenStream {
        buffer: Vec<Token>,
        cursor: usize,
    }

    impl TokenStream {
        pub(crate) fn new(buffer: Vec<Token>) -> Self {
            Self { buffer, cursor: 0 }
        }

        pub(crate) fn curr(&self) -> Option<&Token> {
            self.buffer.get(self.cursor)
        }

        pub(crate) fn next(&mut self) -> Option<&Token> {
            self.cursor += 1;
            self.curr()
        }

        pub(crate) fn lookahead(&self) -> Option<&Token> {
            self.buffer.get(self.cursor + 1)
        }
    }
}

pub struct Lexer {
    token_regexs: [(Kind, Regex); 4],
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            token_regexs: [
                (Kind::Identifier, Regex::new(r"^[a-zA-Z_]+").unwrap()),
                (
                    Kind::Literal,
                    Regex::new(r"^\d+([.]\d+)?(e[+-]?\d+)?").unwrap(),
                ),
                (Kind::Operator, Regex::new(r"^[+-/*^=]").unwrap()),
                (Kind::Separator, Regex::new(r"^[()]").unwrap()),
            ],
        }
    }

    pub fn lex(&self, buffer: &str) -> Result<Vec<Token>, String> {
        let mut token_vector: Vec<Token> = Vec::new();
        let mut cursor: usize = 0;
        let buffer = buffer.trim();
        while cursor < buffer.len() {
            match self.next_token_in_buff(&buffer[cursor..], cursor) {
                Ok(token) => {
                    cursor = token.position + token.raw_value.len();
                    token_vector.push(token);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(token_vector)
    }

    // function is agnostic to the current real buffer. start_offset is used to compute token start
    fn next_token_in_buff(
        &self,
        buffer: &str,
        buffer_start_offset: usize,
    ) -> Result<Token, String> {
        let trimmed_start_whitespaces = buffer.trim_start();
        let delta = buffer_start_offset + buffer.len() - trimmed_start_whitespaces.len(); // we add to the offset the number of whitespace preceeding the potential actual token.
        for r in &self.token_regexs {
            if let Some(res) = r.1.find(trimmed_start_whitespaces) {
                return Ok(Token::new(r.0.clone(), res.as_str().into(), delta));
            }
        }
        Err(format!("unknown token at position {delta}"))
    }
}
