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
        let assign_str = &String::from("=");
        let semicolon_str = &String::from(";");
        let lparen_str = &String::from("(");
        let rparen_str = &String::from(")");
        let comma_str = &String::from(",");
        let plus_str = &String::from("+");
        let lbrace_str = &String::from("{");
        let rbrace_str = &String::from("}");
        let minus_str = &String::from("-");
        let bang_str = &String::from("!");
        let slash_str = &String::from("/");
        let asterisk_str = &String::from("*");
        let lt_str = &String::from("<");
        let gt_str = &String::from(">");

        self.skipWhitespace();

        

        match &self.ch {
            Some(s) if s == assign_str => {
                if self.peekChar() == Some(String::from("=")) {
                    self.readChar();
                    tok = newToken(token::EQ, &Some(String::from("==")));
                } else {
                    tok = newToken(token::ASSIGN, &self.ch);
                }
            },
            Some(s) if s == semicolon_str => {tok = newToken(token::SEMICOLON, &self.ch);},
            Some(s) if s == lparen_str => {tok = newToken(token::LPAREN, &self.ch);},
            Some(s) if s == rparen_str => {tok = newToken(token::RPAREN, &self.ch);},
            Some(s) if s == comma_str => {tok = newToken(token::COMMA, &self.ch);},
            Some(s) if s == plus_str => {tok = newToken(token::PLUS, &self.ch);},
            Some(s) if s == lbrace_str => {tok = newToken(token::LBRACE, &self.ch);},
            Some(s) if s == rbrace_str => {tok = newToken(token::RBRACE, &self.ch);},
            Some(s) if s == minus_str => {tok = newToken(token::MINUS, &self.ch);},
            Some(s) if s == bang_str => {
                if self.peekChar() == Some(String::from("=")) {
                    self.readChar();
                    tok = newToken(token::NOT_EQ, &Some(String::from("!=")));
                } else {
                    tok = newToken(token::BANG, &self.ch);
                }
            },
            Some(s) if s == slash_str => {tok = newToken(token::SLASH, &self.ch);},
            Some(s) if s == asterisk_str => {tok = newToken(token::ASTERISK, &self.ch);},
            Some(s) if s == lt_str => {tok = newToken(token::LT, &self.ch);},
            Some(s) if s == gt_str => {tok = newToken(token::GT, &self.ch);},
            None => {tok = newToken(token::EOF, &Some(String::from("")))},
            _ => {
                if isLetter(self.ch.as_ref().unwrap()) {
                    let literal = self.readIdentifier();
                    tok = newToken(token::LookupIdent(&literal), &Some(literal));
                    return tok
                } else if isDigit(self.ch.as_ref().unwrap()){
                    tok = newToken(token::INT, &Some(self.readNumber()));
                    return tok
                } else {
                    tok = newToken(token::ILLEGAL, &self.ch)
                }
            },
        }

        self.readChar();

        tok
    }

    fn readIdentifier(&mut self) -> String {
        let mut res = String::from("");
        while isLetter(self.ch.as_ref().unwrap()) {
            res += self.ch.as_ref().unwrap();
            self.readChar();
        }
        res
    }

    fn readNumber(&mut self) -> String {
        let mut res = String::from("");
        while isDigit(self.ch.as_ref().unwrap()) {
            res += self.ch.as_ref().unwrap();
            self.readChar();
        }
        res
    }

    fn skipWhitespace(&mut self) {
        while self.ch.as_ref() != None && (*self.ch.as_ref().unwrap() == String::from(" ") || *self.ch.as_ref().unwrap() == String::from("\t") || *self.ch.as_ref().unwrap() == String::from("\n") || *self.ch.as_ref().unwrap() == String::from("\r")) {
            self.readChar();
        }
    }

    fn peekChar(&mut self) -> Option<String> {
        if self.readPosition >= self.input.len() as i32 {
            None
        } else {
            Some(self.input.chars().nth(self.readPosition as usize).unwrap().to_string())
        }
    }
}

pub fn newToken(tokenType: token::TokenType, ch: &Option<String>) -> Token{
    Token{Type: tokenType, Literal: ch.clone().unwrap()}
}

fn isLetter(s: &String) -> bool {
    let ch = s.clone().into_bytes()[0];
    // alphabet(a-z, A-Z) or _
    return (97 <= ch && ch <= 122) || (65 <= ch && ch <= 90) || ch == 95
}

fn isDigit(s: &String) -> bool {
    let ch = s.clone().into_bytes()[0];
    // 0-9
    return 48 <= ch && ch <= 57
}


