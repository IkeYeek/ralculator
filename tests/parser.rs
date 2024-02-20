#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod parser {
        use ralculator::expression::lexer::Lexer;
        use ralculator::expression::parser::ast::Expression;
        use ralculator::expression::parser::ast::Expression::{
            Addition, Multiplication, ParenthesisExpression, UnaryMinus, Variable,
        };
        use ralculator::expression::parser::Parser;

        #[test]
        fn parse_assign() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("a = 1").unwrap()).unwrap(),
                Expression::Assignment("a".into(), Box::new(Expression::Literal(1.0)))
            )
        }

        #[test]
        fn parse_1() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("1").unwrap()).unwrap(),
                Expression::Literal(1f64)
            )
        }

        #[test]
        fn parse_23() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("23").unwrap()).unwrap(),
                Expression::Literal(23f64)
            )
        }

        #[test]
        fn parse_minus_1() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("-1").unwrap()).unwrap(),
                UnaryMinus(Box::new(Expression::Literal(1f64)))
            )
        }

        #[test]
        fn parse_1_plus_1() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("1 + 1").unwrap()).unwrap(),
                Addition(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(Expression::Literal(1f64))
                )
            );
        }

        #[test]
        fn parse_1_minus_2() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("1 - 2").unwrap()).unwrap(),
                Expression::Subtraction(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(Expression::Literal(2f64))
                )
            );
        }

        #[test]
        fn parse_1_times_4() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("1 * 4").unwrap()).unwrap(),
                Multiplication(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(Expression::Literal(4f64))
                )
            );
        }

        #[test]
        fn parse_1_times_parexpr_3_plus_4() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("1 * (3 + 4)").unwrap()).unwrap(),
                Multiplication(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(ParenthesisExpression(Box::new(Addition(
                        Box::new(Expression::Literal(3f64)),
                        Box::new(Expression::Literal(4f64))
                    ))))
                )
            );
        }

        #[test]
        fn parse_1_times_parexpr_3_plus_4_nospace() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("1*(3+4)").unwrap()).unwrap(),
                Multiplication(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(ParenthesisExpression(Box::new(Addition(
                        Box::new(Expression::Literal(3f64)),
                        Box::new(Expression::Literal(4f64))
                    ))))
                )
            );
        }

        #[test]
        fn parse_1_times_a() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            parser.parse(&lexer.lex("a = 3").unwrap()).unwrap(); // Required, else we got an undefined symbol exception
            assert_eq!(
                parser.parse(&lexer.lex("1 + a").unwrap()).unwrap(),
                Addition(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(Variable(String::from("a")))
                )
            );
        }

        #[test]
        fn parse_undefined_variable() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert!(parser.parse(&lexer.lex("a + b").unwrap()).is_err());
        }

        #[test]
        fn parse_nested_parentheses() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("(1 +  2) *  3").unwrap()).unwrap(),
                Multiplication(
                    Box::new(ParenthesisExpression(Box::new(Addition(
                        Box::new(Expression::Literal(1f64)),
                        Box::new(Expression::Literal(2f64))
                    )))),
                    Box::new(Expression::Literal(3f64))
                )
            );
        }

        #[test]
        fn parse_nested_parentheses_with_precedence() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("1 + (2 *  3)").unwrap()).unwrap(),
                Addition(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(ParenthesisExpression(Box::new(Multiplication(
                        Box::new(Expression::Literal(2f64)),
                        Box::new(Expression::Literal(3f64))
                    ))))
                )
            );
        }

        #[test]
        fn parse_minus_1_minus_minus_1() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("-1 - -1").unwrap()).unwrap(),
                Expression::Subtraction(
                    Box::new(UnaryMinus(Box::new(Expression::Literal(1f64)))),
                    Box::new(UnaryMinus(Box::new(Expression::Literal(1f64))))
                )
            )
        }

        #[test]
        fn parse_nested_parentheses_with_precedence_and_unary() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lexer.lex("1 + (-2 *  3)").unwrap()).unwrap(),
                Addition(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(ParenthesisExpression(Box::new(Multiplication(
                        Box::new(UnaryMinus(Box::new(Expression::Literal(2f64)))),
                        Box::new(Expression::Literal(3f64))
                    ))))
                )
            );
        }

        #[test]
        fn parse_nested_parentheses_with_precedence_and_unary_fail() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert!(parser.parse(&lexer.lex("1 + (-2 *  3").unwrap()).is_err());
        }

        #[test]
        fn parse_empty_string() {
            let lexer = Lexer::new();
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lexer.lex("").unwrap()), Ok(Expression::Eof));
        }
    }
}