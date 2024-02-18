use crate::interpreter::Interpreter;
use crate::lexer::lex;
use crate::parser::Parser;

mod test;
mod lexer;
mod parser;
mod interpreter;

fn main() {
    let mut parser = Parser::new();
    let mut interpreter = Interpreter::new();

    let lex = lex("1 + 1").unwrap();
    println!("resp. {}", interpreter.interpret_ast(parser.parse(&lex).unwrap()).unwrap())
}
