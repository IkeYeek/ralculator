#[cfg(test)]
mod tokenizer {
    use crate::{Token, Tokenizer, TokenType};
    use crate::{Literal, Symbol};
    use crate::TokenType::Identifier;

    #[test]
    fn tokenize_1_plus_2() {
        let sample_string = "1 + 2";
        let wanted_sequence = vec![Token {
            token_type: Literal,
            raw_value: "1".into(),
        }, Token {
            token_type: Symbol,
            raw_value: "+".into(),
        }, Token {
            token_type: Literal,
            raw_value: "2".into(),
        }];
        assert_eq!(wanted_sequence, Tokenizer::tokenize_line(sample_string.into()));
    }

    #[test]
    fn no_whitespace() {
        let sample_string = "1+1*4(2+a)/2^4";
        let wanted_sequence = vec![
            Token { token_type: Literal, raw_value: "1".into() },
            Token { token_type: Symbol, raw_value: "+".into() },
            Token { token_type: Literal, raw_value: "1".into() },
            Token { token_type: Symbol, raw_value: "*".into() },
            Token { token_type: Literal, raw_value: "4".into() },
            Token { token_type: Symbol, raw_value: "(".into() },
            Token { token_type: Literal, raw_value: "2".into() },
            Token { token_type: Symbol, raw_value: "+".into() },
            Token { token_type: Identifier, raw_value: "a".into() },
            Token { token_type: Symbol, raw_value: ")".into() },
            Token { token_type: Symbol, raw_value: "/".into() },
            Token { token_type: Literal, raw_value: "2".into() },
            Token { token_type: Symbol, raw_value: "^".into() },
            Token { token_type: Literal, raw_value: "4".into() },

        ];
        assert_eq!(wanted_sequence, Tokenizer::tokenize_line(sample_string.into()));
    }

}