pub mod error;

pub mod parser {

    use super::error::ParserError;
    use super::error::ParserErrors;
    use crate::ast::ast::Expression;
    use crate::ast::ast::Node;
    use crate::ast::ast::Statement;
    use crate::lexer::lexer::Lexer;
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
                _ => Err(ParserError::new("Unknown statement".to_string())),
            }
        }

        fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
            let ident = match &self.peek_token {
                Token::Ident(ref s) => s.clone(),
                t => {
                    return Err(self.error_no_identifier());
                }
            };
            // Consuming the IDENT token
            self.next_token();
            self.expect_peek(&Token::Assign)?;
            self.next_token();

            let expr = Expression::Identifier("".to_string());

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

        fn parse_ident(&self) -> Token {
            todo!()
        }

        fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
            self.next_token();

            let expr = Expression::Identifier("".to_string());

            while !self.cur_token_is(&Token::Semicolon) {
                self.next_token();
            }

            Ok(Statement::Return(expr))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::ast::Statement, lexer};

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
    fn test_let_statement_refactored() {
        let test_case = [
            ("let x = 5;", "let x = 5;"),
            ("let y = true;", "let y = true;"),
            ("let foobar = y;", "let foobar = y;"),
        ];
        apply_test(&test_case);
    }
}
