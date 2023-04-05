pub mod token {
    use core::fmt;

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Eq, Hash, Ord, Serialize, Deserialize, PartialOrd, PartialEq)]
    pub enum TokenKind {
        Illegal,
        Eof,
        Ident,
        Int,
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
                TokenKind::Illegal => write!(f, "ILLEGAL"),
                TokenKind::Eof => write!(f, "EOF"),
                TokenKind::Ident => write!(f, "IDENT"),
                TokenKind::Int => write!(f, "INT"),
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
                TokenKind::Function => write!(f, "FUNCTION"),
                TokenKind::Let => write!(f, "LET"),
                TokenKind::True => write!(f, "TRUE"),
                TokenKind::False => write!(f, "FALSE"),
                TokenKind::If => write!(f, "IF"),
                TokenKind::Else => write!(f, "ELSE"),
                TokenKind::Return => write!(f, "RETURN"),
                TokenKind::NotEq => write!(f, "NOT_EQ"),
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
            _ => TokenKind::Ident,
        }
    }
}
