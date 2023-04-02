pub mod parser {
    use crate::ast::ast;
    use crate::lexer::lexer::Lexer;
    use crate::token;
    use crate::token::token::{Token, TokenType};

    pub struct Parser {
        l: Lexer,
        cur_token: Token,
        peek_token: Token,
    }

    impl Parser {
        pub fn new(l: Lexer) -> Parser {
            let mut p = Parser {
                l,
                cur_token: Token::new(token::token::EOF, "".to_string()),
                peek_token: Token::new(token::token::EOF, "".to_string()),
            };
            p.next_token();
            p.next_token();
            p
        }

        fn next_token(&self) {
            self.cur_token = self.peek_token.clone();
            self.peek_token = self.l.next_token();
        }

        pub fn parse_program(&self) -> ast::Program {
            return ast::Program::new();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::{LetStatement, Statement};
    use crate::{
        ast::{self, ast::Identifier},
        lexer,
    };

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
            let stmt = program.statements[i];
            let stmt = stmt.clone();

            assert!(test_let_statement(stmt, tt.value.clone()));
        }
    }

    fn test_let_statement(statement: Box<Statement>, name: String) -> bool {
        if statement.token_literal() != "let" {
            return false;
        }

        // Check if Box<dyn Statement> is a LetStatement
        if let Statement::LetStatement(let_stmt) = *statement {
            if let_stmt.name.value != name {
                return false;
            }

            if let_stmt.name.token_literal() != name {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }
}
