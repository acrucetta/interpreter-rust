pub mod token {
    use core::fmt;

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Eq, Hash, Ord, Serialize, Deserialize, PartialOrd, PartialEq)]
    pub enum Token {
        // Special tokens
        Illegal,
        Eof,

        // Identifiers
        Ident(String),

        // Literals
        Int(i32),
        String(String),
        Boolean(bool),

        // Operators
        Assign,   // =
        Plus,     // +
        Minus,    // -
        Bang,     // !
        Asterisk, // *
        Slash,    // /
        NotEq,    // !=
        Eq,       // ==
        Lt,       // <
        Gt,       // >

        // Delimiters
        Comma,
        Semicolon,
        LParen,
        RParen,
        LBrace,
        RBrace,
        LBracket,
        RBracket,

        // Keywords
        Fn,
        Let,
        If,
        Else,
        Return,
    }

    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Token::Illegal => write!(f, "ILLEGAL"),
                Token::Eof => write!(f, "EOF"),
                Token::Return => write!(f, "return"),
                Token::Boolean(b) => write!(f, "{}", b),
                Token::String(s) => write!(f, "{}", s),
                Token::Ident(s) => write!(f, "{}", s),
                Token::Int(i) => write!(f, "{}", i),
                Token::Assign => write!(f, "="),
                Token::Plus => write!(f, "+"),
                Token::Minus => write!(f, "-"),
                Token::Bang => write!(f, "!"),
                Token::Asterisk => write!(f, "*"),
                Token::Slash => write!(f, "/"),
                Token::NotEq => write!(f, "!="),
                Token::Eq => write!(f, "=="),
                Token::Lt => write!(f, "<"),
                Token::Gt => write!(f, ">"),
                Token::Comma => write!(f, ","),
                Token::Semicolon => write!(f, ";"),
                Token::LParen => write!(f, "("),
                Token::RParen => write!(f, ")"),
                Token::LBrace => write!(f, "{{"),
                Token::RBrace => write!(f, "}}"),
                Token::LBracket => write!(f, "["),
                Token::RBracket => write!(f, "]"),
                Token::Fn => write!(f, "fn"),
                Token::Let => write!(f, "let"),
                Token::If => write!(f, "if"),
                Token::Else => write!(f, "else"),
            }
        }
    }

    impl TryFrom<char> for Token {
        type Error = String;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '=' => Ok(Token::Assign),
                '+' => Ok(Token::Plus),
                '-' => Ok(Token::Minus),
                '!' => Ok(Token::Bang),
                '*' => Ok(Token::Asterisk),
                '/' => Ok(Token::Slash),
                ',' => Ok(Token::Comma),
                ';' => Ok(Token::Semicolon),
                '(' => Ok(Token::LParen),
                ')' => Ok(Token::RParen),
                '{' => Ok(Token::LBrace),
                '}' => Ok(Token::RBrace),
                '<' => Ok(Token::Lt),
                '>' => Ok(Token::Gt),
                '\0' => Ok(Token::Eof),
                _ => Err(format!("Unknown character: {}", value)),
            }
        }
    }

    pub fn lookup_ident(ident: &str) -> Token {
        match ident {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(ident.to_string()),
        }
    }
}
