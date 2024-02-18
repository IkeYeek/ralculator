use std::collections::HashMap;
use crate::parser::ast::Expression;
use crate::parser::ast::Expression::{Assignment, Literal};

pub(crate) struct Interpreter {
    mem: HashMap<String, Expression>,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }

    pub(crate) fn interpret_ast(&mut self, ast: Expression) -> Result<f64, String> {
       match ast {
           Assignment(identifier, expr) => {
               self.mem.entry(identifier).and_modify(|val| *val = *expr.clone()).or_insert(*expr.clone());
               Ok(self.interpret_ast(*expr.clone())?)
           }
           Expression::Addition(left, right) => {
               Ok(self.interpret_ast(*left)? + self.interpret_ast(*right)? )
           }
           Expression::Subtraction(left, right) => {
               Ok(self.interpret_ast(*left)? - self.interpret_ast(*right)?)

           }
           Expression::UnaryPlus(expr) => {
               Ok(0f64 + self.interpret_ast(*expr)?)  // Let's pretend it is somehow useful
           }
           Expression::UnaryMinus(expr) => {
               Ok(0f64 - self.interpret_ast(*expr)?)
           }
           Expression::ParenthesisExpression(expr) => {
               Ok(self.interpret_ast(*expr)?)
           }
           Expression::Multiplication(left, right) => {
               Ok(self.interpret_ast(*left)? * self.interpret_ast(*right)?)
           }
           Expression::Division(left, right) => {
               Ok(self.interpret_ast(*left)? / self.interpret_ast(*right)?)
           }
           Literal(value) => { Ok(value) }
           Expression::Variable(identifier) => {
               if let Some(expr) = self.mem.get(identifier.as_str()) {
                   Ok(self.interpret_ast(expr.clone())?)
               } else {
                   Err(format!("Variable {identifier} not found"))
               }
           }
       }
    }


}