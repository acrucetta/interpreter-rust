mod lexer;
mod repl;
mod token;

fn main() {
    print!("Welcome to the Monkey programming language!");
    print!("Feel free to type in commands");
    repl::repl::start(&mut std::io::stdin().lock(), &mut std::io::stdout());
}
