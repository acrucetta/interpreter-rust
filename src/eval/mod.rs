pub mod environment;
pub mod error;

use std::env;
use std::rc::Rc;

use self::environment::Env;
use self::error::*;
use crate::ast::ast::{Expression, Literal, Node, Statement};
use crate::object::*;

pub type EvaluatorResult = Result<Rc<Object>, EvaluatorError>;

pub fn eval(node: Node, env: &Env) -> EvaluatorResult {
    match node {
        Node::Expr(expr) => eval_expression(expr, env),
        Node::Statement(statement) => eval_statement(statement, env),
        Node::Program(program) => eval_program(program, env),
    }
}

pub fn eval_expression(expr: Expression, env: &Env) -> EvaluatorResult {
    match expr {
        Expression::Identifier(id) => eval_identifier(&id, env),
        Expression::Prefix(_, _) => todo!(),
        Expression::Infix(_, _, _) => todo!(),
        Expression::Postfix(_, _) => todo!(),
        Expression::If(_, _, _) => todo!(),
        Expression::Fn(_, _) => todo!(),
        Expression::Call(_, _) => todo!(),
        Expression::Lit(l) => eval_literal(&l, env),
    }
}

fn eval_literal(lit: &Literal, env: &Env) -> EvaluatorResult {
    match lit {
        Literal::Int(i) => Ok(Rc::new(Object::Integer(*i))),
        Literal::String(_) => todo!(),
        Literal::Bool(_) => todo!(),
        Literal::Array(_) => todo!(),
        Literal::Hash(_) => todo!(),
    }
}

pub fn eval_identifier(id: &str, env: &Env) -> Result<Rc<Object>, EvaluatorError> {
    match env.borrow().get(id) {
        Some(obj) => Ok(obj),
        None => Err(EvaluatorError::new(format!("identifier not found: {}", id))),
    }
}

pub fn eval_statement(statement: Statement, env: &Env) -> EvaluatorResult {
    match statement {
        Statement::Let(_, _) => todo!(),
        Statement::Return(_) => todo!(),
        Statement::Expr(expr) => eval_expression(expr, env),
    }
}

pub fn eval_program(program: Vec<Statement>, env: &Env) -> EvaluatorResult {
    let mut result = Rc::new(Object::Null);

    for statement in program {
        result = eval_statement(statement, &Rc::clone(env))?;
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::ast::ast::{Expression, Node, Statement};
    use crate::lexer::lexer::Lexer;
    use crate::parser::parser::{parse, Parser};

    use super::environment::Env;
    use super::error::EvaluatorError;
    use super::eval;

    fn apply_test(test_case: &[(&str, &str)]) {
        let env: Env = Rc::new(RefCell::new(Default::default()));

        for (input, expected) in test_case {
            match parse(input) {
                Ok(node) => match eval(node, &Rc::clone(&env)) {
                    Ok(evaluated) => assert_eq!(evaluated.to_string(), expected.to_string()),
                    Err(e) => panic!("Error: {}", e),
                },
                Err(err) => panic!("Error: {:?}", err),
            }
        }
    }

    #[test]
    fn test_integer_expression() {
        let test_case = vec![("5", "5"), ("10", "10")];

        apply_test(&test_case);
    }
}
