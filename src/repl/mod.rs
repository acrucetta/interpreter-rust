pub mod repl {

    use crate::lexer::lexer::Lexer;
    use crate::token::token::TokenKind;
    use crate::token::token::{self, Token};
    use std::io::{BufRead, Write};

    pub const PROMPT: &str = ">> ";

    pub fn start(reader: &mut dyn BufRead, writer: &mut dyn Write) {
        loop {
            let mut line = String::new();
            print!("{}", PROMPT);
            writer.flush().unwrap();

            if reader.read_line(&mut line).unwrap() == 0 {
                break;
            }
            let mut l = Lexer::new(line);
            loop {
                let tok = l.next_token();
                println!("{:?}", tok);
                if tok.kind == TokenKind::Eof {
                    break;
                }
            }
        }
    }
}
