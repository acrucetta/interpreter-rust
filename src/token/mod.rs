pub mod token {
    use core::fmt;

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Eq, Hash, Ord, Serialize, Deserialize, PartialOrd, PartialEq)]
    pub enum TokenKind {
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

    impl fmt::Display for TokenKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                TokenKind::Illegal(s) => write!(f, "{}", s),
                TokenKind::Eof => write!(f, "EOF"),
                TokenKind::Ident(s) => write!(f, "{}", s),
                TokenKind::Int(i) => write!(f, "{}", i),
                TokenKind::Assign => write!(f, "="),
                TokenKind::Plus => write!(f, "+"),
                TokenKind::Minus => write!(f, "-"),
                TokenKind::Bang => write!(f, "!"),
                TokenKind::Asterisk => write!(f, "*"),
                TokenKind::Slash => write!(f, "/"),
                TokenKind::NotEq => write!(f, "!="),
                TokenKind::Eq => write!(f, "=="),
                TokenKind::Lt => write!(f, "<"),
                TokenKind::Gt => write!(f, ">"),
                TokenKind::Comma => write!(f, ","),
                TokenKind::Semicolon => write!(f, ";"),
                TokenKind::LParen => write!(f, "("),
                TokenKind::RParen => write!(f, ")"),
                TokenKind::LBrace => write!(f, "{{"),
                TokenKind::RBrace => write!(f, "}}"),
                TokenKind::Function => write!(f, "fn"),
                TokenKind::Let => write!(f, "let"),
                TokenKind::True => write!(f, "true"),
                TokenKind::False => write!(f, "false"),
                TokenKind::If => write!(f, "if"),
                TokenKind::Else => write!(f, "else"),
                TokenKind::Return => write!(f, "return"),
            }
        }
    }

    impl TryFrom<char> for TokenKind {
        type Error = String;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '=' => Ok(TokenKind::Assign),
                '+' => Ok(TokenKind::Plus),
                '-' => Ok(TokenKind::Minus),
                '!' => Ok(TokenKind::Bang),
                '*' => Ok(TokenKind::Asterisk),
                '/' => Ok(TokenKind::Slash),
                ',' => Ok(TokenKind::Comma),
                ';' => Ok(TokenKind::Semicolon),
                '(' => Ok(TokenKind::LParen),
                ')' => Ok(TokenKind::RParen),
                '{' => Ok(TokenKind::LBrace),
                '}' => Ok(TokenKind::RBrace),
                '<' => Ok(TokenKind::Lt),
                '>' => Ok(TokenKind::Gt),
                _ => Err(format!("Unknown character: {}", value)),
            }
        }
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, Serialize, Deserialize, PartialOrd, PartialEq)]
    pub struct Token {
        pub kind: TokenKind,
        pub literal: String,
    }

    impl Token {
        pub fn new(kind: TokenKind, literal: String) -> Token {
            Token {
                kind,
                literal: literal,
            }
        }
    }

    pub fn lookup_ident(ident: &str) -> TokenKind {
        match ident {
            "fn" => TokenKind::Function,
            "let" => TokenKind::Let,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            _ => TokenKind::Ident(ident.to_string()),
        }
    }
}
