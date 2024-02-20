use crate::expressions::parser::ast::Expression;
use crate::expressions::parser::ast::Expression::{Assignment, Literal};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Interpreter {
    mem: HashMap<String, Expression>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }

    fn is_assignation_legal(&self, identifier_name: &str, expression: &Expression) -> bool {
        match expression {
            Assignment(_, _) | Literal(_) | Expression::Eof => true,
            Expression::Variable(variable_name) => {
                identifier_name != variable_name
                    && self
                        .is_assignation_legal(identifier_name, self.mem.get(variable_name).unwrap())
            }
            Expression::Addition(left, right)
            | Expression::Subtraction(left, right)
            | Expression::Multiplication(left, right)
            | Expression::Division(left, right) => {
                self.is_assignation_legal(identifier_name, left)
                    && self.is_assignation_legal(identifier_name, right)
            }
            Expression::UnaryPlus(expr)
            | Expression::UnaryMinus(expr)
            | Expression::ParenthesisExpression(expr) => {
                self.is_assignation_legal(identifier_name, expr)
            }
        }
    }

    pub fn interpret(&mut self, ast: Expression) -> Result<f64, String> {
        match ast {
            Assignment(identifier, expr) => {
                if self.is_assignation_legal(&identifier, &expr) {
                    self.mem
                        .entry(identifier.clone())
                        .and_modify(|val| {
                            *val = *expr.clone();
                        })
                        .or_insert(*expr.clone());
                    return self.interpret(*expr.clone());
                }
                Err(String::from("Illegal assignation"))
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
            Expression::UnaryMinus(expr) => Ok(0f64 - self.interpret(*expr)?),
            Expression::ParenthesisExpression(expr) => Ok(self.interpret(*expr)?),
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
            Literal(value) => Ok(value),
            Expression::Variable(identifier) => {
                if let Some(expr) = self.mem.get(identifier.as_str()) {
                    Ok(self.interpret(expr.clone())?)
                } else {
                    Err(format!("Variable {identifier} not found"))
                }
            }
            Expression::Eof => {
                Err(String::from("EOF"))
            }
        }
    }
}
