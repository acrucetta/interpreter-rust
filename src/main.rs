use interpreter_rust::repl::repl;

fn main() {
    print!("Welcome to the Monkey programming language!");
    print!("Feel free to type in commands");
    repl::start();
}
