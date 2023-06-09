pub mod lexer {

    use crate::token::token::Token;

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

            l
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

        pub fn next_token(&mut self) -> Result<Token, String> {
            let token: Token;

            self.skip_whitespace();

            match self.ch {
                '=' => {
                    if self.peek_char() == '=' {
                        self.read_char();
                        token = Token::Eq;
                    } else {
                        token = Token::Assign;
                    }
                }
                '!' => {
                    if self.peek_char() == '=' {
                        self.read_char();
                        token = Token::NotEq;
                    } else {
                        token = Token::Bang;
                    }
                }
                ';' => token = Token::Semicolon,
                '(' => token = Token::LParen,
                ')' => token = Token::RParen,
                ',' => token = Token::Comma,
                '+' => token = Token::Plus,
                '-' => token = Token::Minus,
                '/' => token = Token::Slash,
                '*' => token = Token::Asterisk,
                '<' => token = Token::Lt,
                '>' => token = Token::Gt,
                '{' => token = Token::LBrace,
                '}' => token = Token::RBrace,
                '[' => token = Token::LBracket,
                ']' => token = Token::RBracket,
                '\0' => token = Token::Eof,
                '"' => {
                    self.read_char();
                    let start_position = self.position;
                    let mut end_position = start_position;
                    while self.ch != '"' {
                        self.read_char();
                        end_position += 1;
                    }
                    token = Token::String(self.input[start_position..end_position].to_string());
                }
                _ => {
                    if self.ch.is_alphabetic() {
                        return Ok(self.read_keyword_or_ident());
                    } else if self.ch.is_numeric() {
                        let n = self.read_number();
                        return Ok(n);
                    } else {
                        return Ok(Token::Illegal);
                    }
                }
            }
            self.read_char();
            Ok(token)
        }

        fn peek_char(&self) -> char {
            if self.read_position >= self.input.len() {
                0 as char
            } else {
                return self.input.chars().nth(self.read_position).unwrap();
            }
        }

        fn read_keyword_or_ident(&mut self) -> Token {
            let start_position = self.position;
            let mut end_position = start_position;
            while self.ch.is_alphanumeric() {
                self.read_char();
                end_position += 1;
            }
            match &self.input[start_position..end_position] {
                "fn" => Token::Fn,
                "let" => Token::Let,
                "if" => Token::If,
                "else" => Token::Else,
                "return" => Token::Return,
                "true" => Token::Boolean(true),
                "false" => Token::Boolean(false),
                ident => Token::Ident(ident.to_string()),
            }
        }

        fn skip_whitespace(&mut self) {
            while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            }
        }

        fn read_number(&mut self) -> Token {
            let start_position = self.position;
            let mut end_position = start_position;

            while self.ch.is_numeric() {
                self.read_char();
                end_position += 1;
            }
            match self.input[start_position..end_position].parse() {
                Ok(num) => Token::Int(num),
                Err(_msg) => Token::Illegal,
            }
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::token::token::Token;

    use super::{lexer::Lexer, *};

    #[test]
    pub fn test_return_token() {
        let input = "return 5;";

        let tests: Vec<Token> = vec![Token::Return, Token::Int(5), Token::Semicolon, Token::Eof];

        let mut lexer = Lexer::new(input);
        let mut data = Vec::new();
        loop {
            let token = lexer.next_token().expect("token");
            data.push(token.clone());
            if token == Token::Eof {
                break;
            }
        }
        assert_eq!(data, tests);
    }

    #[test]
    pub fn test_next_token_small() {
        let input = "=+(){},;";

        let tests: Vec<Token> = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut l = lexer::Lexer::new(input);
        let mut data: Vec<Token> = Vec::new();
        while let Ok(tok) = l.next_token() {
            if tok == Token::Eof {
                data.push(tok);
                break;
            }
            data.push(tok);
        }
        assert_eq!(data, tests);
    }

    /// .
    #[test]
    fn test_next_token_assignment() {
        let input = "let five = 5;";

        let tests: Vec<Token> = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);
        let mut data = Vec::new();
        loop {
            let token = lexer.next_token().expect("token");
            data.push(token.clone());
            if token == Token::Eof {
                break;
            }
        }
        assert_eq!(data, tests);
    }

    #[test]
    fn test_eq_token() {
        let input = "5 == 5;";

        let tests: Vec<Token> = vec![
            Token::Int(5),
            Token::Eq,
            Token::Int(5),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);
        let mut data = Vec::new();
        loop {
            let token = lexer.next_token().expect("token");
            data.push(token.clone());
            if token == Token::Eof {
                break;
            }
        }
        assert_eq!(data, tests);
    }

    #[test]
    fn test_boolean_expression() {
        let input = "true;";

        let tests: Vec<Token> = vec![Token::Boolean(true), Token::Semicolon, Token::Eof];

        let mut lexer = Lexer::new(input);
        let mut data = Vec::new();
        loop {
            let token = lexer.next_token().expect("token");
            data.push(token.clone());
            if token == Token::Eof {
                break;
            }
        }
        assert_eq!(data, tests);
    }
}
