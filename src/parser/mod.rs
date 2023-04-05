pub mod parser {
    use crate::ast::ast;
    use crate::ast::ast::Let;
    use crate::ast::ast::Statement;
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token;

    pub struct Parser {
        l: Lexer,
        cur_token: Token,
        peek_token: Token,
    }

    impl Parser {
        pub fn new(l: Lexer) -> Parser {
            let mut p = Parser {
                l,
                cur_token: Token::Eof,
                peek_token: Token::Eof,
            };
            p.next_token();
            p.next_token();
            p
        }

        fn next_token(&mut self) {
            self.cur_token = self.peek_token.clone();
            self.peek_token = self.l.next_token().unwrap();
        }

        pub fn parse_program(&mut self) -> ast::Program {
            let mut program = ast::Program::new();

            while self.cur_token != Token::Eof {
                let stmt = self.parse_statement();
                if let Some(stmt) = stmt {
                    program.statements.push(stmt);
                }
                self.next_token();
            }
            program
        }

        fn parse_statement(&mut self) -> Option<Statement> {
            match self.cur_token {
                Token::Let => self.parse_let_statement(),
                _ => None,
            }
        }

        fn parse_let_statement(&mut self) -> Option<Statement> {
            let mut stmt = Let::new();

            if !self.expect_peek(Token::Ident) {
                return None;
            }

            stmt.name = ast::Identifier::new(self.cur_token.clone());

            if !self.expect_peek(Token::Assign) {
                return None;
            }

            while !self.cur_token_is(Token::Semicolon) {
                self.next_token();
            }

            Some(Statement::Let(Let {
                token: Token::Let,
                name: ast::Identifier::new(name),
                value: ast::Expression::Identifier(ast::Identifier::new("".to_string())),
            }))
        }

        fn cur_token_is(&self, t: Token) -> bool {
            self.cur_token == t
        }

        fn peek_token_is(&self, t: Token) -> bool {
            self.peek_token == t
        }

        fn expect_peek(&mut self, t: Token) -> bool {
            if self.peek_token_is(t) {
                self.next_token();
                return true;
            } else {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::Statement;
    use crate::{ast::ast::Identifier, lexer};

    use super::*;

    #[test]
    fn test_let_statements() {
        let input: String = " \
        let x = 5; \
        let y = 10; \
        let foobar = 838383; \
        "
        .to_string();

        let l = lexer::lexer::Lexer::new(input);
        let mut p = parser::Parser::new(l);

        let program = p.parse_program();

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
            return false;
        } else if let_statement.name.token_literal() != name {
            return false;
        } else {
            return true;
        }
    }
}
