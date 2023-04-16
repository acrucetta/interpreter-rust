pub mod error;
pub mod precedence;

pub mod parser {

    use super::error::ParserError;
    use super::error::ParserErrors;
    use super::precedence;
    use super::precedence::Precedence;
    use crate::ast::ast::BlockStatement;
    use crate::ast::ast::Expression;
    use crate::ast::ast::Literal;
    use crate::ast::ast::Node;
    use crate::ast::ast::Statement;
    use crate::lexer::lexer::Lexer;
    use crate::token;
    use crate::token::token::Token;

    pub fn parse(input: &str) -> Result<Node, ParserErrors> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;

        Ok(Node::Program(program))
    }

    pub struct Parser {
        pub l: Lexer,
        pub cur_token: Token,
        pub peek_token: Token,
        pub errors: Vec<ParserError>,
    }

    impl Parser {
        pub fn new(l: Lexer) -> Parser {
            let mut p = Parser {
                l,
                cur_token: Token::Eof,
                peek_token: Token::Eof,
                errors: Vec::new(),
            };
            p.next_token();
            p.next_token();
            p
        }

        pub fn errors(&self) -> Vec<ParserError> {
            self.errors.clone()
        }

        fn peek_error(&mut self, t: &Token) {
            let msg = format!(
                "expected next token to be {}, got {} instead",
                t, self.peek_token
            );
            self.errors.push(ParserError::new(msg));
        }

        fn next_token(&mut self) {
            self.cur_token = self.peek_token.clone();
            self.peek_token = self.l.next_token().unwrap();
        }

        fn error_no_identifier(&mut self) -> ParserError {
            let msg = format!(
                "expected next token to be IDENT, got {} instead",
                self.peek_token
            );
            ParserError::new(msg)
        }

        pub fn parse_program(&mut self) -> Result<Vec<Statement>, ParserErrors> {
            let mut program = vec![];

            while self.cur_token != Token::Eof {
                match self.parse_statement() {
                    Ok(stmt) => program.push(stmt),
                    Err(e) => self.errors.push(e),
                }
                self.next_token();
            }
            if !self.errors.is_empty() {
                Err(self.errors.clone())
            } else {
                Ok(program)
            }
        }

        fn parse_statement(&mut self) -> Result<Statement, ParserError> {
            match self.cur_token {
                Token::Let => self.parse_let_statement(),
                Token::Return => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
            }
        }

        fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
            let ident = match &self.peek_token {
                Token::Ident(ref s) => s.clone(),
                _t => {
                    return Err(self.error_no_identifier());
                }
            };
            // Consuming the IDENT token
            self.next_token();
            self.expect_peek(&Token::Assign)?;
            self.next_token();

            let expr = Expression::Identifier(self.cur_token.to_string());

            while !self.cur_token_is(&Token::Semicolon) {
                self.next_token();
            }

            Ok(Statement::Let(ident, expr))
        }

        fn cur_token_is(&self, t: &Token) -> bool {
            self.cur_token == *t
        }

        fn peek_token_is(&self, t: &Token) -> bool {
            self.peek_token == *t
        }

        fn expect_peek(&mut self, t: &Token) -> Result<(), ParserError> {
            if self.peek_token_is(t) {
                self.next_token();
                Ok(())
            } else {
                Err(ParserError::new(format!(
                    "expected next token to be {}, but got {} instead",
                    t, self.peek_token
                )))
            }
        }

        fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
            self.next_token();

            let expr = Expression::Identifier(self.cur_token.to_string());

            while !self.cur_token_is(&Token::Semicolon) {
                self.next_token();
            }

            Ok(Statement::Return(expr))
        }

        fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
            let expr = self.parse_expression(Precedence::Lowest)?;

            if self.peek_token_is(&Token::Semicolon) {
                self.next_token();
            }

            Ok(Statement::Expr(expr))
        }

        fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
            let mut left_expr = match self.cur_token {
                Token::Bang | Token::Minus => self.parse_prefix_expression(),
                Token::Ident(ref id) => Ok(Expression::Identifier(id.clone())),
                Token::Int(i) => Ok(Expression::Lit(Literal::Int(i))),
                Token::String(ref s) => Ok(Expression::Lit(Literal::String(s.clone()))),
                Token::Boolean(b) => Ok(Expression::Lit(Literal::Bool(b))),
                Token::LParen => {
                    self.next_token();
                    let expr = self.parse_expression(Precedence::Lowest)?;
                    self.expect_peek(&Token::RParen)?;
                    Ok(expr)
                }
                Token::If => self.parse_if_expression(),
                _ => Err(ParserError::new(format!(
                    "no prefix parse function for {:?} found",
                    self.cur_token
                ))),
            };

            while !self.peek_token_is(&Token::Semicolon)
                && precedence < self.next_token_precedence()
            {
                match self.peek_token {
                    Token::Plus
                    | Token::Minus
                    | Token::Slash
                    | Token::Asterisk
                    | Token::Eq
                    | Token::NotEq
                    | Token::Lt
                    | Token::Gt => {
                        self.next_token();
                        let expr = left_expr.unwrap();
                        left_expr = self.parse_infix_expression(expr);
                    }
                    // Token::LBracket => {
                    //     self.next_token();
                    //     let expr = left_expr.unwrap();
                    //     left_expr = self.parse_index_expression(expr);
                    // }
                    // Token::LParen => {
                    //     self.next_token();
                    //     let expr = left_expr.unwrap();
                    //     left_expr = self.parse_call_expression(expr);
                    // }
                    _ => {
                        return left_expr;
                    }
                }
            }
            left_expr
        }

        fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
            let prefix = self.cur_token.clone();
            self.next_token();
            let expr = self.parse_expression(Precedence::Prefix)?;
            Ok(Expression::Prefix(prefix, Box::new(expr)))
        }

        fn next_token_precedence(&self) -> Precedence {
            precedence::token_to_precedence(&self.peek_token)
        }

        fn parse_infix_expression(
            &mut self,
            left_expr: Expression,
        ) -> Result<Expression, ParserError> {
            let infix_op = self.cur_token.clone();
            self.next_token();
            let precedence = precedence::token_to_precedence(&infix_op);
            let right_expr = self.parse_expression(precedence)?;
            Ok(Expression::Infix(
                infix_op,
                Box::new(left_expr),
                Box::new(right_expr),
            ))
        }

        fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
            self.expect_peek(&Token::LParen)?;
            self.next_token();
            let condition = self.parse_expression(Precedence::Lowest)?;
            self.expect_peek(&Token::RParen)?;
            self.expect_peek(&Token::LBrace)?;
            let consequence = self.parse_block_statement()?;
            Ok(Expression::If(Box::new(condition), consequence, None))
        }

        fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
            self.next_token();
            let mut block_statement = Vec::new();

            while !self.cur_token_is(&Token::RBrace) && !self.cur_token_is(&Token::Eof) {
                if let Ok(stmt) = self.parse_statement() {
                    block_statement.push(stmt);
                }
                self.next_token();
            }
            Ok(block_statement)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parser::parse;

    fn apply_test(test_case: &[(&str, &str)]) {
        for (input, expected) in test_case {
            match parse(input) {
                Ok(node) => assert_eq!(expected, &format!("{}", node)),
                Err(e) => panic!("Parsing Error: {:#?}", e),
            }
        }
    }

    #[test]
    fn test_let_statement() {
        let test_case = [
            ("let x = 5;", "let x = 5;"),
            ("let y = true;", "let y = true;"),
            ("let foobar = y;", "let foobar = y;"),
        ];
        apply_test(&test_case);
    }

    #[test]
    fn test_return_statement() {
        let test_case = [
            ("return 5;", "return 5;"),
            ("return true;", "return true;"),
            ("return foobar;", "return foobar;"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_identifier_expression() {
        let test_case = [("foobar;", "foobar")];

        apply_test(&test_case);
    }

    #[test]
    fn test_integer_literal_expression() {
        let test_case = [("5;", "5")];

        apply_test(&test_case);
    }

    #[test]
    fn test_parsing_prefix_expression() {
        let test_case = [
            ("!5;", "(!5)"),
            ("-15;", "(-15)"),
            ("!true;", "(!true)"),
            ("!false;", "(!false)"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_parsing_infix_expression() {
        let test_case = [
            ("5 + 5;", "(5 + 5)"),
            ("5 - 5;", "(5 - 5)"),
            ("5 * 5;", "(5 * 5)"),
            ("5 / 5;", "(5 / 5)"),
            ("5 > 5;", "(5 > 5)"),
            ("5 < 5;", "(5 < 5)"),
            ("5 == 5;", "(5 == 5)"),
            ("5 != 5;", "(5 != 5)"),
            ("true == true", "(true == true)"),
            ("true != false", "(true != false)"),
            ("false == false", "(false == false)"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let test_case = [
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        ];
        apply_test(&test_case);
    }

    #[test]
    fn test_operator_precedence_parsing_bools() {
        let test_case = [
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("true == true", "(true == true)"),
            ("true != false", "(true != false)"),
            ("false == false", "(false == false)"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_operator_precedence_parsing_groups() {
        let test_case = [
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];

        apply_test(&test_case);
    }

    #[test]
    fn test_if_expression() {
        let test_case = [
            ("if (x < y) { x }", "if (x < y) { x }"),
            ("if (x < y) { x } else { y }", "if (x < y) { x } else { y }"),
        ];

        apply_test(&test_case);
    }
}
