use crate::cli::{Cli, Mode};
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::repl::Repl;
use clap::Parser;

mod cli;
pub mod interpreter;
pub mod lexer;
pub mod parser;
mod repl;

fn main() {
    let lexer = Lexer::new();
    let mut parser = parser::Parser::new();
    let mut interpreter = Interpreter::new();
    let cli = Cli::parse();

    let program = || match cli.mode {
        Mode {
            interactive: true,
            exec: None,
        } => {
            let mut repl = Repl::new(lexer, parser, interpreter);
            repl.run()
        }
        Mode {
            interactive: false,
            exec: Some(raw_expr),
        } => {
            let tokens = lexer.lex(&raw_expr)?;
            let ast = parser.parse(&tokens)?;
            let result = interpreter.interpret(ast)?;
            println!("{raw_expr} = {result}");
            Ok(())
        }
        _ => Err(String::from("wtf")),
    };

    match program() {
        Ok(()) => println!("Goodbye."),
        Err(e) => eprintln!("{e}"),
    }
}
