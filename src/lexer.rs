use crate::token;
use token::Token;

pub struct Lexer {
    input: String,
    position: i32,
    readPosition: i32,
    ch: Option<String>,
}

pub fn New(input: String) -> Lexer {
    let mut l = Lexer{input, position: 0, readPosition: 0, ch: Some(String::from(""))};
    l.readChar();
    l
}

impl Lexer {
    fn readChar(&mut self) {
        if self.readPosition >= self.input.len() as i32 {
            self.ch = None;
        } else {
            //self.ch = String::from(&self.input[self.readPosition..2]);
            self.ch = Some(self.input.chars().nth(self.readPosition as usize).unwrap().to_string());
        }
        self.position = self.readPosition;
        self.readPosition += 1;
    }


    pub fn NextToken(&mut self) -> Token{
        let tok: Token;
        let equal_str = &String::from("=");
        let semicolon_str = &String::from(";");
        let lparen_str = &String::from("(");
        let rparen_str = &String::from(")");
        let comma_str = &String::from(",");
        let plus_str = &String::from("+");
        let lbrace_str = &String::from("{");
        let rbrace_str = &String::from("}");

        
        
        match &self.ch {
            Some(s) if s == equal_str => {tok = newToken(token::ASSIGN, &self.ch);},
            Some(s) if s == semicolon_str => {tok = newToken(token::SEMICOLON, &self.ch);},
            Some(s) if s == lparen_str => {tok = newToken(token::LPAREN, &self.ch);},
            Some(s) if s == rparen_str => {tok = newToken(token::RPAREN, &self.ch);},
            Some(s) if s == comma_str => {tok = newToken(token::COMMA, &self.ch);},
            Some(s) if s == plus_str => {tok = newToken(token::PLUS, &self.ch);},
            Some(s) if s == lbrace_str => {tok = newToken(token::LBRACE, &self.ch);},
            Some(s) if s == rbrace_str => {tok = newToken(token::RBRACE, &self.ch);},

            _ => {tok = newToken(token::EOF, &Some(String::from("")))},
        }

        self.readChar();

        tok
    }
}

fn newToken(tokenType: token::TokenType, ch: &Option<String>) -> Token{
    Token{Type: tokenType, Literal: ch.clone().unwrap()}
}


