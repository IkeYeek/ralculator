use crate::errors::parser_error::SyntaxError;
use crate::expressions::lexer::tokens::Kind::{Operator, Separator};
use crate::expressions::lexer::tokens::{Kind, Token, TokenStream};
use crate::expressions::parser::ast::Expression;
use crate::expressions::parser::ast::Expression::{Assignment, Eof, Literal, UnaryMinus, UnaryPlus, Variable};

pub mod ast {
    #[derive(Debug, PartialEq, Clone)]
    pub enum Expression {
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
        Eof,
    }
}
#[derive(Clone)]
pub struct Parser {
    symbol_table: Vec<String>,
    tokens: TokenStream,
}

impl Parser {
    #[must_use]
    pub fn new() -> Self {
        Self {
            symbol_table: Vec::new(),
            tokens: TokenStream::new(Vec::new()),
        }
    }

    fn parse_factor(&mut self) -> Result<Expression, SyntaxError> {
        match self.tokens.curr() {
            Some(token) => match token.kind {
                Operator => match token.raw_value.as_str() {
                    "+" => {
                        self.tokens.next();
                        Ok(UnaryPlus(Box::new(self.parse_factor()?)))
                    }
                    "-" => {
                        self.tokens.next();
                        Ok(UnaryMinus(Box::new(self.parse_factor()?)))
                    }
                    _ => Err(SyntaxError::new(format!("Unexpected operator {token:?}"))),
                },
                Kind::Literal => {
                    let literal_value = token
                        .raw_value
                        .parse::<f64>()
                        .map_err(|_| SyntaxError::new(String::from("Couldn't parse token to an integer")))?;
                    self.tokens.next();
                    Ok(Literal(literal_value))
                }

                Kind::Identifier => {
                    if self.symbol_table.contains(&token.raw_value) {
                        let res = Ok(Variable(token.raw_value.clone()));
                        self.tokens.next();
                        res
                    } else {
                        Err(SyntaxError::new(format!("Couldn't find symbol {}", token.raw_value)))
                    }
                }
                Separator => {
                    if let "(" = token.raw_value.as_str() {
                        self.tokens.next();
                        let expr = self.parse_expr()?;
                        match self.tokens.curr() {
                            Some(token) => {
                                if token.kind == Separator && token.raw_value.as_str() == ")" {
                                    self.tokens.next();
                                    Ok(Expression::ParenthesisExpression(Box::new(expr)))
                                } else {
                                    Err(SyntaxError::new(format!(
                                        "Expected ')', got {:?}, which is definitely not ')'",
                                        token.raw_value
                                    )))
                                }
                            }
                            None => Err(SyntaxError::new(String::from("Expected ')', got nothing bruuuuh"))),
                        }
                    } else {
                        Err(SyntaxError::new(format!("Expected a '(' got {:?}", token.raw_value)))
                    }
                }
            },
            None => Err(SyntaxError::new(String::from("Expected a factor, got nothing"))),
        }
    }

    fn parse_term_prime(&mut self, left: Expression) -> Result<Expression, SyntaxError> {
        match self.tokens.curr() {
            Some(token) => match token.kind {
                Operator => match token.raw_value.as_str() {
                    "*" => {
                        self.tokens.next();
                        let factor = self.parse_factor()?;
                        self.parse_term_prime(Expression::Multiplication(
                            Box::from(left),
                            Box::from(factor),
                        ))
                    }
                    "/" => {
                        self.tokens.next();
                        let factor = self.parse_factor()?;
                        self.parse_term_prime(Expression::Division(
                            Box::from(left),
                            Box::from(factor),
                        ))
                    }
                    _ => Ok(left),
                },
                Separator => match token.raw_value.as_str() {
                    "(" => {
                      self.tokens.next();
                        let expr = self.parse_factor()?;
                        match self.tokens.curr() {
                            Some(token) => if let ")" = token.raw_value.as_str() {
                                Ok(Expression::Multiplication(
                                    Box::from(left),
                                    Box::from(expr),
                                ))
                            } else {
                                Err(SyntaxError::new(format!("Expected ')' got {token:?}")))
                            }
                            None => Err(SyntaxError::new(String::from("Expected ')' got nothing.")))
                        }
                    },
                    ")" => Ok(left),
                    _ => Err(SyntaxError::new(String::from("??")))  // means we would have a tokenization problem...
                }
                _ => Err(SyntaxError::new(format!("unexpected token {token:?}")))
            },
            None => Ok(left),
        }
    }

    fn parse_term(&mut self) -> Result<Expression, SyntaxError> {
        self.parse_factor()
            .and_then(|factor| self.parse_term_prime(factor))
    }

    fn parse_expr_prime(&mut self, left: Expression) -> Result<Expression, SyntaxError> {
        match self.tokens.curr() {
            Some(token) => match token.kind {
                Operator => match token.raw_value.as_str() {
                    "+" => {
                        self.tokens.next();
                        let right = self.parse_term()?;
                        self.parse_expr_prime(Expression::Addition(Box::new(left), Box::new(right)))
                    }
                    "-" => {
                        self.tokens.next();
                        let right = self.parse_term()?;
                        self.parse_expr_prime(Expression::Subtraction(
                            Box::new(left),
                            Box::new(right),
                        ))
                    }
                    _ => Ok(left),
                },
                _ => Ok(left),
            },
            None => Ok(left),
        }
    }

    fn parse_expr(&mut self) -> Result<Expression, SyntaxError> {
        self.parse_term()
            .and_then(|term| self.parse_expr_prime(term))
    }

    fn parse_assignment(&mut self) -> Result<Expression, SyntaxError> {
        match self.tokens.curr() {
            Some(idt_token) if idt_token.kind == Kind::Identifier => {
                let idt_token_clone = idt_token.clone();
                match self.tokens.next() {
                    Some(assignment_token)
                    if assignment_token.kind == Operator
                        && assignment_token.raw_value.as_str() == "=" =>
                        {
                            self.tokens.next();
                            if !self.symbol_table.contains(&idt_token_clone.raw_value) {
                                self.symbol_table.push(idt_token_clone.clone().raw_value);
                            }
                            Ok(Assignment(
                                idt_token_clone.raw_value,
                                Box::new(self.parse_expr()?),
                            ))
                        }
                    _ => Err(SyntaxError::new(String::from("Expected an = after the identifier"))),
                }
            }
            _ => Err(SyntaxError::new(String::from("Expected an identifier"))),
        }
    }


    /// # Errors
    ///
    /// Will return an error if it fails creating AST from tokens
    pub fn parse(&mut self, line: &[Token]) -> Result<Expression, SyntaxError> {
        self.tokens = TokenStream::new(line.to_vec());
        if let Some(token) = self.tokens.curr() {
            match token.kind {
                Kind::Identifier => match self.tokens.lookahead() {
                    Some(lookahead) => match lookahead.kind {
                        Operator => {
                            if let "=" = lookahead.raw_value.as_str() {
                                self.parse_assignment()
                            } else {
                                self.parse_expr()
                            }
                        }
                        _ => Err(SyntaxError::new(format!(
                            "Expected calculus or assignment operator, got {token:?}"
                        ))),
                    },
                    None => self.parse_expr(),
                },
                Separator | Kind::Literal => self.parse_expr(),
                Operator => match token.raw_value.as_str() {
                    "+" | "-" => self.parse_expr(),
                    _ => Err(SyntaxError::new(format!("Expected + or -, got {token:?}"))),
                },
            }
        } else {
            Ok(Eof)
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
