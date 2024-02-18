use crate::lexer::{ Token, TokenKind };
use crate::parser::ast::Expression;
use crate::lexer::TokenKind::{ Operator, Separator };
use crate::parser::ast::Expression::{ Assignment, Literal, UnaryMinus, UnaryPlus, Variable };

#[derive(Debug)]
struct TokenStream {
    buffer: Vec<Token>,
    cursor: usize,
}

impl TokenStream {
    pub(crate) fn new(buffer: Vec<Token>) -> Self {
        Self {
            buffer,
            cursor: 0,
        }
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

pub(crate) mod ast {
    #[derive(Debug, PartialEq, Clone)]
    pub(crate) enum Expression {
        Assignment(String, Box<Expression>),

        Addition(Box<Expression>, Box<Expression>),
        Subtraction(Box<Expression>, Box<Expression>),
        UnaryPlus(Box<Expression>),
        UnaryMinus(Box<Expression>),

        ParenthesisExpression(Box<Expression>),

        Multiplication(Box<Expression>, Box<Expression>),
        Division(Box<Expression>, Box<Expression>),
        Literal(f64),
        Variable(String),
    }
}

pub(crate) struct Parser {
    symbol_table: Vec<String>,
    tokens: TokenStream,
}

impl Parser {
    pub(crate) fn new() -> Self {
        Self {
            symbol_table: Vec::new(),
            tokens: TokenStream::new(Vec::new()),
        }
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        match self.tokens.curr() {
            Some(token) =>
                match token.kind {
                    Operator =>
                        match token.raw_value.as_str() {
                            "+" => {
                                self.tokens.next();
                                match self.parse_factor() {
                                    Ok(factor) => Ok(UnaryPlus(Box::new(factor))),
                                    Err(e) => Err(e),
                                }
                            }
                            "-" => {
                                self.tokens.next();
                                match self.parse_factor() {
                                    Ok(factor) => Ok(UnaryMinus(Box::new(factor))),
                                    Err(e) => Err(e),
                                }
                            }
                            _ => Err(format!("Unexpected operator {token:?}")),
                        }
                    TokenKind::Literal => {
                        let literal_value = token.raw_value
                            .parse::<f64>()
                            .map_err(|_| "Couldn't parse token to an integer")?;
                        self.tokens.next();
                        Ok(Literal(literal_value))
                    }

                    TokenKind::Identifier => {
                        if self.symbol_table.contains(&token.raw_value) {
                            let res = Ok(Variable(token.raw_value.clone()));
                            self.tokens.next();
                            res
                        } else {
                            Err(format!("Couldn't find symbol {}", token.raw_value))
                        }
                    }
                    Separator => {
                        if let "(" = token.raw_value.as_str() {
                            self.tokens.next();
                            let expr = self.parse_expr();
                            match expr {
                                Ok(expr) => {
                                    match self.tokens.curr() {
                                        Some(token) => {
                                            if
                                                token.kind == Separator &&
                                                token.raw_value.as_str() == ")"
                                            {
                                                self.tokens.next();
                                                Ok(
                                                    Expression::ParenthesisExpression(
                                                        Box::new(expr)
                                                    )
                                                )
                                            } else {
                                                Err(
                                                    format!(
                                                        "Expected ')', got {:?}, which is definitely not ')'",
                                                        token.raw_value
                                                    )
                                                )
                                            }
                                        }
                                        None =>
                                            Err(String::from("Expected ')', got nothing bruuuuh")),
                                    }
                                }
                                Err(e) => Err(e),
                            }
                        } else {
                            Err(format!("Expected a '(' got {:?}", token.raw_value))
                        }
                    }
                }
            None => Err(String::from("Expected a factor, got nothing")),
        }
    }

    fn parse_term_prime(&mut self, left: Expression) -> Result<Expression, String> {
        match self.tokens.curr() {
            Some(token) => {
                match token.kind {
                    Operator => {
                        match token.raw_value.as_str() {
                            "*" => {
                                self.tokens.next();
                                let factor = self.parse_factor();
                                match factor {
                                    Ok(factor) => {
                                        self.parse_term_prime(
                                            Expression::Multiplication(
                                                Box::from(left),
                                                Box::from(factor)
                                            )
                                        )
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            "/" => {
                                self.tokens.next();
                                let factor = self.parse_factor();
                                match factor {
                                    Ok(factor) => {
                                        self.parse_term_prime(
                                            Expression::Division(Box::from(left), Box::from(factor))
                                        )
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            _ => Ok(left),
                        }
                    }
                    _ => Ok(left),
                }
            }
            None => Ok(left),
        }
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        self.parse_factor().and_then(|factor| self.parse_term_prime(factor))
    }

    fn parse_expr_prime(&mut self, left: Expression) -> Result<Expression, String> {
        match self.tokens.curr() {
            Some(token) => {
                match token.kind {
                    Operator => {
                        match token.raw_value.as_str() {
                            "+" => {
                                self.tokens.next();
                                let right = self.parse_term()?;
                                self.parse_expr_prime(
                                    Expression::Addition(Box::new(left), Box::new(right))
                                )
                            }
                            "-" => {
                                self.tokens.next();
                                let right = self.parse_term()?;
                                self.parse_expr_prime(
                                    Expression::Subtraction(Box::new(left), Box::new(right))
                                )
                            }
                            _ => Ok(left),
                        }
                    }
                    _ => Ok(left),
                }
            }
            None => Ok(left),
        }
    }

    fn parse_expr(&mut self) -> Result<Expression, String> {
        self.parse_term().and_then(|term| self.parse_expr_prime(term))
    }

    fn parse_assignment(&mut self) -> Result<Expression, String> {
        match self.tokens.curr() {
            Some(idt_token) if idt_token.kind == TokenKind::Identifier => {
                let idt_token_clone = idt_token.clone();
                match self.tokens.next() {
                    Some(assignment_token) if
                        assignment_token.kind == Operator &&
                        assignment_token.raw_value.as_str() == "="
                    => {
                        self.tokens.next();
                        let maybe_expr = self.parse_expr();
                        match maybe_expr {
                            Ok(expr) => {
                                if !self.symbol_table.contains(&idt_token_clone.raw_value) {
                                    self.symbol_table.push(idt_token_clone.clone().raw_value);
                                }
                                Ok(Assignment(idt_token_clone.raw_value, Box::new(expr)))
                            }
                            Err(e) => Err(e),
                        }
                    }
                    _ => Err(String::from("Expected an = after the identifier")),
                }
            }
            _ => Err(String::from("Expected an identifier")),
        }
    }

    pub(crate) fn parse(&mut self, line: &[Token]) -> Result<Expression, String> {
        self.tokens = TokenStream::new(line.to_vec());
        if let Some(token) = self.tokens.curr() {
            match token.kind {
                TokenKind::Identifier =>
                    match self.tokens.lookahead() {
                        Some(lookahead) => {
                            match lookahead.kind {
                                Operator => if let "=" = lookahead.raw_value.as_str() {
                                    self.parse_assignment()
                                } else {
                                    self.parse_expr()
                                }
                                _ =>
                                    Err(
                                        format!(
                                            "Expected calculus or assignment operator, got {token:?}"
                                        )
                                    ),
                            }
                        }
                        None => self.parse_expr(),
                    }
                Separator | TokenKind::Literal => self.parse_expr(),
                Operator => {
                    match token.raw_value.as_str() {
                        "+" | "-" => self.parse_expr(),
                        _ => Err(format!("Expected + or -, got {token:?}")),
                    }
                }
            }
        } else {
            Err(String::from("Trying to parse an empty string"))
        }
    }
}
#[cfg(test)]
mod test {
    #[cfg(test)]
    mod parser {
        use crate::lexer::lex;
        use crate::parser::ast::Expression;
        use crate::parser::{ Parser };
        use crate::parser::ast::Expression::{
            Addition,
            Multiplication,
            ParenthesisExpression,
            UnaryMinus,
            Variable,
        };

        #[test]
        fn parse_assign() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("a = 1").unwrap()).unwrap(),
                Expression::Assignment("a".into(), Box::new(Expression::Literal(1.0)))
            )
        }

        #[test]
        fn parse_1() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("1").unwrap()).unwrap(), Expression::Literal(1f64))
        }

        #[test]
        fn parse_23() {
            let mut parser = Parser::new();
            assert_eq!(parser.parse(&lex("23").unwrap()).unwrap(), Expression::Literal(23f64))
        }

        #[test]
        fn parse_minus_1() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("-1").unwrap()).unwrap(),
                UnaryMinus(Box::new(Expression::Literal(1f64)))
            )
        }

