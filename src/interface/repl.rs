use std::io;
use std::io::Write;
use crate::expressions::interpreter::Interpreter;
use crate::expressions::lexer::Lexer;
use crate::expressions::parser::Parser;
pub struct Repl {
    lexer: Lexer,
    parser: Parser,
    interpreter: Interpreter,
}

impl Repl {
    pub fn new(lexer: Lexer, parser: Parser, interpreter: Interpreter) -> Self {
        Self {
            lexer,
            parser,
            interpreter,
        }
    }

    fn looper(&mut self) -> Result<(), String> {
        let mut line_buffer = String::new();
        print!("> ");
        io::stdout().flush().map_err(|err| err.to_string())?;
        io::stdin().read_line(&mut line_buffer).map_err(|e| e.to_string())?;
        if line_buffer.is_empty() {
            Ok(())
        } else {
            let tokens = self.lexer.lex(&line_buffer).map_err(|err| format!("Lexer error: {}", err.to_string()))?;
            let ast = self.parser.parse(&tokens).map_err(|err| format!("Parser error: {}", err.to_string()))?;
            let result = self.interpreter.interpret(ast).map_err(|err| format!("Interpreter error: {}", err.to_string()))?;
            println!("= {result}");
            self.looper()
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        Repl::greet();
        self.looper()
    }

    fn tabs(n: usize) -> String {
        (0..n).fold(String::new(), |acc, _| acc + "  ")
    }

    fn greet() {
        println!("=== Interactive mathematical expression calculator ===");
        println!("{}Usage:", Self::tabs(1));
        println!("{}- Supported operator: +, -, *, /.", Self::tabs(2));
        println!(
            "{}- Supports assigning expressions to variables.",
            Self::tabs(2)
        );
        println!("{}- Supports parenthesis expression.", Self::tabs(2));
        println!("{}Press enter on an empty line to exit!", Self::tabs(1));
    }
}
