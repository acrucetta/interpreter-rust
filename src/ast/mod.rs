pub mod ast {

    use core::fmt;
    use std::any::Any;

    use crate::token::token::{Token, IDENT};
    use serde::{Deserialize, Serialize};
    use std::fmt::Formatter;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Node {
        Program(Program),
        Statement(Statement),
        Identifier(Identifier),
    }

    impl fmt::Display for Node {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Node::Program(p) => write!(f, "{}", p),
                Node::Statement(s) => write!(f, "{}", s),
                Node::Identifier(i) => write!(f, "{}", i),
            }
        }
    }

    impl Node {
        pub fn token_literal(&self) -> &str {
            match self {
                Node::Program(p) => p.token_literal(),
                Node::Statement(s) => s.token_literal(),
                Node::Identifier(i) => i.token_literal(),
            }
        }
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Statement {
        LetStatement(LetStatement),
    }

    impl fmt::Display for Statement {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Statement::LetStatement(s) => write!(f, "{}", s),
            }
        }
    }

    impl Statement {
        pub fn token_literal(&self) -> &str {
            match self {
                Statement::LetStatement(s) => &s.token_literal(),
            }
        }
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, Serialize, Deserialize, PartialOrd, PartialEq)]
    pub enum Expression {
        Identifier(Identifier),
    }

    impl fmt::Display for Expression {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Expression::Identifier(i) => write!(f, "{}", i),
            }
        }
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Program {
        pub statements: Vec<Statement>,
    }

    impl fmt::Display for Program {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let mut result = String::new();
            for s in &self.statements {
                result.push_str(&s.token_literal());
            }
            write!(f, "{}", result)
        }
    }

    impl Program {
        pub fn token_literal(&self) -> &str {
            if self.statements.len() > 0 {
                self.statements[0].token_literal()
            } else {
                ""
            }
        }
    }

    #[derive(Clone, Debug, Eq, Serialize, Deserialize, Hash, PartialEq)]
    pub struct LetStatement {
        pub token: Token,
        pub name: Identifier,
        pub value: Expression,
    }

    impl fmt::Display for LetStatement {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{} {} = {}", self.token_literal(), self.name, self.value)
        }
    }

    impl LetStatement {
        pub fn statement_node(&self) {}

        pub fn token_literal(&self) -> &str {
            self.token.literal.as_str()
        }
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, Serialize, Deserialize, PartialOrd, PartialEq)]
    pub struct Identifier {
        pub token: Token,
        pub value: String,
    }

    impl fmt::Display for Identifier {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl Identifier {
        pub fn expression_node() {}

        pub fn token_literal(&self) -> &str {
            &self.token.literal
        }

        pub fn new(to_string: String) -> Identifier {
            Identifier {
                token: Token::new(IDENT, to_string.clone()),
                value: to_string,
            }
        }
    }
}
