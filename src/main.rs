use crate::lexer::lex;
use crate::parser::Parser;

mod test;
mod lexer;
mod parser;
mod interpreter;

fn main() {
    let mut parser = Parser::new();  // initializing context
    parser.parse(&lex("a = 4").unwrap()).unwrap();
    parser.parse(&lex("b = 2").unwrap()).unwrap();
    parser.parse(&lex("x = 8").unwrap()).unwrap();
    println!("{:?}", parser.parse(&lex("1 + 3 * (a + b / x) / 2").unwrap()).unwrap());
}
