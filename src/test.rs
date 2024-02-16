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
    use crate::parser::ast::Expression::{Addition, Multiplication, ParenthesisExpression, UnaryMinus, Variable};

    #[test]
    fn parse_assign() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("a = 1")).unwrap()).unwrap(),
            Expression::Assignment("a".into(), Box::new(Expression::Literal(1))))
    }

    #[test]
    fn parse_1() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("1")).unwrap()).unwrap(), Expression::Literal(1))
    }

    #[test]
    fn parse_23() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("23")).unwrap()).unwrap(), Expression::Literal(23))
    }

    #[test] fn parse_minus_1() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("-1")).unwrap()).unwrap(), UnaryMinus(Box::new(Expression::Literal(1))))
    }

    #[test]
    fn parse_1_plus_1() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("1 + 1")).unwrap()).unwrap(),
            Addition(Box::new(Expression::Literal(1)), Box::new(Expression::Literal(1))));
    }

    #[test]
    fn parse_1_minus_2() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("1 - 2")).unwrap()).unwrap(),
                   Expression::Subtraction(Box::new(Expression::Literal(1)), Box::new(Expression::Literal(2))));
    }

    #[test]
    fn parse_1_times_4() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("1 * 4")).unwrap()).unwrap(),
                   Expression::Multiplication(Box::new(Expression::Literal(1)), Box::new(Expression::Literal(4))));
    }

    #[test]
    fn parse_1_times_parexpr_3_plus_4() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("1 * (3 + 4)")).unwrap()).unwrap(),
            Expression::Multiplication(Box::new(Expression::Literal(1)), Box::new(
                ParenthesisExpression(Box::new(
                    Addition(Box::new(Expression::Literal(3)), Box::new(Expression::Literal(4)))
                ))
            ))
        );
    }

    #[test]
    fn parse_1_times_parexpr_3_plus_4_nospace() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("1*(3+4)")).unwrap()).unwrap(),
                   Expression::Multiplication(Box::new(Expression::Literal(1)), Box::new(
                       ParenthesisExpression(Box::new(
                           Addition(Box::new(Expression::Literal(3)), Box::new(Expression::Literal(4)))
                       ))
                   ))
        );
    }

    #[test]
    fn parse_1_times_a() {
        let mut parser = Parser::new();
        parser.parse(&lex(String::from("a = 3")).unwrap()).unwrap();  // Required, else we got an undefined symbol exception
        assert_eq!(parser.parse(&lex(String::from("1 + a")).unwrap()).unwrap(), Expression::Addition(Box::new(Expression::Literal(1)), Box::new(Variable(String::from("a")))))
    }

    #[test]
    fn parse_nested_parentheses() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("(1 +  2) *  3")).unwrap()).unwrap(),
                   Expression::Multiplication(
                       Box::new(
                           ParenthesisExpression(Box::new(
                               Addition(
                                   Box::new(Expression::Literal(1)),
                                   Box::new(Expression::Literal(2))
                               )
                           ))
                       ),
                       Box::new(Expression::Literal(3))
                   )
        );
    }

    #[test]
    fn parse_nested_parentheses_with_precedence() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("1 + (2 *  3)")).unwrap()).unwrap(),
                   Expression::Addition(
                       Box::new(Expression::Literal(1            )),
                       Box::new(
                           ParenthesisExpression(Box::new(
                               Multiplication(
                                   Box::new(Expression::Literal(2)),
                                   Box::new(Expression::Literal(3))
                               )
                           ))
                       )
                   )
        );
    }

    #[test]
    fn parse_nested_parentheses_with_precedence_and_unary() {
        let mut parser = Parser::new();
        assert_eq!(parser.parse(&lex(String::from("1 + (-2 *  3)")).unwrap()).unwrap(),
                   Expression::Addition(
                       Box::new(Expression::Literal(1)),
                       Box::new(
                           ParenthesisExpression(Box::new(
                               Multiplication(
                                   Box::new(UnaryMinus(Box::new(Expression::Literal(2)))),
                                   Box::new(Expression::Literal(3))
                               )
                           ))
                       )
                   )
        );
    }

    #[test]
    fn parse_nested_parentheses_with_precedence_and_unary_fail() {
        let mut parser = Parser::new();
        assert!(parser.parse(&lex(String::from("1 + (-2 *  3")).unwrap()).is_err());
    }


}