pub mod repl {

    use std::rc::Rc;
    

    use crate::eval::eval;
    use crate::parser::error::ParserError;
    use crate::parser::parser::parse;
    use rustyline::error::ReadlineError;
    use rustyline::DefaultEditor;

    pub fn start() -> rustyline::Result<()> {
        let mut rl = DefaultEditor::new()?;
        let env = Rc::new(Default::default());
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => match parse(&line) {
                    Ok(node) => match eval(node, &Rc::clone(&env)) {
                        Ok(evaluated) => println!("{}", evaluated),
                        Err(e) => println!("Error: {}", e),
                    },
                    Err(e) => {
                        print_parse_errors(e);
                    }
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        Ok(())
    }

    fn print_parse_errors(errors: Vec<ParserError>) {
        println!(
            "Woops! We ran into some issues parsing your input, please fix the following errors:"
        );
        for err in errors {
            println!("{}", err);
        }
    }
}
