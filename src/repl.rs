use crate::{token, lexer};
use std::io;

const PROMPT: &str = ">> ";

pub fn Start() {
    println!(">> ");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failing in input");
        let mut l = lexer::New(input);
        let mut tok = l.NextToken();
        while tok.Type != token::EOF {
            println!("{:?}", tok);
            tok = l.NextToken();
        }
    }
}