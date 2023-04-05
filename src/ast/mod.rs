pub mod ast {

    use core::fmt;
    use std::any::Any;

    use crate::token::token::Token;
    use crate::token::token::TokenKind;
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

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Statement {
        Let(Let),
    }

    impl fmt::Display for Statement {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Statement::Let(s) => write!(f, "{}", s),
            }
        }
    }

    impl Statement {
        pub fn token_literal(&self) -> String {
            match self {
                Statement::Let(s) => s.token_literal(),
            }
        }

        pub fn new() -> Statement {
            Statement::Let(Let::new())
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
        pub fn token_literal(&self) -> String {
            if self.statements.len() > 0 {
                self.statements[0].token_literal()
            } else {
                "".to_string()
            }
        }

        pub fn new() -> Program {
            Program { statements: vec![] }
        }
    }

    #[derive(Clone, Debug, Eq, Serialize, Deserialize, Hash, PartialEq)]
    pub struct Let {
        pub token: TokenKind,
        pub name: Identifier,
        pub value: Expression,
    }

    impl fmt::Display for Let {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{} {} = {}", self.token_literal(), self.name, self.value)
        }
    }

    impl Let {
        pub fn statement_node(&self) {}

        pub fn token_literal(&self) -> String {
            self.token.to_string()
        }

        pub fn new() -> Let {
            Let {
                token: TokenKind::Let,
                name: Identifier::new("".to_string()),
                value: Expression::Identifier(Identifier::new("".to_string())),
            }
        }
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, Serialize, Deserialize, PartialOrd, PartialEq)]
    pub struct Identifier {
        pub token: TokenKind,
        pub value: String,
    }

    impl fmt::Display for Identifier {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl Identifier {
        pub fn expression_node() {}

        pub fn token_literal(&self) -> String {
            self.token.to_string()
        }

        pub fn new(to_string: String) -> Identifier {
            Identifier {
                token: TokenKind::Ident,
                value: to_string,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        ast::{Let, Statement},
        *,
    };

    #[test]
    fn display() {
        let p = ast::Program {
            statements: vec![Statement::Let(Box::new(Let {
                name: "asdf".to_string(),
                value: Expression::Identifier("bar".to_string()),
            }))],
        };

        let expected = "let asdf = bar;";

        if p.to_string() != expected {
            panic!("expected {} but got {}", "foo", expected)
        }
    }
}
