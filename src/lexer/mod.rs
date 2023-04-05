pub mod lexer {

    use crate::token;
    use crate::token::token::{Token, TokenKind};

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
                '=' => {
                    tok = {
                        if self.peek_char() == '=' {
                            let ch = self.ch;
                            self.read_char();
                            let literal = ch.to_string() + &self.ch.to_string();
                            Token::new(TokenKind::Eq, literal)
                        } else {
                            Token::new(TokenKind::Assign, self.ch.to_string())
                        }
                    }
                }
                ';' => tok = Token::new(TokenKind::Semicolon, self.ch.to_string()),
                '(' => tok = Token::new(TokenKind::LParen, self.ch.to_string()),
                ')' => tok = Token::new(TokenKind::RParen, self.ch.to_string()),
                ',' => tok = Token::new(TokenKind::Comma, self.ch.to_string()),
                '+' => tok = Token::new(TokenKind::Plus, self.ch.to_string()),
                '{' => tok = Token::new(TokenKind::LBrace, self.ch.to_string()),
                '}' => tok = Token::new(TokenKind::RBrace, self.ch.to_string()),
                '-' => tok = Token::new(TokenKind::Minus, self.ch.to_string()),
                '!' => {
                    tok = {
                        if self.peek_char() == '=' {
                            let ch = self.ch;
                            self.read_char();
                            let literal = ch.to_string() + &self.ch.to_string();
                            Token::new(TokenKind::NotEq, literal)
                        } else {
                            Token::new(TokenKind::Bang, self.ch.to_string())
                        }
                    }
                }
                '*' => tok = Token::new(TokenKind::Asterisk, self.ch.to_string()),
                '/' => tok = Token::new(TokenKind::Slash, self.ch.to_string()),
                '<' => tok = Token::new(TokenKind::Lt, self.ch.to_string()),
                '>' => tok = Token::new(TokenKind::Gt, self.ch.to_string()),
                '\0' => tok = Token::new(TokenKind::Eof, "".to_string()),
                _ => {
                    if is_letter(self.ch) {
                        let literal = self.read_identifier();
                        let kind = token::token::lookup_ident(&literal.to_string());
                        return Token::new(kind, literal);
                    } else if is_digit(self.ch) {
                        let literal = self.read_number();
                        let kind = TokenKind::Int;
                        return Token::new(kind, literal);
                    } else {
                        tok = Token::new(TokenKind::Illegal, self.ch.to_string());
                        return tok;
                    }
                }
            }
            self.read_char();
            return tok;
        }

        fn peek_char(&self) -> char {
            if self.read_position >= self.input.len() {
                return 0 as char;
            } else {
                return self.input.chars().nth(self.read_position).unwrap();
            }
        }

        fn read_identifier(&mut self) -> String {
            let position = self.position;
            while is_letter(self.ch) {
                self.read_char();
            }
            // Return from the position to the current position
            return self.input[position..self.position].to_string();
        }

        fn skip_whitespace(&mut self) -> () {
            while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            }
        }

        fn read_number(&mut self) -> String {
            let position = self.position;
            while is_digit(self.ch) {
                self.read_char();
            }
            return self.input[position..self.position].to_string();
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
    use crate::token::token::TokenKind;

    use super::{lexer::Lexer, *};

    #[test]
    pub fn test_next_token_small() {
        let input = "=+(){},;";

        let tests: Vec<(TokenKind, String)> = vec![
            (TokenKind::Assign, "=".to_string()),
            (TokenKind::Plus, "+".to_string()),
            (TokenKind::LParen, "(".to_string()),
            (TokenKind::RParen, ")".to_string()),
            (TokenKind::LBrace, "{".to_string()),
            (TokenKind::RBrace, "}".to_string()),
            (TokenKind::Comma, ",".to_string()),
            (TokenKind::Semicolon, ";".to_string()),
            (TokenKind::Eof, "".to_string()),
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
    fn test_next_token_assignment() {
        let input = "let five = 5;";

        let tests: Vec<(TokenKind, String)> = vec![
            (TokenKind::Let, "let".to_string()),
            (TokenKind::Ident, "five".to_string()),
            (TokenKind::Assign, "=".to_string()),
            (TokenKind::Int, "5".to_string()),
            (TokenKind::Semicolon, ";".to_string()),
            (TokenKind::Eof, "".to_string()),
        ];

        let mut l = lexer::Lexer::new(input.to_string());

        for (expected_kind, expected_literal) in tests {
            let tok = l.next_token();

            assert_eq!(tok.kind, expected_kind);
            assert_eq!(tok.literal, expected_literal);
        }
    }
}
