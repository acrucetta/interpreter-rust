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
                '=' => {
                    tok = {
                        if self.peek_char() == '=' {
                            let ch = self.ch;
                            self.read_char();
                            let literal = ch.to_string() + &self.ch.to_string();
                            Token::new(token::token::EQ, literal)
                        } else {
                            Token::new(token::token::ASSIGN, self.ch.to_string())
                        }
                    }
                }
                ';' => tok = Token::new(token::token::SEMICOLON, self.ch.to_string()),
                '(' => tok = Token::new(token::token::LPAREN, self.ch.to_string()),
                ')' => tok = Token::new(token::token::RPAREN, self.ch.to_string()),
                ',' => tok = Token::new(token::token::COMMA, self.ch.to_string()),
                '+' => tok = Token::new(token::token::PLUS, self.ch.to_string()),
                '{' => tok = Token::new(token::token::LBRACE, self.ch.to_string()),
                '}' => tok = Token::new(token::token::RBRACE, self.ch.to_string()),
                '-' => tok = Token::new(token::token::MINUS, self.ch.to_string()),
                '!' => {
                    tok = {
                        if self.peek_char() == '=' {
                            let ch = self.ch;
                            self.read_char();
                            let literal = ch.to_string() + &self.ch.to_string();
                            Token::new(token::token::NOT_EQ, literal)
                        } else {
                            Token::new(token::token::BANG, self.ch.to_string())
                        }
                    }
                }
                '*' => tok = Token::new(token::token::ASTERISK, self.ch.to_string()),
                '/' => tok = Token::new(token::token::SLASH, self.ch.to_string()),
                '<' => tok = Token::new(token::token::LT, self.ch.to_string()),
                '>' => tok = Token::new(token::token::GT, self.ch.to_string()),
                '\0' => tok = Token::new(token::token::EOF, "".to_string()),
                _ => {
                    if is_letter(self.ch) {
                        let literal = self.read_identifier();
                        let kind = token::token::lookup_ident(&literal.to_string());
                        return Token::new(kind, literal);
                    } else if is_digit(self.ch) {
                        let literal = self.read_number();
                        let kind = token::token::INT;
                        return Token::new(kind, literal);
                    } else {
                        tok = Token::new(token::token::ILLEGAL, self.ch.to_string());
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
    use crate::token::{self, token::TokenType};

    use super::{lexer::Lexer, *};

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
    fn test_next_token_assignment() {
        let input = "let five = 5;";

        let tests: Vec<(TokenType, String)> = vec![
            (token::token::LET, "let".to_string()),
            (token::token::IDENT, "five".to_string()),
            (token::token::ASSIGN, "=".to_string()),
            (token::token::INT, "5".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::EOF, "".to_string()),
        ];

        let mut l = lexer::Lexer::new(input.to_string());

        for (expected_kind, expected_literal) in tests {
            let tok = l.next_token();

            assert_eq!(tok.kind, expected_kind);
            assert_eq!(tok.literal, expected_literal);
        }
    }

    #[test]
    fn test_next_token_assignment_function() {
        let input = "let five = 5; \
                 let ten = 10; \
                 let add = fn(x, y) { \
                   x + y; \
                 }; \
                 let result = add(five, ten); \
                 !-/*5; \
                5 < 10 > 5; \
                if (5 < 10) { \
                  return true; \
                } else { \
                  return false; \
                } \
                10 == 10; \
                10 != 9;";

        let tests: Vec<(token::token::TokenType, String)> = vec![
            (token::token::LET, "let".to_string()),
            (token::token::IDENT, "five".to_string()),
            (token::token::ASSIGN, "=".to_string()),
            (token::token::INT, "5".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::LET, "let".to_string()),
            (token::token::IDENT, "ten".to_string()),
            (token::token::ASSIGN, "=".to_string()),
            (token::token::INT, "10".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::LET, "let".to_string()),
            (token::token::IDENT, "add".to_string()),
            (token::token::ASSIGN, "=".to_string()),
            (token::token::FUNCTION, "fn".to_string()),
            (token::token::LPAREN, "(".to_string()),
            (token::token::IDENT, "x".to_string()),
            (token::token::COMMA, ",".to_string()),
            (token::token::IDENT, "y".to_string()),
            (token::token::RPAREN, ")".to_string()),
            (token::token::LBRACE, "{".to_string()),
            (token::token::IDENT, "x".to_string()),
            (token::token::PLUS, "+".to_string()),
            (token::token::IDENT, "y".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::RBRACE, "}".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::LET, "let".to_string()),
            (token::token::IDENT, "result".to_string()),
            (token::token::ASSIGN, "=".to_string()),
            (token::token::IDENT, "add".to_string()),
            (token::token::LPAREN, "(".to_string()),
            (token::token::IDENT, "five".to_string()),
            (token::token::COMMA, ",".to_string()),
            (token::token::IDENT, "ten".to_string()),
            (token::token::RPAREN, ")".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::BANG, "!".to_string()),
            (token::token::MINUS, "-".to_string()),
            (token::token::SLASH, "/".to_string()),
            (token::token::ASTERISK, "*".to_string()),
            (token::token::INT, "5".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::INT, "5".to_string()),
            (token::token::LT, "<".to_string()),
            (token::token::INT, "10".to_string()),
            (token::token::GT, ">".to_string()),
            (token::token::INT, "5".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::IF, "if".to_string()),
            (token::token::LPAREN, "(".to_string()),
            (token::token::INT, "5".to_string()),
            (token::token::LT, "<".to_string()),
            (token::token::INT, "10".to_string()),
            (token::token::RPAREN, ")".to_string()),
            (token::token::LBRACE, "{".to_string()),
            (token::token::RETURN, "return".to_string()),
            (token::token::TRUE, "true".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::RBRACE, "}".to_string()),
            (token::token::ELSE, "else".to_string()),
            (token::token::LBRACE, "{".to_string()),
            (token::token::RETURN, "return".to_string()),
            (token::token::FALSE, "false".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::RBRACE, "}".to_string()),
            (token::token::INT, "10".to_string()),
            (token::token::EQ, "==".to_string()),
            (token::token::INT, "10".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::INT, "10".to_string()),
            (token::token::NOT_EQ, "!=".to_string()),
            (token::token::INT, "9".to_string()),
            (token::token::SEMICOLON, ";".to_string()),
            (token::token::EOF, "".to_string()),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (expected_kind, expected_literal) in tests {
            let tok = lexer.next_token();

            assert_eq!(tok.kind, expected_kind);
            assert_eq!(tok.literal, expected_literal);
        }
    }
}
