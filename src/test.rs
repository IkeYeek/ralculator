#[cfg(test)]
mod tests {


    #[cfg(test)]
    mod parser {
        use crate::lexer::lex;
        use crate::parser::ast::Expression;
        use crate::parser::{Parser};
        use crate::parser::ast::Expression::{Addition, Multiplication, ParenthesisExpression, UnaryMinus, Variable};

        #[test]
        fn parse_assign() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("a = 1").unwrap()).unwrap(),
                       Expression::Assignment("a".into(), Box::new(Expression::Literal(1))))
        }

        #[test]
        fn parse_1() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("1").unwrap()).unwrap(), Expression::Literal(1))
        }

        #[test]
        fn parse_23() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("23").unwrap()).unwrap(), Expression::Literal(23))
        }

        #[test] fn parse_minus_1() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("-1").unwrap()).unwrap(), UnaryMinus(Box::new(Expression::Literal(1))))
        }

        #[test]
        fn parse_1_plus_1() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("1 + 1").unwrap()).unwrap(),
                       Addition(Box::new(Expression::Literal(1)), Box::new(Expression::Literal(1))));
        }

        #[test]
        fn parse_1_minus_2() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("1 - 2").unwrap()).unwrap(),
                       Expression::Subtraction(Box::new(Expression::Literal(1)), Box::new(Expression::Literal(2))));
        }

        #[test]
        fn parse_1_times_4() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("1 * 4").unwrap()).unwrap(),
                       Expression::Multiplication(Box::new(Expression::Literal(1)), Box::new(Expression::Literal(4))));
        }

        #[test]
        fn parse_1_times_parexpr_3_plus_4() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("1 * (3 + 4)").unwrap()).unwrap(),
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
            assert_eq!(parser.parse(&lex("1*(3+4)").unwrap()).unwrap(),
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
            parser.parse(&lex("a = 3").unwrap()).unwrap();  // Required, else we got an undefined symbol exception
            assert_eq!(parser.parse(&lex("1 + a").unwrap()).unwrap(), Expression::Addition(Box::new(Expression::Literal(1)), Box::new(Variable(String::from("a")))));
        }

        #[test]
        fn parse_undefined_variable() {
            let mut parser = Parser::new();
            assert!(parser.parse(&lex("a + b").unwrap()).is_err());
        }

        #[test]
        fn parse_nested_parentheses() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("(1 +  2) *  3").unwrap()).unwrap(),
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
            assert_eq!(parser.parse(&lex("1 + (2 *  3)").unwrap()).unwrap(),
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
        fn parse_minus_1_minus_minus_1() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("-1 - -1").unwrap()).unwrap(), Expression::Subtraction(
                Box::new(UnaryMinus(Box::new(Expression::Literal(1)))), Box::new(UnaryMinus(Box::new(Expression::Literal(1))))
            ))
        }

        #[test]
        fn parse_nested_parentheses_with_precedence_and_unary() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("1 + (-2 *  3)").unwrap()).unwrap(),
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
            assert!(parser.parse(&lex("1 + (-2 *  3").unwrap()).is_err());
        }

        #[test]
        fn parse_empty_string() {
            let mut parser = Parser::new();
            assert!(parser.parse(&lex("").unwrap()).is_err());
        }
    }

    #[cfg(test)]
    mod interpreter {
        use crate::interpreter::Interpreter;
        use crate::lexer::lex;
        use crate::parser::Parser;

        #[test]
        fn interpret_assignment() {
            let mut parser = Parser::new();
            let mut interpreter = Interpreter::new();
            assert_eq!(interpreter.interpret_ast(parser.parse(&lex("a = 2").unwrap()).unwrap()).unwrap(), 2);
            assert_eq!(interpreter.interpret_ast(parser.parse(&lex("b = a + 2").unwrap()).unwrap()).unwrap(), 4);
            assert_eq!(interpreter.interpret_ast(parser.parse(&lex("a").unwrap()).unwrap()).unwrap(), 2);
            assert_eq!(interpreter.interpret_ast(parser.parse(&lex("b").unwrap()).unwrap()).unwrap(), 4);
            assert_eq!(interpreter.interpret_ast(parser.parse(&lex("a = 4").unwrap()).unwrap()).unwrap(), 4);
            assert_eq!(interpreter.interpret_ast(parser.parse(&lex("b").unwrap()).unwrap()).unwrap(), 6);
        }

        #[test]
        fn interpret_parenthesed_expr() {
            let mut parser = Parser::new();
            let mut interpreter = Interpreter::new();
            assert_eq!(interpreter.interpret_ast(parser.parse(&lex("1 + 2 * 4").unwrap()).unwrap()).unwrap(), 9);
            assert_eq!(interpreter.interpret_ast(parser.parse(&lex("1 + 2 * 4 + 3 - (4 + 6 * 4)").unwrap()).unwrap()).unwrap(), -16);
        }
    }
}