#[cfg(test)]
mod tests {
    use ralculator::expression::interpreter::Interpreter;
    use ralculator::expression::lexer::Lexer;
    use ralculator::expression::parser::Parser;

    #[test]
    fn interpret_1_plus_1() {
        let lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();
        let tokens = lexer.lex("1 + 1").unwrap();
        let res = interpreter
            .interpret(parser.parse(&tokens).unwrap())
            .unwrap();
        assert_eq!(res, 2.0);
    }

    #[test]
    fn interpret_longer_numbers() {
        let lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();
        let tokens = lexer.lex("12 + 28.6 - 23.41 * 2.3").unwrap();
        let res = interpreter
            .interpret(parser.parse(&tokens).unwrap())
            .unwrap();
        assert_eq!(format!("{res:.3}"), "-13.243");
    }

    #[test]
    fn cannot_divide_by_zero() {
        let lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();
        let tokens = lexer.lex("1/0").unwrap();
        let res = interpreter.interpret(parser.parse(&tokens).unwrap());
        assert!(res.is_err());
    }

    #[test]
    fn keeps_track_of_vars() {
        let lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();
        interpreter
            .interpret(parser.parse(&lexer.lex("a = 3").unwrap()).unwrap())
            .unwrap();
        let v = interpreter
            .interpret(parser.parse(&lexer.lex("a").unwrap()).unwrap())
            .unwrap();
        assert_eq!(v, 3.0);
        interpreter
            .interpret(parser.parse(&lexer.lex("a = 8").unwrap()).unwrap())
            .unwrap();
        let v = interpreter
            .interpret(parser.parse(&lexer.lex("a").unwrap()).unwrap())
            .unwrap();
        assert_eq!(v, 8.0);
    }

    #[test]
    fn crash_circular_ref_simple() {
        let lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();
        interpreter
            .interpret(parser.parse(&lexer.lex("a = 3").unwrap()).unwrap())
            .unwrap();
        assert!(interpreter
            .interpret(parser.parse(&lexer.lex("a = a").unwrap()).unwrap())
            .is_err());
    }

    #[test]
    fn crash_circular_ref() {
        let lexer = Lexer::new();
        let mut parser = Parser::new();
        let mut interpreter = Interpreter::new();
        interpreter
            .interpret(parser.parse(&lexer.lex("a = 3").unwrap()).unwrap())
            .unwrap();
        interpreter
            .interpret(parser.parse(&lexer.lex("b = a").unwrap()).unwrap())
            .unwrap();
        assert!(interpreter
            .interpret(parser.parse(&lexer.lex("a = b").unwrap()).unwrap())
            .is_err());
    }
}