pub mod environment;
pub mod error;

use std::env;
use std::rc::Rc;

use self::environment::Env;
use self::error::*;
use crate::ast::ast::{Expression, Literal, Node, Statement};
use crate::object::*;
use crate::token::token::Token;

pub type EvaluatorResult = Result<Rc<Object>, EvaluatorError>;

pub fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(b) => *b,
        _ => true,
    }
}

pub fn eval(node: Node, env: &Env) -> EvaluatorResult {
    match node {
        Node::Expr(expr) => eval_expression(&expr, env),
        Node::Statement(statement) => eval_statement(&statement, env),
        Node::Program(program) => eval_program(program, env),
    }
}

pub fn eval_expression(expr: &Expression, env: &Env) -> EvaluatorResult {
    match expr {
        Expression::Identifier(id) => eval_identifier(&id, env),
        Expression::Lit(l) => eval_literal(&l, env),
        Expression::Prefix(op, expr) => {
            let right = eval_expression(expr, env)?;
            eval_prefix_expression(op, &right)
        }
        Expression::Infix(op, left, right) => {
            let left = eval_expression(left, env)?;
            let right = eval_expression(right, env)?;
            eval_infix_expression(op, &left, &right)
        }
        Expression::Postfix(_, _) => todo!(),
        Expression::If(condition, consequence, alternative) => {
            let condition = eval_expression(condition, &Rc::clone(env))?;

            if is_truthy(&condition) {
                eval_block_statement(consequence, env)
            } else {
                match alternative {
                    Some(alt) => eval_block_statement(alt, env),
                    None => Ok(Rc::new(Object::Null)),
                }
            }
        }
        Expression::Fn(_, _) => todo!(),
        Expression::Call(_, _) => todo!(),
    }
}

fn eval_block_statement(statements: &[Statement], env: &Env) -> EvaluatorResult {
    let mut result = Rc::new(Object::Null);

    for statement in statements {
        let val = eval_statement(statement, env)?;

        match val.as_ref() {
            Object::ReturnValue(_) => return Ok(val),
            _ => result = val,
        }
    }
    Ok(result)
}

fn eval_infix_expression(op: &Token, left: &Object, right: &Object) -> EvaluatorResult {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix_expression(op, *l, *r),
        (Object::Boolean(l), Object::Boolean(r)) => eval_boolean_infix_expression(op, *l, *r),
        _ => Err(EvaluatorError::new(format!(
            "type mismatch: {} {} {}",
            left, op, right
        ))),
    }
}

fn eval_boolean_infix_expression(op: &Token, l: bool, r: bool) -> EvaluatorResult {
    match op {
        Token::Eq => Ok(Rc::new(Object::Boolean(l == r))),
        Token::NotEq => Ok(Rc::new(Object::Boolean(l != r))),
        _ => Err(EvaluatorError::new(format!(
            "unknown operator: {} {} {}",
            l, op, r
        ))),
    }
}

fn eval_integer_infix_expression(op: &Token, l: i32, r: i32) -> EvaluatorResult {
    let left_val = Rc::new(Object::Integer(l));
    let right_val = Rc::new(Object::Integer(r));

    match op {
        Token::Plus => Ok(Rc::new(Object::Integer(l + r))),
        Token::Minus => Ok(Rc::new(Object::Integer(l - r))),
        Token::Asterisk => Ok(Rc::new(Object::Integer(l * r))),
        Token::Slash => Ok(Rc::new(Object::Integer(l / r))),
        Token::Lt => Ok(Rc::new(Object::Boolean(l < r))),
        Token::Gt => Ok(Rc::new(Object::Boolean(l > r))),
        Token::Eq => Ok(Rc::new(Object::Boolean(l == r))),
        Token::NotEq => Ok(Rc::new(Object::Boolean(l != r))),
        _ => Err(EvaluatorError::new(format!(
            "unknown operator: {} {} {}",
            left_val, op, right_val
        ))),
    }
}

