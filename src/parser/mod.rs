pub mod parser {
    use crate::ast::ast;
    use crate::lexer::lexer::Lexer;
    use crate::token;
    use crate::token::token::{Token, TokenType};

    struct Parser {
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

        fn parse_program(&self) -> ast::Program {
            return None;
        }
    }
}
