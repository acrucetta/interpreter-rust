pub mod repl {

    use crate::parser::parser::parse;
    use rustyline::error::ReadlineError;
    use rustyline::DefaultEditor;

    pub fn start() -> rustyline::Result<()> {
        let mut rl = DefaultEditor::new()?;
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => match parse(&line) {
                    Ok(program) => {
                        println!("{:#?}", program);
                    }
                    Err(e) => {
                        for err in e {
                            println!("{}", err);
                        }
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
}