fn eval_prefix_expression(op: &Token, expr: &Rc<Object>) -> EvaluatorResult {
    match op {
        Token::Bang => eval_bang_operator_expression(expr),
        Token::Minus => eval_minus_prefix_operator_expression(expr),
        _ => Err(EvaluatorError::new(format!(
            "unknown operator: {}{}",
            op, expr
        ))),
    }
}

fn eval_minus_prefix_operator_expression(expr: &Rc<Object>) -> EvaluatorResult {
    match **expr {
        Object::Integer(i) => Ok(Rc::new(Object::Integer(-i))),
        _ => Err(EvaluatorError::new(format!("unknown operator: -{}", expr))),
    }
}

fn eval_bang_operator_expression(expr: &Rc<Object>) -> EvaluatorResult {
    match **expr {
        Object::Boolean(b) => Ok(Rc::new(Object::Boolean(!b))),
        Object::Null => Ok(Rc::new(Object::Boolean(true))),
        _ => Ok(Rc::new(Object::Boolean(false))),
    }
}

fn eval_literal(lit: &Literal, env: &Env) -> EvaluatorResult {
    match lit {
        Literal::Int(i) => Ok(Rc::new(Object::Integer(*i))),
        Literal::String(_) => todo!(),
        Literal::Bool(bool) => Ok(Rc::new(Object::Boolean(*bool))),
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

pub fn eval_statement(statement: &Statement, env: &Env) -> EvaluatorResult {
    match statement {
        Statement::Let(_, _) => todo!(),
        Statement::Return(expr) => {
            let val = eval_expression(expr, env)?;
            Ok(Rc::new(Object::ReturnValue(val)))
        }
        Statement::Expr(expr) => eval_expression(&expr, env),
    }
}

pub fn eval_program(program: Vec<Statement>, env: &Env) -> EvaluatorResult {
    let mut result = Rc::new(Object::Null);

    for statement in program {
        result = eval_statement(&statement, &Rc::clone(env))?;
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
                    Ok(evaluated) => assert_eq!(expected, &format!("{}", evaluated)),
                    Err(err) => assert_eq!(expected, &format!("{}", err)),
                },
                Err(err) => panic!("Parser Error: {:?}", err),
            }
        }
    }

    #[test]
    fn test_integer_expression() {
        let test_case = vec![
            ("5", "5"),
            ("10", "10"),
            ("-5", "-5"),
            ("-10", "-10"),
            ("5+5", "10"),
            ("10+20", "30"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_boolean_expression() {
        let test_case = vec![
            ("true", "true"),
            ("false", "false"),
            ("1 < 2", "true"),
            ("1<1", "false"),
        ];

        apply_test(&test_case)
    }

    #[test]
    fn test_if_else_expressions() {
        let test_case = vec![
            ("if (true) { 10 }", "10"),
            ("if (false) { 10 }", "null"),
            ("if (1) { 10 }", "10"),
            ("if (1 < 2) { 10 }", "10"),
            ("if (1 > 2) { 10 }", "null"),
            ("if (1 > 2) { 10 } else { 20 }", "20"),
            ("if (1 < 2) { 10 } else { 20 }", "10"),
        ];

        apply_test(&test_case)
    }

    #[test]
    fn test_bang_operator() {
        let test_case = vec![
            ("!true", "false"),
            ("!false", "true"),
            ("!5", "false"),
            ("!!true", "true"),
            ("!!false", "false"),
            ("!!5", "true"),
        ];

        apply_test(&test_case)
    }

    #[test]
    fn test_return_statements() {
        let test_case = vec![
            ("return 10;", "10"),
            ("return 10; 9;", "10"),
            ("return 2 * 5; 9;", "10"),
            ("9; return 2 * 5; 9;", "10"),
        ];

        apply_test(&test_case)
    }
}
