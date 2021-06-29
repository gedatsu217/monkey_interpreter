use std::collections::HashMap;
pub type TokenType = &'static str;



pub const ILLEGAL: TokenType   = "ILLEGAL";
pub const EOF: TokenType       = "EOF";

pub const IDENT: TokenType     = "IDENT";
pub const INT: TokenType       = "INT";

pub const ASSIGN: TokenType    = "=";
pub const PLUS: TokenType      = "+";
pub const MINUS: TokenType     = "-";
pub const BANG: TokenType      = "!";
pub const ASTERISK: TokenType  = "*";
pub const SLASH: TokenType     = "/";
pub const COMMA: TokenType     = ",";
pub const SEMICOLON: TokenType = ";";

pub const LPAREN: TokenType	   = "(";
pub const RPAREN: TokenType	   = ")";
pub const LBRACE: TokenType	   = "{";
pub const RBRACE: TokenType    = "}";

pub const LT: TokenType        = "<";
pub const GT: TokenType        = ">";

pub const EQ: TokenType        = "==";
pub const NOT_EQ: TokenType    = "!=";

pub const FUNCTION: TokenType  = "FUNCTION";
pub const LET: TokenType 	   = "LET";
pub const TRUE: TokenType 	   = "TRUE";
pub const FALSE: TokenType 	   = "FALSE";
pub const IF: TokenType 	   = "IF";
pub const ELSE: TokenType 	   = "ELSE";
pub const RETURN : TokenType   = "RETURN";




pub struct Token {
    pub Type: TokenType,
    pub Literal: String,
}

pub fn LookupIdent(ident: &String) -> TokenType {
    let mut keywords = HashMap::new();
    keywords.insert("fn", FUNCTION);
    keywords.insert("let", LET);
    keywords.insert("true", TRUE);
    keywords.insert("false", FALSE);
    keywords.insert("if", IF);
    keywords.insert("else", ELSE);
    keywords.insert("return", RETURN);

    let tok: TokenType;

    if keywords.contains_key(ident.as_str()) {
        let tok = keywords.get(ident.as_str()).unwrap();
        return tok
    } 

    IDENT
}
