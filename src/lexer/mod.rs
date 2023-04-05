pub mod lexer {

    use core::num;

    use crate::token;
    use crate::token::token::{Token, TokenKind};

    pub struct Lexer {
        pub input: String,
        pub position: usize,
        pub read_position: usize,
        pub ch: char,
    }

    impl Lexer {
        pub fn new(input: &str) -> Lexer {
            let mut l = Lexer {
                input: input.into(),
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

        pub fn next_token(&mut self) -> Result<TokenKind, String> {
            self.skip_whitespace();

            let token = if self.ch.is_alphabetic() {
                self.read_keyword_or_ident()
            } else if self.ch.is_numeric() {
                self.read_number()
            } else {
                let token = self.ch.try_into()?;
                self.read_char();
                token
            };
            Ok(token)
        }

        fn peek_char(&self) -> char {
            if self.read_position >= self.input.len() {
                return 0 as char;
            } else {
                return self.input.chars().nth(self.read_position).unwrap();
            }
        }

        fn read_keyword_or_ident(&mut self) -> TokenKind {
            let start_position = self.position;
            let mut end_position = start_position;
            while self.ch.is_alphanumeric() {
                self.read_char();
                end_position += 1;
            }
            match &self.input[start_position..end_position] {
                "fn" => TokenKind::Function,
                "let" => TokenKind::Let,
                ident => TokenKind::Ident(ident.to_string()),
            }
        }

        fn skip_whitespace(&mut self) -> () {
            while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            }
        }

        fn read_number(&mut self) -> TokenKind {
            let start_position = self.position;
            let mut end_position = start_position;

            while self.ch.is_numeric() {
                self.read_char();
                end_position += 1;
            }
            match self.input[start_position..end_position].parse() {
                Ok(num) => TokenKind::Int(num),
                Err(msg) => TokenKind::Illegal(msg.to_string()),
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
    use crate::token::token::TokenKind;

    use super::{lexer::Lexer, *};

    #[test]
    pub fn test_next_token_small() {
        let input = "=+(){},;";

        let tests: Vec<TokenKind> = vec![
            TokenKind::Assign,
            TokenKind::Plus,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::RBrace,
            TokenKind::Comma,
            TokenKind::Semicolon,
            TokenKind::Eof,
        ];

        let mut l = lexer::Lexer::new(input);
        let mut data: Vec<TokenKind> = Vec::new();
        while let Ok(tok) = l.next_token() {
            data.push(tok);
            if tok == TokenKind::Eof {
                break;
            }
        }
        assert_eq!(data, tests);
    }

    /// .
    #[test]
    fn test_next_token_assignment() {
        let input = "let five = 5;";

        let tests: Vec<TokenKind> = vec![
            TokenKind::Let,
            TokenKind::Ident("five".to_string()),
            TokenKind::Assign,
            TokenKind::Int(5),
            TokenKind::Semicolon,
            TokenKind::Eof,
        ];

        let mut lexer = Lexer::new(input);
        let mut data = Vec::new();
        loop {
            let token = lexer.next_token().expect("token");
            data.push(token.clone());
            if token == TokenKind::Eof {
                break;
            }
        }
        assert_eq!(data, tests);
    }
}
