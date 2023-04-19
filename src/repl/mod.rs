pub mod repl {

    use crate::parser::parser::Parser;
    use crate::token::token::Token;
    use crate::{lexer::lexer::Lexer, parser::parser::parse};
    use rustyline::error::ReadlineError;
    use rustyline::Editor;
    use std::io::{BufRead, Write};

    pub fn start(reader: &mut dyn BufRead, writer: &mut dyn Write) {
        let mut rl = Editor::<()>::new();
        loop {
            match rl.readline(">> ") {
                Ok(line) => {
                    let mut lexer = Lexer::new(line);
                    let mut parser = Parser::new(&mut lexer);
                    let program = parser.parse_program();
                    if parser.errors.len() > 0 {
                        for err in parser.errors {
                            println!("parser error: {}", err);
                        }
                    } else {
                        println!("{}", program);
                    }
                }
                Err(_) => {
                    println!("Goodbye!");
                    break;
                }
            }
        }
    }
}
