pub mod parser {
    use crate::ast::ast;
    use crate::ast::ast::LetStatement;
    use crate::ast::ast::Statement;
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token;
    use crate::token::token::TokenKind;

    pub struct Parser {
        l: Lexer,
        cur_token: Token,
        peek_token: Token,
    }

    impl Parser {
        pub fn new(l: Lexer) -> Parser {
            let mut p = Parser {
                l,
                cur_token: Token::new(TokenKind::Eof, "".to_string()),
                peek_token: Token::new(TokenKind::Eof, "".to_string()),
            };
            p.next_token();
            p.next_token();
            p
        }

        fn next_token(&mut self) {
            self.cur_token = self.peek_token.clone();
            self.peek_token = self.l.next_token();
        }

        pub fn parse_program(&self) -> ast::Program {
            let mut program = ast::Program::new();

            while self.cur_token.kind != TokenKind::Eof {
                let stmt = self.parse_statement();
                if let Some(stmt) = stmt {
                    program.statements.push(stmt);
                }
                self.next_token();
            }
            program
        }

        fn parse_statement(&self) -> Option<Statement> {
            match self.cur_token.kind {
                TokenKind::Let => self.parse_let_statement(),
                _ => None,
            }
        }

        fn parse_let_statement(&self) -> Option<LetStatement> {
            let mut stmt = LetStatement::new();

            if !self.expect_peek(TokenKind::Ident) {
                return None;
            }

            stmt.name = ast::Identifier::new(self.cur_token.literal.clone());

            if !self.expect_peek(TokenKind::Assign) {
                return None;
            }

            while !self.cur_token_is(TokenKind::Semicolon) {
                self.next_token();
            }

            Some(stmt)
        }

        fn cur_token_is(&self, t: TokenKind) -> bool {
            self.cur_token.kind == t
        }

        fn peek_token_is(&self, t: TokenKind) -> bool {
            self.peek_token.kind == t
        }

        fn expect_peek(&self, t: TokenKind) -> bool {
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
        let p = parser::Parser::new(l);

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
            Statement::LetStatement(let_statement) => let_statement,
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
