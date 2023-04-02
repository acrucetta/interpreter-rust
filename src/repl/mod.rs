mod lexer;
mod token;

pub mod repl {

    use crate::lexer::Lexer;
    use crate::token::token::Token;

    pub const PROMPT: &str = ">> ";

    pub fn start() {
        let mut input = String::new();
        loop {
            print!("{}", PROMPT);
            std::io::stdin().read_line(&mut input).unwrap();
            let mut lexer = Lexer::new(input.clone());
            let mut token = lexer.next_token();
            while token.kind != token::EOF {
                println!("{:?}", token);
                token = lexer.next_token();
            }
            input.clear();
        }
    }
}
