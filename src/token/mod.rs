pub mod token {

    pub type TokenType<'a> = &'a str;

    pub struct Token {
        pub kind: TokenType<'static>,
        pub literal: String,
    }

    impl Token {
        pub fn new(kind: TokenType<'static>, literal: char) -> Token {
            Token {
                kind,
                literal: literal.to_string(),
            }
        }
    }

    pub fn lookup_ident(ident: &str) -> TokenType<'static> {
        match ident {
            "fn" => FUNCTION,
            "let" => LET,
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
}
