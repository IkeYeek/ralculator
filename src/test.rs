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

#[cfg(test)]
mod parser {
    use crate::lexer::lex;
    use crate::parser::ast::Expression;
    use crate::parser::{Parser};

    #[test]
    fn parse_assign() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("a = 1")).unwrap()).unwrap(),
            Expression::Assignment("a".into(), Box::new(Expression::Literal(1))))
    }

    #[test]
    fn parse_1_plus_1() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from(("1 + 1"))).unwrap()).unwrap(),
            Expression::Addition(Box::new(Expression::Literal(1)), Box::new(Expression::Literal(1))));
    }


}