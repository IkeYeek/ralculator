use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
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

    pub(crate) fn looper(&mut self) -> Result<(), String> {
        let mut line_buffer = String::new();
        print!("> ");
        io::stdout().flush().map_err(|err| err.to_string())?;
        io::stdin().read_line(&mut line_buffer).map_err(|e| e.to_string())?;
        match line_buffer.as_str()
        {
            "" => Ok(()),
            _ => {
                let tokens = self.lexer.lex(&line_buffer)?;
                let ast = self.parser.parse(&tokens)?;
                let result = self.interpreter.interpret(ast)?;
                println!("= {result}");
                self.looper()
            }
        }
    }

    pub(crate) fn run(&mut self) -> Result<(), String> {
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
