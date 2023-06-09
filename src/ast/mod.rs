use self::ast::{Expression, Statement};

pub mod ast {

    use core::fmt;

    use crate::token::token::Token;

    use std::fmt::Formatter;

    use super::{format_expressions, format_statements};

    pub type BlockStatement = Vec<Statement>;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Node {
        Program(Vec<Statement>),
        Statement(Statement),
        Expr(Expression),
    }

    impl fmt::Display for Node {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Node::Program(p) => write!(f, "{}", format_statements(p)),
                Node::Statement(s) => write!(f, "{}", s),
                Node::Expr(i) => write!(f, "{}", i),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Statement {
        Let(String, Expression),
        Return(Expression),
        Expr(Expression),
    }

    impl fmt::Display for Statement {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Statement::Let(id, expr) => write!(f, "let {} = {};", id, expr),
                Statement::Return(expr) => write!(f, "return {};", expr),
                Statement::Expr(expr) => write!(f, "{}", expr),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Expression {
        Identifier(String),
        Lit(Literal),
        Prefix(Token, Box<Expression>),
        Infix(Token, Box<Expression>, Box<Expression>),
        Postfix(Token, Box<Expression>),
        If(Box<Expression>, BlockStatement, Option<BlockStatement>),
        Fn(Vec<String>, BlockStatement),
        Call(Box<Expression>, Vec<Expression>),
    }

    impl fmt::Display for Expression {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Expression::Identifier(s) => write!(f, "{}", s),
                Expression::Lit(l) => write!(f, "{}", l),
                Expression::Prefix(op, expr) => write!(f, "({}{})", op, expr),
                Expression::Infix(op, e1, e2) => write!(f, "({} {} {})", e1, op, e2),
                Expression::Postfix(op, e) => write!(f, "({}{})", e, op),
                Expression::If(cond, cons, alt) => {
                    if let Some(alt) = alt {
                        write!(
                            f,
                            "if {} {{ {} }} else {{ {} }}",
                            cond,
                            format_statements(cons),
                            format_statements(alt)
                        )
                    } else {
                        write!(f, "if {} {{ {} }}", cond, format_statements(cons))
                    }
                }
                Expression::Fn(params, _body) => {
                    write!(f, "fn({}) {{...}}", params.join(", "))
                }
                Expression::Call(fn_expr, args) => {
                    write!(f, "{}({})", fn_expr, format_expressions(args))
                }
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Literal {
        Int(i32),
        String(String),
        Bool(bool),
        Array(Vec<Expression>),
        Hash(Vec<(Expression, Expression)>),
    }

    impl fmt::Display for Literal {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Literal::Int(i) => write!(f, "{}", i),
                Literal::String(s) => write!(f, "{}", s),
                Literal::Bool(b) => write!(f, "{}", b),
                Literal::Array(a) => write!(f, "[{}]", format_expressions(a)),
                Literal::Hash(h) => {
                    let mut result = String::new();
                    for (k, v) in h {
                        result.push_str(&format!("{}: {}", k, v));
                    }
                    write!(f, "{{{}}}", result)
                }
            }
        }
    }
}

fn format_statements(stmts: &[Statement]) -> String {
    stmts
        .iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn format_expressions(expressions: &[Expression]) -> String {
    expressions
        .iter()
        .map(|stmt| stmt.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

#[cfg(test)]
mod test {

    use super::ast::{Expression, Statement};

    #[test]
    fn display() {
        let p = vec![Statement::Let(
            "asdf".to_string(),
            Expression::Identifier("bar".to_string()),
        )];

        let expected = "let asdf = bar;";

        for stmt in p {
            assert_eq!(stmt.to_string(), expected);
        }
    }
}
