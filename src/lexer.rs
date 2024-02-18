use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenKind {
    Identifier,
    Operator,
    Separator,
    Literal,
}
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) raw_value: String,
    pub(crate) position: usize,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, raw_value: String, position: usize) -> Self {
        Token {
            kind,
            raw_value,
            position
        }
    }
}

// function is agnostic to the current real buffer. start_offset is used to compute token start
fn next_token_in_buff(buffer: &str, buffer_start_offset: usize) -> Result<Token, String> {
    let trimmed_start_whitespaces = buffer.trim_start();
    let delta = buffer_start_offset + buffer.len() - trimmed_start_whitespaces.len();
    let regexs = [
        (TokenKind::Identifier, Regex::new(r"^[a-zA-Z_]+").unwrap()),
        (TokenKind::Literal, Regex::new(r"^\d+([.]\d+)?(e[+-]?\d+)?").unwrap()),
        (TokenKind::Operator, Regex::new(r"^[+-/*^=]").unwrap()),
        (TokenKind::Separator, Regex::new(r"^[()]").unwrap())
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
    let buffer = buffer.trim();
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

#[cfg(test)]
pub(crate) mod test {
    use crate::lexer;
    use crate::lexer::TokenKind::{Identifier, Literal, Operator, Separator};
    use crate::lexer::Token;

    #[test]
    fn lex_returns_result() {
        assert_eq!(lexer::lex("".into()), Ok(Vec::new()))
    }

    #[test]
    fn lex_returns_1_plus_1() {
        assert_eq!(lexer::lex("1 + 1".into()), Ok(vec![
            Token::new(Literal, "1".into(), 0),
            Token::new(Operator, "+".into(), 2),
            Token::new(Literal, "1".into(), 4),
        ]));
    }

    #[test]
    fn lex_ignores_whitespace() {
        assert_eq!(lexer::lex("1 +  1".into()), Ok(vec![
            Token::new(Literal, "1".into(),  0),
            Token::new(Operator, "+".into(),  2),
            Token::new(Literal, "1".into(),  5),
        ]));
    }

    #[test]
    fn lex_returns_complex() {
        assert_eq!(lexer::lex("1+1*(4^2)/ a".into()).unwrap(), vec![
            Token::new(Literal, "1".into(), 0),
            Token::new(Operator, "+".into(), 1),
            Token::new(Literal, "1".into(), 2),
            Token::new(Operator, "*".into(), 3),
            Token::new(Separator, "(".into(), 4),
            Token::new(Literal, "4".into(), 5),
            Token::new(Operator, "^".into(), 6),
            Token::new(Literal, "2".into(), 7),
            Token::new(Separator, ")".into(), 8),
            Token::new(Operator, "/".into(), 9),
            Token::new(Identifier, "a".into(), 11),
        ]);
    }
}

