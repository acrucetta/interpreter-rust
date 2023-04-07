pub mod token {
    use core::fmt;

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Eq, Hash, Ord, Serialize, Deserialize, PartialOrd, PartialEq)]
    pub enum Token {
        Illegal(String),
        Eof,
        Ident(String),
        Int(i64),
        Assign,
        Plus,
        Minus,
        Bang,
        Asterisk,
        Slash,
        NotEq,
        Eq,
        Lt,
        Gt,
        Comma,
        Semicolon,
        LParen,
        RParen,
        LBrace,
        RBrace,
        Function,
        Let,
        True,
        False,
        If,
        Else,
        Return,
    }

    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Token::Illegal(s) => write!(f, "{}", s),
                Token::Eof => write!(f, "EOF"),
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
                Token::Function => write!(f, "fn"),
                Token::Let => write!(f, "let"),
                Token::True => write!(f, "true"),
                Token::False => write!(f, "false"),
                Token::If => write!(f, "if"),
                Token::Else => write!(f, "else"),
                Token::Return => write!(f, "return"),
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
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(ident.to_string()),
        }
    }

    impl Token {
        pub fn is_ident(&self) -> bool {
            match self {
                Token::Ident(_) => true,
                _ => false,
            }
        }

        pub fn is_int(&self) -> bool {
            match self {
                Token::Int(_) => true,
                _ => false,
            }
        }
    }
}
