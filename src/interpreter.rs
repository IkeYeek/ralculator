use std::collections::HashMap;
use crate::parser::ast::Expression;
use crate::parser::ast::Expression::{ Assignment, Literal };

#[derive(Clone)]
pub(crate) struct Interpreter {
    mem: HashMap<String, Expression>,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }

    pub(crate) fn interpret(&mut self, ast: Expression) -> Result<f64, String> {
        match ast {
            Assignment(identifier, expr) => {
                self.mem
                    .entry(identifier)
                    .and_modify(|val| {
                        *val = *expr.clone();
                    })
                    .or_insert(*expr.clone());
                Ok(self.interpret(*expr.clone())?)
            }
            Expression::Addition(left, right) => {
                Ok(self.interpret(*left)? + self.interpret(*right)?)
            }
            Expression::Subtraction(left, right) => {
                Ok(self.interpret(*left)? - self.interpret(*right)?)
            }
            Expression::UnaryPlus(expr) => {
                Ok(0f64 + self.interpret(*expr)?) // Let's pretend it is somehow useful
            }
            Expression::UnaryMinus(expr) => { Ok(0f64 - self.interpret(*expr)?) }
            Expression::ParenthesisExpression(expr) => { Ok(self.interpret(*expr)?) }
            Expression::Multiplication(left, right) => {
                Ok(self.interpret(*left)? * self.interpret(*right)?)
            }
            Expression::Division(left, right) => {
                let right_operand = self.interpret(*right)?;
                if right_operand == 0.0 {
                    Err(String::from("Cannot divide by 0."))
                } else {
                    Ok(self.interpret(*left)? / right_operand)
                }
            }
            Literal(value) => { Ok(value) }
            Expression::Variable(identifier) => {
                if let Some(expr) = self.mem.get(identifier.as_str()) {
                    Ok(self.interpret(expr.clone())?)
                } else {
                    Err(format!("Variable {identifier} not found"))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter::Interpreter;
    use crate::lexer::lex;
    use crate::parser::Parser;

    #[test]
    fn interpret_1_plus_1() {
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();
        let tokens = lex("1 + 1").unwrap();
        let res = interpreter.interpret(parser.parse(&tokens).unwrap()).unwrap();
        assert_eq!(res, 2.0);
    }

    #[test]
    fn interpret_longer_numbers() {
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();
        let tokens = lex("12 + 28.6 - 23.41 * 2.3").unwrap();
        let res = interpreter.interpret(parser.parse(&tokens).unwrap()).unwrap();
        assert_eq!(format!("{res:.3}"), "-13.243");
    }

    #[test]
    fn cannot_divide_by_zero() {
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();
        let tokens = lex("1/0").unwrap();
        let res = interpreter.interpret(parser.parse(&tokens).unwrap());
        assert!(res.is_err());
    }

    #[test]
    fn keeps_track_of_vars() {
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(parser.parse(&lex("a = 3").unwrap()).unwrap()).unwrap();
        let v = interpreter.interpret(parser.parse(&lex("a").unwrap()).unwrap()).unwrap();
        assert_eq!(v, 3.0);
        interpreter.interpret(parser.parse(&lex("a = 8").unwrap()).unwrap()).unwrap();
        let v = interpreter.interpret(parser.parse(&lex("a").unwrap()).unwrap()).unwrap();
        assert_eq!(v, 8.0);
    }
}
