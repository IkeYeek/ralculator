use crate::lexer::lex;
use crate::parser::Parser;

mod test;
mod lexer;
mod parser;
mod interpreter;

fn main() {
    let mut parser = Parser::new();  // initializing context

}
