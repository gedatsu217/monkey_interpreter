use crate::{ast, lexer, token, lexer::Lexer, token::Token, ast::Program};
use std::collections::HashMap;

const LOWEST: i32       = 1;
const EQUALS: i32       = 2;
const LESSGRATER: i32   = 3;
const SUM: i32          = 4;
const PRODUCT: i32      = 5;
const PREFIX: i32       = 6;
const CALL: i32         = 7;

pub struct Parser {
    l: lexer::Lexer,
    curToken: token::Token,
    peekToken: token::Token,
    errors: Vec<String>,
}

impl Lexer {
    pub fn New(self) -> Parser {
        let mut p = Parser{l: self, curToken: lexer::newToken(token::ILLEGAL, &Some(String::from(""))), peekToken: lexer::newToken(token::ILLEGAL, &Some(String::from(""))), errors: vec![]};
        p.nextToken();
        p.nextToken();
        p
    }
}

impl Parser {
    fn nextToken(&mut self) {
        self.curToken = lexer::newToken(self.peekToken.Type, &Some(self.peekToken.Literal.clone()));
        self.peekToken = self.l.NextToken();
    }

    pub fn ParseProgram(&mut self) -> Program{
        let mut program = ast::Program{Statements: vec![]};
        while self.curToken.Type != token::EOF {
            let stmt = self.parseStatement();
            if let Some(x) = stmt {
                program.Statements.push(x);
            }
            self.nextToken();
        }
        program
    }

    fn parseStatement(&mut self) -> Option<ast::Statement> {
        match self.curToken.Type {
            token::LET => {return self.parseLetStatement()},
            token::RETURN => {return self.parseReturnStatement()},
            _ => {return self.parseExpressionStatement()},
        };
    }

    fn parseLetStatement(&mut self) -> Option<ast::Statement> {
        //let stmt = ast::Statement::LetStatement{Token: self.curToken.clone(), Name: ast::Identifier{Token: token::Token{Type: token::ILLEGAL, Literal: String::from("")}, Value: String::from("")}, Value: ast::Expression::Nil};

        if !self.expectPeek(token::IDENT) {
            return None;
        }

        let stmt = ast::Statement::LetStatement{Token: self.curToken.clone(), Name: ast::Identifier{Token: self.curToken.clone(), Value: self.curToken.Literal.clone()}, Value: ast::Expression::Nil};


        if !self.expectPeek(token::ASSIGN) {
            return None;
        }

        while !self.curTokenIs(token::SEMICOLON) {
            self.nextToken();
        }

        Some(stmt)
    }

    fn curTokenIs(&self, t: token::TokenType) -> bool {
        self.curToken.Type == t
    }

    fn peekTokenIs(&self, t: token::TokenType) -> bool {
        self.peekToken.Type == t
    }

    fn expectPeek(&mut self, t: token::TokenType) -> bool {
        if self.peekTokenIs(t) {
            self.nextToken();
            return true;
        } else {
            self.peekError(t);
            return false;
        }
    }

    fn Errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peekError(&mut self, t: token::TokenType) {
        let msg = format!("expected next token to be {}, got {}, instead", t, self.peekToken.Type);
        self.errors.push(msg);
    }

    pub fn checkParserErrors(&self) {
        let errors = self.Errors();
        if errors.len() == 0 {
            return
        }
        println!("parser has {} errors", errors.len());
        for error in errors.iter() {
            println!("{}", error);
        }
        panic!();
    }

    fn parseReturnStatement(&mut self) -> Option<ast::Statement> {
        let stmt = ast::Statement::ReturnStatement{Token: self.curToken.clone(), ReturnValue: ast::Expression::Nil};
        self.nextToken();
        while !self.curTokenIs(token::SEMICOLON) {
            self.nextToken();
        }
        Some(stmt)
    }

    fn parseExpressionStatement(&mut self) -> Option<ast::Statement> {
        let stmt = ast::Statement::ExpressionStatement{Token: self.curToken.clone(), Expression: self.parseExpression(LOWEST)};

        if self.peekTokenIs(token::SEMICOLON) {
            self.nextToken();
        }

        Some(stmt)
    }

    fn parseExpression(&self, precedence: i32) -> ast::Expression {
        let mut temp = match self.curToken.Type {
            token::IDENT => ast::Expression::Identifier(ast::Identifier{Token: self.curToken.clone(), Value: self.curToken.Literal.clone()}),
            _ => ast::Expression::Nil,
        };
        temp
    }


}

