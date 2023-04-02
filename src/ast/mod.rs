pub mod ast {

    use crate::token::token::{Token, TokenType};

    // Defining the interface for the AST
    pub trait Node {
        fn token_literal(&self) -> String;
    }

    pub trait Statement: Node {
        fn statement_node(&self);
    }

    pub trait Expression: Node {
        fn expression_node(&self);
    }

    struct Program {
        statements: Vec<Box<dyn Statement>>,
    }

    impl Program {
        pub fn token_literal(&mut self) -> String {
            if self.statements.len() > 0 {
                self.statements[0].token_literal()
            } else {
                "".to_string()
            }
        }
    }
    struct LetStatement {
        token: Token,
        name: Identifier,
        value: Box<dyn Expression>,
    }

    impl LetStatement {
        pub fn statement_node() {}

        pub fn token_literal(&self) -> String {
            self.token.literal.clone()
        }
    }

    struct Identifier {
        token: Token,
        value: String,
    }

    impl Identifier {
        pub fn expression_node() {}

        pub fn token_literal(&self) -> String {
            self.token.literal.clone()
        }
    }
}
