pub mod lexer {

    use crate::token;
    use crate::token::token::Token;

    pub struct Lexer {
        input: String,
        position: usize,
        read_position: usize,
        ch: u8,
    }

    impl Lexer {
        pub fn new(input: String) -> Lexer {
            let mut l = Lexer {
                input: input,
                position: 0,
                read_position: 0,
                ch: 0,
            };
            l.read_char();

            return l;
        }

        fn read_char(&mut self) {
            if self.read_position >= self.input.len() {
                self.ch = 0;
            } else {
                self.ch = self.input.as_bytes()[self.read_position];
            }
            self.position = self.read_position;
            self.read_position += 1;
        }

        pub fn next_token(&mut self) -> Token {
            let tok: Token;

            match self.ch {
                b'=' => tok = Token::new(token::token::ASSIGN, self.ch),
                b';' => tok = Token::new(token::token::SEMICOLON, self.ch),
                b'(' => tok = Token::new(token::token::LPAREN, self.ch),
                b')' => tok = Token::new(token::token::RPAREN, self.ch),
                b',' => tok = Token::new(token::token::COMMA, self.ch),
                b'+' => tok = Token::new(token::token::PLUS, self.ch),
                b'{' => tok = Token::new(token::token::LBRACE, self.ch),
                b'}' => tok = Token::new(token::token::RBRACE, self.ch),
                b'\0' => tok = Token::new(token::token::EOF, self.ch),
                _ => tok = Token::new(token::token::ILLEGAL, self.ch),
            }
            self.read_char();
            return tok;
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::token::{self, token::TokenType};

    use super::*;

    #[test]
    pub fn test_next_token_small() {
        let input = "=+(){},;";

        let tests: Vec<(TokenType, &str)> = vec![
            (token::token::ASSIGN, "="),
            (token::token::PLUS, "+"),
            (token::token::LPAREN, "("),
            (token::token::RPAREN, ")"),
            (token::token::LBRACE, "{"),
            (token::token::RBRACE, "}"),
            (token::token::COMMA, ","),
            (token::token::SEMICOLON, ";"),
            (token::token::EOF, ""),
        ];

        let mut l = lexer::Lexer::new(input.to_string());

        for (expected_kind, expected_literal) in tests {
            let tok = l.next_token();

            assert_eq!(tok.kind, expected_kind);
            assert_eq!(tok.value, expected_literal);
        }
    }

    /// .
    #[test]
    fn test_next_token_function() {
        let input = "
            let five = 5;";

        let tests: Vec<(TokenType, String)> = vec![
            (token::token::LET, "let".to_string()),
            (token::token::IDENT, "five".to_string()),
            (token::token::ASSIGN, "=".to_string()),
            (token::token::INT, "5".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
        ];

        let mut l = lexer::Lexer::new(input.to_string());

        for (expected_kind, expected_literal) in tests {
            let tok = l.next_token();

            assert_eq!(tok.kind, expected_kind);
            assert_eq!(tok.value, expected_literal);
        }
    }
}
