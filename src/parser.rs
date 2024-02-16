use std::iter::Peekable;
use std::ops::Deref;
use std::process::id;
use std::slice::Iter;
use crate::lexer::token::{Kind, Token};
use crate::parser::ast::Expression;
use crate::lexer::token::Kind::{Operator, Separator};
use crate::parser::ast::Expression::{Literal, UnaryMinus, UnaryPlus, Variable};



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
        self.buffer.get::<usize>(self.cursor.into())
    }

    pub(crate) fn next(&mut self) -> Option<&Token> {
        self.cursor = self.cursor + 1usize;
        self.curr()
    }

    pub(crate) fn lookahead(&self) -> Option<&Token> {
        self.buffer.get(self.cursor + 1)
    }
}

pub(crate) mod ast {
    #[derive(Debug, PartialEq)]
    pub(crate) enum Expression {
        Assignment(String, Box<Expression>),

        Addition(Box<Expression>, Box<Expression>),
        Subtraction(Box<Expression>, Box<Expression>),
        UnaryPlus(Box<Expression>),
        UnaryMinus(Box<Expression>),

        Multiplication(Box<Expression>, Box<Expression>),
        Division(Box<Expression>, Box<Expression>),
        Factor(Box<Expression>),
        Literal(i64),
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

    fn parse_expr(&mut self) -> Result<Expression, String> {
        todo!("parse_expr")
    }

    fn parse_assignment(&mut self) -> Result<Expression, String> {
        match self.tokens.curr() {
            Some(idt_token) if idt_token.kind == Kind::Identifier => {
                let idt_token_clone = idt_token.clone();
                match self.tokens.next() {
                    Some(assignment_token) if assignment_token.kind == Operator && assignment_token.raw_value.as_str() == "=" => {
                        match self.tokens.next() {
                            Some(literal_token) => {
                                if literal_token.kind == Kind::Literal {
                                    if let Ok(literal_value) = literal_token.raw_value.parse::<i64>() {
                                        Ok(
                                            Expression::Assignment(
                                                idt_token_clone.raw_value,
                                                Box::new(Literal(literal_value))
                                            )
                                        )
                                    } else {
                                        Err(String::from(format!("Expected a literal integer value, got {:?}", literal_token)))
                                    }

                                } else {
                                    Err(String::from(format!("Expected a literal token, got {:?}", literal_token)))
                                }
                            },
                            None => Err(String::from("Expected a literal, got nothing")),
                        }
                    },
                    _ => Err(String::from("Expected an = after the identifier")),
                }
            },
            _ => Err(String::from("Expected an identifier")),
        }
    }

    pub(crate) fn parse(&mut self, line: &Vec<Token>) -> Result<Expression, String> {
        self.tokens = TokenStream::new(line.to_vec());
        if let Some(token) = self.tokens.curr() {
            match token.kind {
                Kind::Identifier => match self.tokens.lookahead() {
                    Some(lookahead) => {
                        match lookahead.kind {
                            Operator => if let "=" = lookahead.raw_value.as_str() {
                                self.parse_assignment()
                            } else {
                                self.parse_expr()
                            },
                            _ => Err(String::from(format!("Expected calculus or assignment operator, got {:?}", token)))
                        }
                    },
                    None => todo!("try parse expr (solo identifier are possible)")
                },
                Separator => self.parse_expr(),
                Operator => {
                    match token.raw_value.as_str() {
                        "+" | "-" => self.parse_expr(),
                        _ => Err(String::from(format!("Expected + or -, got {:?}", token)))
                    }
                }
                Kind::Literal => self.parse_expr(),
            }
        } else {
            return Err(String::from("Trying to parse an empty string"));
        }
    }
}