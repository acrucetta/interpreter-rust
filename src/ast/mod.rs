pub mod ast {

    use core::fmt;

    use crate::token::token::Token;
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
        Return(Return),
        Expr(Expr),
    }

    impl fmt::Display for Statement {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Statement::Let(s) => write!(f, "let {} = {};", s.name, s.value),
                Statement::Return(s) => write!(f, "return {};", s.return_value),
                Statement::Expr(expr) => write!(f, "{}", expr.expression),
            }
        }
    }

    impl Statement {
        pub fn token_literal(&self) -> String {
            match self {
                Statement::Let(s) => s.token_literal(),
                Statement::Return(s) => s.token_literal(),
                Statement::Expr(s) => s.token_literal(),
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

    impl Expression {
        pub fn token_literal(&self) -> String {
            match self {
                Expression::Identifier(i) => i.token_literal(),
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
            if !self.statements.is_empty() {
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
        pub token: Token,
        pub name: Identifier,
        pub value: Expression,
    }

    impl Let {
        pub fn token_literal(&self) -> String {
            self.token.to_string()
        }

        pub fn new() -> Let {
            Let {
                token: Token::Let,
                name: Identifier::new("".to_string()),
                value: Expression::Identifier(Identifier::new("".to_string())),
            }
        }
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, Serialize, Deserialize, PartialOrd, PartialEq)]
    pub struct Return {
        pub token: Token,
        pub return_value: Expression,
    }

    impl Return {
        pub fn token_literal(&self) -> String {
            self.token.to_string()
        }

        pub fn new() -> Return {
            Return {
                token: Token::Return,
                return_value: Expression::Identifier(Identifier::new("".to_string())),
            }
        }
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Expr {
        pub token: Token,
        pub expression: Expression,
    }

    impl Expr {
        pub fn token_literal(&self) -> String {
            self.token.to_string()
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
        pub fn token_literal(&self) -> String {
            self.token.to_string()
        }

        pub fn new(to_string: String) -> Identifier {
            Identifier {
                token: Token::Ident(to_string.clone()),
                value: to_string,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::token::token::Token;

    use super::{
        ast::{Expression, Identifier, Let, Program, Statement},
        *,
    };

    #[test]
    fn display() {
        let p = Program {
            statements: vec![Statement::Let(Let {
                token: Token::Let,
                name: Identifier::new("asdf".to_string()),
                value: Expression::Identifier(Identifier::new("bar".to_string())),
            })],
        };

        let expected = "let asdf = bar;";

        if p.to_string() != expected {
            panic!("expected {} but got {}", expected, p.to_string())
        }
    }
}
