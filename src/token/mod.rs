pub mod token {

    pub type TokenType<'a> = &'a str;

    #[derive(Debug, PartialEq)]
    pub struct Token {
        pub kind: TokenType<'static>,
        pub literal: String,
    }

    impl Token {
        pub fn new(kind: TokenType<'static>, literal: String) -> Token {
            Token {
                kind,
                literal: literal,
            }
        }
    }

    pub fn lookup_ident(ident: &str) -> TokenType<'static> {
        match ident {
            "fn" => FUNCTION,
            "let" => LET,
            "true" => TRUE,
            "false" => FALSE,
            "if" => IF,
            "else" => ELSE,
            "return" => RETURN,
            _ => IDENT,
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
