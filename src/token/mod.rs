pub mod token {

    use serde::{Deserialize, Serialize};
    use std::fmt;
    use std::fmt::Formatter;

    pub type TokenType = String;

    #[derive(Clone, Debug, Eq, Hash, Ord, Serialize, Deserialize, PartialOrd, PartialEq)]
    pub struct Token {
        pub kind: TokenType,
        pub literal: String,
    }

    impl Token {
        pub fn new(kind: TokenType, literal: String) -> Token {
            Token {
                kind,
                literal: literal,
            }
        }
    }

    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => FUNCTION.to_owned(),
            "let" => LET.to_owned(),
            "true" => TRUE.to_owned(),
            "false" => FALSE.to_owned(),
            "if" => IF.to_owned(),
            "else" => ELSE.to_owned(),
            "return" => RETURN.to_owned(),
            _ => IDENT.to_owned(),
        }
    }

    pub const ILLEGAL: &str = "ILLEGAL";
    pub const EOF: &str = "";
    // Identifiers + literals
    pub const IDENT: &str = "IDENT"; // add, foobar, x, y, ...
    pub const INT: &str = "INT"; // 1343456

    // Operators
    pub const ASSIGN: &str = "=";
    pub const PLUS: &str = "+";
    pub const MINUS: &str = "-";
    pub const BANG: &str = "!";
    pub const ASTERISK: &str = "*";
    pub const SLASH: &str = "/";

    pub const NOT_EQ: &str = "!=";
    pub const EQ: &str = "==";

    pub const LT: &str = "<";
    pub const GT: &str = ">";

    // Delimiters
    pub const COMMA: &str = ",";
    pub const SEMICOLON: &str = ";";
    pub const LPAREN: &str = "(";
    pub const RPAREN: &str = ")";
    pub const LBRACE: &str = "{";
    pub const RBRACE: &str = "}";

    // Keywords
    pub const FUNCTION: &str = "FUNCTION";
    pub const LET: &str = "LET";
    pub const TRUE: &str = "TRUE";
    pub const FALSE: &str = "FALSE";
    pub const IF: &str = "IF";
    pub const ELSE: &str = "ELSE";
    pub const RETURN: &str = "RETURN";
}
