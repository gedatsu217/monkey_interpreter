extern crate monkey_interpreter;
use monkey_interpreter::repl;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::Start();
}
