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
        self.buffer.get(self.cursor)
    }

    pub(crate) fn next(&mut self) -> Option<&Token> {
        self.cursor +=  1;
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

        ParenthesisExpression(Box<Expression>),

        Multiplication(Box<Expression>, Box<Expression>),
        Division(Box<Expression>, Box<Expression>),
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

    fn parse_factor(&mut self) -> Result<Expression, String> {
        match self.tokens.curr() {
            Some(token) => match token.kind {
                Operator => match token.raw_value.as_str() {
                    "+" => {
                        self.tokens.next();
                        match self.parse_factor() {
                            Ok(factor) => Ok(UnaryPlus(Box::new(factor))),
                            Err(e) => Err(e)
                        }
                    },
                    "-" => {
                        self.tokens.next();
                        match self.parse_factor() {
                            Ok(factor) => Ok(UnaryMinus(Box::new(factor))),
                            Err(e) => Err(e)
                        }
                    },
                    _ => Err(String::from(format!("Unexpected operator {:?}", token)))
                },
                Kind::Literal => {
                    let literal_value = token.raw_value.parse::<i64>().map_err(|_| "Couldn't parse token to an integer")?;
                    self.tokens.next();
                    Ok(Literal(literal_value))
                }

                Kind::Identifier => {
                    if self.symbol_table.contains(&token.raw_value) {
                        let res = Ok(Variable(token.raw_value.clone()));
                        self.tokens.next();
                        res
                    } else {
                        Err(String::from(format!("Couldn't find symbol {}", token.raw_value)))
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
                                        if token.kind == Separator && token.raw_value.as_str() == ")" {
                                            self.tokens.next();
                                            Ok(
                                                Expression::ParenthesisExpression(Box::new(expr))
                                            )
                                        } else {
                                            Err(String::from(format!("Expected ')', got {:?}, which is definitly not ')'", token.raw_value)))
                                        }
                                    },
                                    None => Err(String::from("Expected ')', got nothing bruuuuh"))
                                }
                            },
                            Err(e) => Err(e)
                        }
                    } else {
                        Err(String::from(format!("Expected a '(' got {:?}", token.raw_value)))
                    }
                }
            },
            None => Err(String::from("Expected a factor, got nothing"))
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
                                                Box::from(left), Box::from(factor)
                                            )
                                        )
                                    },
                                    Err(e) => Err(e),
                                }
                            },
                            "/" => {
                                self.tokens.next();
                                let factor = self.parse_factor();
                                match factor {
                                    Ok(factor) => {
                                        self.parse_term_prime(
                                            Expression::Division(
                                                Box::from(left), Box::from(factor)
                                            )
                                        )
                                    },
                                    Err(e) => Err(e),
                                }
                            },
                            _ => Ok(left)
                        }
                    }
                    _ => Ok(left)
                }
            },
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
                                let factor = self.parse_factor();
                                match factor {
                                    Ok(factor) => {
                                        self.parse_term_prime(
                                            Expression::Addition(
                                                Box::from(left), Box::from(factor)
                                            )
                                        )
                                    },
                                    Err(e) => Err(e),
                                }
                            },
                            "-" => {
                                self.tokens.next();
                                let factor = self.parse_factor();
                                match factor {
                                    Ok(factor) => {
                                        self.parse_term_prime(
                                            Expression::Subtraction(
                                                Box::from(left), Box::from(factor)
                                            )
                                        )
                                    },
                                    Err(e) => Err(e),
                                }
                            },
                            _ => Ok(left)
                        }
                    }
                    _ => Ok(left)
                }
            },
            None => Ok(left),
        }
    }

    fn parse_expr(&mut self) -> Result<Expression, String> {
        self.parse_term().and_then(|term| self.parse_expr_prime(term))
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
                                    let literal_value = literal_token.raw_value.parse::<i64>().map_err(|_| format!("Expected a literal integer value, got {:?}", literal_token))?;
                                    let literal_name = idt_token_clone.clone().raw_value;
                                    if !self.symbol_table.contains(&literal_name) {
                                        self.symbol_table.push(idt_token_clone.clone().raw_value);
                                    }
                                    Ok(
                                        Expression::Assignment(
                                            idt_token_clone.raw_value,
                                            Box::new(Literal(literal_value))
                                        )
                                    )
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