        #[test]
        fn parse_1_plus_1() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("1 + 1").unwrap()).unwrap(),
                Addition(Box::new(Expression::Literal(1f64)), Box::new(Expression::Literal(1f64)))
            );
        }

        #[test]
        fn parse_1_minus_2() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("1 - 2").unwrap()).unwrap(),
                Expression::Subtraction(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(Expression::Literal(2f64))
                )
            );
        }

        #[test]
        fn parse_1_times_4() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("1 * 4").unwrap()).unwrap(),
                Multiplication(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(Expression::Literal(4f64))
                )
            );
        }

        #[test]
        fn parse_1_times_parexpr_3_plus_4() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("1 * (3 + 4)").unwrap()).unwrap(),
                Multiplication(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(
                        ParenthesisExpression(
                            Box::new(
                                Addition(
                                    Box::new(Expression::Literal(3f64)),
                                    Box::new(Expression::Literal(4f64))
                                )
                            )
                        )
                    )
                )
            );
        }

        #[test]
        fn parse_1_times_parexpr_3_plus_4_nospace() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("1*(3+4)").unwrap()).unwrap(),
                Multiplication(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(
                        ParenthesisExpression(
                            Box::new(
                                Addition(
                                    Box::new(Expression::Literal(3f64)),
                                    Box::new(Expression::Literal(4f64))
                                )
                            )
                        )
                    )
                )
            );
        }

        #[test]
        fn parse_1_times_a() {
            let mut parser = Parser::new();
            parser.parse(&lex("a = 3").unwrap()).unwrap(); // Required, else we got an undefined symbol exception
            assert_eq!(
                parser.parse(&lex("1 + a").unwrap()).unwrap(),
                Addition(Box::new(Expression::Literal(1f64)), Box::new(Variable(String::from("a"))))
            );
        }

        #[test]
        fn parse_undefined_variable() {
            let mut parser = Parser::new();
            assert!(parser.parse(&lex("a + b").unwrap()).is_err());
        }

        #[test]
        fn parse_nested_parentheses() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("(1 +  2) *  3").unwrap()).unwrap(),
                Multiplication(
                    Box::new(
                        ParenthesisExpression(
                            Box::new(
                                Addition(
                                    Box::new(Expression::Literal(1f64)),
                                    Box::new(Expression::Literal(2f64))
                                )
                            )
                        )
                    ),
                    Box::new(Expression::Literal(3f64))
                )
            );
        }

        #[test]
        fn parse_nested_parentheses_with_precedence() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("1 + (2 *  3)").unwrap()).unwrap(),
                Addition(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(
                        ParenthesisExpression(
                            Box::new(
                                Multiplication(
                                    Box::new(Expression::Literal(2f64)),
                                    Box::new(Expression::Literal(3f64))
                                )
                            )
                        )
                    )
                )
            );
        }

        #[test]
        fn parse_minus_1_minus_minus_1() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("-1 - -1").unwrap()).unwrap(),
                Expression::Subtraction(
                    Box::new(UnaryMinus(Box::new(Expression::Literal(1f64)))),
                    Box::new(UnaryMinus(Box::new(Expression::Literal(1f64))))
                )
            )
        }

        #[test]
        fn parse_nested_parentheses_with_precedence_and_unary() {
            let mut parser = Parser::new();
            assert_eq!(
                parser.parse(&lex("1 + (-2 *  3)").unwrap()).unwrap(),
                Addition(
                    Box::new(Expression::Literal(1f64)),
                    Box::new(
                        ParenthesisExpression(
                            Box::new(
                                Multiplication(
                                    Box::new(UnaryMinus(Box::new(Expression::Literal(2f64)))),
                                    Box::new(Expression::Literal(3f64))
                                )
                            )
                        )
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
}
