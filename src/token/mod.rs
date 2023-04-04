pub mod token {
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

    pub const ILLEGAL: TokenKind = TokenKind::Illegal;
    pub const EOF: TokenKind = TokenKind::Eof;
    pub const IDENT: TokenKind = TokenKind::Ident;
    pub const INT: TokenKind = TokenKind::Int;
    pub const ASSIGN: TokenKind = TokenKind::Assign;
    pub const PLUS: TokenKind = TokenKind::Plus;
    pub const MINUS: TokenKind = TokenKind::Minus;
    pub const BANG: TokenKind = TokenKind::Bang;
    pub const ASTERISK: TokenKind = TokenKind::Asterisk;
    pub const SLASH: TokenKind = TokenKind::Slash;
    pub const NOT_EQ: TokenKind = TokenKind::NotEq;
    pub const EQ: TokenKind = TokenKind::Eq;
    pub const LT: TokenKind = TokenKind::Lt;
    pub const GT: TokenKind = TokenKind::Gt;
    pub const COMMA: TokenKind = TokenKind::Comma;
    pub const SEMICOLON: TokenKind = TokenKind::Semicolon;
    pub const LPAREN: TokenKind = TokenKind::LParen;
    pub const RPAREN: TokenKind = TokenKind::RParen;
    pub const LBRACE: TokenKind = TokenKind::LBrace;
    pub const RBRACE: TokenKind = TokenKind::RBrace;
    pub const FUNCTION: TokenKind = TokenKind::Function;
    pub const LET: TokenKind = TokenKind::Let;
    pub const TRUE: TokenKind = TokenKind::True;
    pub const FALSE: TokenKind = TokenKind::False;
    pub const IF: TokenKind = TokenKind::If;
    pub const ELSE: TokenKind = TokenKind::Else;
    pub const RETURN: TokenKind = TokenKind::Return;
}
