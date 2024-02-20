use crate::interpreter::Interpreter;
use crate::lexer::{Lexer};
use crate::parser::Parser;
use std::io;
use std::io::Write;

pub(crate) struct Repl {
    lexer: Lexer,
    parser: Parser,
    interpreter: Interpreter,
}

impl Repl {
    pub(crate) fn new() -> Self {
        Self {
            parser: Parser::new(),
            interpreter: Interpreter::new(),
            lexer: Lexer::new(),
        }
    }
    pub(crate) fn run(&mut self) -> Result<(), String> {
        Repl::greet();
        let mut line_buffer = String::new();
        let mut eof = false;
        while !eof {
            line_buffer.clear();
            print!("> ");
            io::stdout().flush().map_err(|e| e.to_string())?;
            eof = io::stdin()
                .read_line(&mut line_buffer)
                .map_err(|err| err.to_string())?
                == 1; // As we are reading lines, if we only had 1 character it's a newline (let's say we live in a world with only LF, perfect world imo)
            if eof {
                println!();
                continue;
            }
            match self.lexer.lex(&line_buffer) {
                Ok(tokens) => match self.parser.parse(&tokens) {
                    Ok(ast) => {
                        let result = self.interpreter.interpret(ast);
                        match result {
                            Ok(result) => println!("= {result}"),
                            Err(e) => eprintln!("{e}"),
                        }
                    }
                    Err(e) => eprintln!("{e}"),
                },
                Err(e) => eprintln!("{e}"),
            }
        }
        Ok(())
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
