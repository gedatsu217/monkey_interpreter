use crate::{evaluator, lexer, object, repl, token};
use std::io;

const PROMPT: &str = ">> ";

pub fn Start() {
    println!(">> ");
    let mut env = object::NewEnvironment();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failing in input");
        let l = lexer::New(input);
        let mut p = l.New();
        let program = p.ParseProgram();
        if p.Errors().len() != 0 {
            printParserErrors(p.Errors());
            continue;
        }
        let evaluated = evaluator::Eval(program, &mut env);
        if let object::Object::Null = evaluated {
            println!("semantics error");
        } else {
            println!("{}", evaluated.Inspect());
        }
    }
}

fn printParserErrors(errors: &Vec<String>) {
    println!("Error!");
    println!(" parser errors: ");
    for msg in errors.iter() {
        println!("{}", msg);
    }
}
