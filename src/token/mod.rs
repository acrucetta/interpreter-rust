pub mod token {

    pub(crate) type TokenType<'a> = &'a str;

    pub(crate) struct Token {
        pub kind: TokenType<'static>,
        pub value: String,
    }

    impl Token {
        pub(crate) fn new(kind: TokenType<'static>, value: u8) -> Token {
            Token {
                kind: kind,
                value: value.to_string(),
            }
        }
    }

    pub const ILLEGAL: &str = "ILLEGAL";
    pub const EOF: &str = "EOF";
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
