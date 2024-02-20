#[cfg(test)]
mod tests {
    use ralculator::logic::lexer::tokens::Token;
    use ralculator::logic::lexer::tokens::Kind;
    use ralculator::logic::lexer::Lexer;
    #[test]
    fn lex_returns_result() {
        let lexer = Lexer::new();
        assert_eq!(lexer.lex("".into()), Ok(Vec::new()))
    }

    #[test]
    fn lex_returns_1_plus_1() {
        let lexer = Lexer::new();
        assert_eq!(
            lexer.lex("1 + 1".into()),
            Ok(vec![
                Token::new(Kind::Literal, "1".into(), 0),
                Token::new(Kind::Operator, "+".into(), 2),
                Token::new(Kind::Literal, "1".into(), 4)
            ])
        );
    }

    #[test]
    fn lex_ignores_whitespace() {
        let lexer = Lexer::new();
        assert_eq!(
            lexer.lex("1 +  1".into()),
            Ok(vec![
                Token::new(Kind::Literal, "1".into(), 0),
                Token::new(Kind::Operator, "+".into(), 2),
                Token::new(Kind::Literal, "1".into(), 5)
            ])
        );
    }

    #[test]
    fn lex_returns_complex() {
        let lexer = Lexer::new();
        assert_eq!(
            lexer.lex("1+1*(4^2)/ a".into()).unwrap(),
            vec![
                Token::new(Kind::Literal, "1".into(), 0),
                Token::new(Kind::Operator, "+".into(), 1),
                Token::new(Kind::Literal, "1".into(), 2),
                Token::new(Kind::Operator, "*".into(), 3),
                Token::new(Kind::Separator, "(".into(), 4),
                Token::new(Kind::Literal, "4".into(), 5),
                Token::new(Kind::Operator, "^".into(), 6),
                Token::new(Kind::Literal, "2".into(), 7),
                Token::new(Kind::Separator, ")".into(), 8),
                Token::new(Kind::Operator, "/".into(), 9),
                Token::new(Kind::Identifier, "a".into(), 11)
            ]
        );
    }
}