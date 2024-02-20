use ralculator::expressions::lexer::Lexer;
use clap::Parser;
use ralculator::expressions::interpreter::Interpreter;
use ralculator::interface::cli::{Cli, Mode};
use ralculator::interface::repl::Repl;

fn program() -> Result<(), String> {
    let lexer = Lexer::new();
    let mut parser = ralculator::expressions::parser::Parser::new();
    let mut interpreter = Interpreter::new();
    let cli = Cli::parse();
    match cli.mode {
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
    }
}

fn main() {
    match program() {
        Ok(()) => println!("Goodbye."),
        Err(e) => eprintln!("{e}"),
    }
}
