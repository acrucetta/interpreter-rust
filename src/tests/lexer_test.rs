mod lexer;

use lexer::Lexer;
use token::Token;

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    pub fn test_next_token() {
        let input = "=+(){},;";

        let tests: Vec<(TokenKind, string)> = vec![
            (TokenKind::ASSIGN, "="),
            (TokenKind::PLUS, "+"),
            (TokenKind::LPAREN, "("),
            (TokenKind::RPAREN, ")"),
            (TokenKind::LBRACE, "{"),
            (TokenKind::RBRACE, "}"),
            (TokenKind::COMMA, ","),
            (TokenKind::SEMICOLON, ";"),
            (TokenKind::EOF, ""),
        ];

        let mut l = Lexer::new(input);

        for (expected_kind, expected_literal) in tests {
            let tok = l.next_token();

            assert_eq!(tok.kind, expected_kind);
            assert_eq!(tok.literal, expected_literal);
        }
    }

    pub fn test_test() {
        assert_eq!(1, 1);
    }
}
