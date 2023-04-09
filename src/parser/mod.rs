pub mod error;

pub mod parser {
    use crate::ast::ast;
    use crate::ast::ast::Identifier;
    use crate::ast::ast::Let;
    use crate::ast::ast::Statement;
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token;

    use super::error::ParserError;

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

        fn error_no_identifier(&mut self) {
            let msg = format!(
                "expected next token to be IDENT, got {} instead",
                self.peek_token
            );
            self.errors.push(ParserError::new(msg));
        }

        pub fn parse_program(&mut self) -> ast::Program {
            let mut program = ast::Program::new();

            while self.cur_token != Token::Eof {
                let mut statement: Option<Statement> = None;
                match self.cur_token {
                    Token::Let => statement = self.parse_statement(),
                    Token::Return => statement = self.parse_statement(),
                    _ => (),
                }
                match statement {
                    Some(s) => program.statements.push(s),
                    None => (),
                }
                self.next_token();
            }
            program
        }

        fn parse_statement(&mut self) -> Option<Statement> {
            match self.cur_token {
                Token::Let => self.parse_let_statement(),
                Token::Return => self.parse_return_statement(),
                _ => None,
            }
        }

        fn parse_let_statement(&mut self) -> Option<Statement> {
            let ident = match self.peek_token {
                Token::Ident(ref s) => s.clone(),
                _ => {
                    self.error_no_identifier();
                    return None;
                }
            };
            // Consuming the IDENT token
            self.next_token();
            match self.expect_peek(&Token::Assign) {
                Ok(_) => (),
                Err(e) => {
                    self.errors.push(e);
                    return None;
                }
            }
            self.next_token();

            while !self.cur_token_is(&Token::Semicolon) {
                self.next_token();
            }

            let mut let_statement = Let::new();
            let_statement.name = Identifier::new(ident);
            Some(Statement::Let(let_statement))
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

        fn parse_return_statement(&mut self) -> Option<Statement> {
            let return_statement = ast::Return::new();
            self.next_token();
            while !self.cur_token_is(&Token::Semicolon) {
                self.next_token();
            }
            Some(Statement::Return(return_statement))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::Statement;
    use crate::{ast::ast::Identifier, lexer};

    use super::*;

    #[test]
    fn test_let_statements_small() {
        let input: String = "let x=5;".to_string();

        let l = lexer::lexer::Lexer::new(&input);
        let mut p = parser::Parser::new(l);

        let program = p.parse_program();

        assert_eq!(program.statements.len(), 1);

        // Decleare the expected identifiers
        let tests: Vec<Identifier> = {
            let mut v = Vec::new();
            v.push(Identifier::new("x".to_string()));
            v
        };

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            let stmt = Box::new(stmt);
            assert!(test_let_statement(stmt, tt.value.clone()));
        }
    }

    #[test]
    fn test_let_statements() {
        let input: String = " \
        let x = 5; \
        let y = 10; \
        let 838383; \
        "
        .to_string();

        let l = lexer::lexer::Lexer::new(&input);
        let mut p = parser::Parser::new(l);

        let program = p.parse_program();
        check_parse_errors(&p);

        assert_eq!(program.statements.len(), 3);

        // Decleare the expected identifiers
        let tests: Vec<Identifier> = {
            let mut v = Vec::new();
            v.push(Identifier::new("x".to_string()));
            v.push(Identifier::new("y".to_string()));
            v.push(Identifier::new("foobar".to_string()));
            v
        };

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            let stmt = Box::new(stmt);
            assert!(test_let_statement(stmt, tt.value.clone()));
        }
    }

    #[test]
    fn test_return_statement() {
        let input: String = "return 5;".to_string();

        let l = lexer::lexer::Lexer::new(&input);
        let mut p = parser::Parser::new(l);
        let program = p.parse_program();
        check_parse_errors(&p);

        assert_eq!(program.statements.len(), 1);

        for stmt in program.statements {
            match stmt {
                Statement::Return(_) => (),
                _ => panic!("statement not a return statement"),
            }
        }
    }

    fn check_parse_errors(p: &parser::Parser) {
        let errors = p.errors();
        if errors.is_empty() {
            return;
        }

        println!("parser has {} errors", errors.len());
        for msg in errors {
            println!("parser error: {}", msg);
        }
        panic!();
    }

    fn test_let_statement(statement: Box<&Statement>, name: String) -> bool {
        if statement.token_literal() != "let" {
            return false;
        }

        // Check if Box<dyn Statement> is a LetStatement
        let let_statement = match statement.as_ref() {
            Statement::Let(let_statement) => let_statement,
            _ => return false,
        };

        if let_statement.name.value != name {
            false
        } else {
            let_statement.name.token_literal() == name
        }
    }
}
