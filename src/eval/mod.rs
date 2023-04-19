pub mod eval {
    use crate::ast::ast::{Expression, Node, Statement};

    pub fn eval(node: Node) {
        match node {
            Node::Expr(expr) => eval_expression(expr),
            Node::Statement(statement) => eval_statement(statement),
            Node::Program(program) => eval_program(program),
        }
    }

    pub fn eval_expression(expr: Expression) {
        match expr {
            Expression::Lit(l) => println!("Literal: {}", l),
            Expression::Identifier(_) => todo!(),
            Expression::Prefix(_, _) => todo!(),
            Expression::Infix(_, _, _) => todo!(),
            Expression::Postfix(_, _) => todo!(),
            Expression::If(_, _, _) => todo!(),
            Expression::Fn(_, _) => todo!(),
            Expression::Call(_, _) => todo!(),
        }
    }

    pub fn eval_statement(statement: Statement) {
        match statement {
            Statement::Let(_, _) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::Expr(expr) => eval_expression(expr),
        }
    }

    pub fn eval_program(program: Vec<Statement>) {
        for statement in program {
            eval_statement(statement);
        }
    }
}
