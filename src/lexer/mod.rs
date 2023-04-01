pub mod lexer {

    use crate::token;
    use crate::token::token::Token;

    pub struct Lexer {
        pub input: String,
        pub position: usize,
        pub read_position: usize,
        pub ch: char,
    }

    impl Lexer {
        pub fn new(input: String) -> Lexer {
            let mut l = Lexer {
                input: input,
                position: 0,
                read_position: 0,
                ch: 0 as char,
            };
            l.read_char();

            return l;
        }

        pub fn read_char(&mut self) {
            if self.read_position >= self.input.len() {
                self.ch = 0 as char;
            } else {
                self.ch = self.input.chars().nth(self.read_position).unwrap();
            }
            self.position = self.read_position;
            self.read_position += 1;
        }

        pub fn next_token(&mut self) -> Token {
            let tok: Token;

            self.skip_whitespace();

            match self.ch {
                '=' => tok = Token::new(token::token::ASSIGN, self.ch),
                ';' => tok = Token::new(token::token::SEMICOLON, self.ch),
                '(' => tok = Token::new(token::token::LPAREN, self.ch),
                ')' => tok = Token::new(token::token::RPAREN, self.ch),
                ',' => tok = Token::new(token::token::COMMA, self.ch),
                '+' => tok = Token::new(token::token::PLUS, self.ch),
                '{' => tok = Token::new(token::token::LBRACE, self.ch),
                '}' => tok = Token::new(token::token::RBRACE, self.ch),
                _ => {
                    if is_letter(self.ch) {
                        let literal = self.read_identifier();
                        let kind = token::token::lookup_ident(&literal.to_string());
                        return Token::new(kind, literal);
                    } else if is_digit(self.ch) {
                        return Token::new(token::token::INT, self.ch);
                    } else {
                        tok = Token::new(token::token::ILLEGAL, self.ch);
                        return tok;
                    }
                }
            }
            self.read_char();
            return tok;
        }

        fn read_identifier(&mut self) -> char {
            let position = self.position;
            while is_letter(self.ch) {
                self.read_char();
            }
            return self.input.chars().nth(position).unwrap();
        }

        fn skip_whitespace(&mut self) -> () {
            while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            }
        }
    }

    fn is_digit(ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }

    fn is_letter(ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
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
            assert_eq!(tok.literal, expected_literal);
        }
    }

    /// .
    #[test]
    fn test_next_token_function() {
        let input = "let five = 5;";

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
            assert_eq!(tok.literal, expected_literal);
        }
    }
}
