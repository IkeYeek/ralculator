#[cfg(test)]
mod lexer {
    use crate::lexer;
    use crate::lexer::token::Kind::{Identifier, Literal, Operator, Separator};
    use crate::lexer::token::Token;

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