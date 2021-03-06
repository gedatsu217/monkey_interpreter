use crate::{ast, ast::Program, lexer, lexer::Lexer, token, token::Token};
use phf::phf_map;
use std::collections::HashMap;

const LOWEST: i32 = 1;
const EQUALS: i32 = 2;
const LESSGRATER: i32 = 3;
const SUM: i32 = 4;
const PRODUCT: i32 = 5;
const PREFIX: i32 = 6;
const CALL: i32 = 7;

static precedences: phf::Map<&'static str, i32> = phf_map! {
    "==" => EQUALS,
    "!=" => EQUALS,
    "<" => LESSGRATER,
    ">" => LESSGRATER,
    "+" => SUM,
    "-" => SUM,
    "/" => PRODUCT,
    "*" => PRODUCT,
    "(" => CALL,
};

pub struct Parser {
    l: lexer::Lexer,
    curToken: token::Token,
    peekToken: token::Token,
    errors: Vec<String>,
}

impl Lexer {
    pub fn New(self) -> Parser {
        let mut p = Parser {
            l: self,
            curToken: lexer::newToken(token::ILLEGAL, &Some(String::from(""))),
            peekToken: lexer::newToken(token::ILLEGAL, &Some(String::from(""))),
            errors: vec![],
        };
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

    pub fn ParseProgram(&mut self) -> Program {
        let mut program = ast::Program { Statements: vec![] };
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
            token::LET => return self.parseLetStatement(),
            token::RETURN => return self.parseReturnStatement(),
            _ => return self.parseExpressionStatement(),
        };
    }

    fn parseLetStatement(&mut self) -> Option<ast::Statement> {
        let temp_token = self.curToken.clone();
        if !self.expectPeek(token::IDENT) {
            return None;
        }

        let temp_name = ast::Identifier {
            Token: self.curToken.clone(),
            Value: self.curToken.Literal.clone(),
        };

        if !self.expectPeek(token::ASSIGN) {
            return None;
        }

        self.nextToken();

        let stmt = ast::Statement::LetStatement {
            Token: temp_token,
            Name: temp_name,
            Value: self.parseExpression(LOWEST),
        };

        if self.peekTokenIs(token::SEMICOLON) {
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

    pub fn Errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peekError(&mut self, t: token::TokenType) {
        let msg = format!(
            "expected next token to be {}, got {}, instead",
            t, self.peekToken.Type
        );
        self.errors.push(msg);
    }

    pub fn checkParserErrors(&self) {
        let errors = self.Errors();
        if errors.len() == 0 {
            return;
        }
        println!("parser has {} errors", errors.len());
        for error in errors.iter() {
            println!("{}", error);
        }
        panic!();
    }

    fn parseReturnStatement(&mut self) -> Option<ast::Statement> {
        let token_temp = self.curToken.clone();
        self.nextToken();
        let stmt = ast::Statement::ReturnStatement {
            Token: token_temp,
            ReturnValue: self.parseExpression(LOWEST),
        };

        if self.peekTokenIs(token::SEMICOLON) {
            self.nextToken();
        }

        Some(stmt)
    }

    fn parseExpressionStatement(&mut self) -> Option<ast::Statement> {
        let res = self.parseExpression(LOWEST);
        if let ast::Expression::Nil = res {
            return None;
        }

        let stmt = ast::Statement::ExpressionStatement {
            Token: self.curToken.clone(),
            Expression: res,
        };

        if self.peekTokenIs(token::SEMICOLON) {
            self.nextToken();
        }

        Some(stmt)
    }

    fn parseExpression(&mut self, precedence: i32) -> ast::Expression {
        let mut left = match self.curToken.Type {
            token::IDENT => ast::Expression::Identifier(ast::Identifier {
                Token: self.curToken.clone(),
                Value: self.curToken.Literal.clone(),
            }),
            token::INT => self.parseIntergerLiteral(),
            token::BANG => self.parsePrefixExpression(),
            token::MINUS => self.parsePrefixExpression(),
            token::TRUE => self.parseBoolean(),
            token::FALSE => self.parseBoolean(),
            token::LPAREN => self.parseGroupedExpression(),
            token::IF => self.parseIfExpression(),
            token::FUNCTION => self.parseFunctionLiteral(),
            _ => {
                let msg = format!("no prefix parse function for {} found", self.curToken.Type);
                self.errors.push(msg);
                ast::Expression::Nil
            }
        };

        while !self.peekTokenIs(token::SEMICOLON) && precedence < self.peekPrecedence() {
            match self.peekToken.Type {
                token::PLUS => {
                    self.nextToken();
                    left = self.parseInfixExpression(left);
                }
                token::MINUS => {
                    self.nextToken();
                    left = self.parseInfixExpression(left);
                }
                token::SLASH => {
                    self.nextToken();
                    left = self.parseInfixExpression(left);
                }
                token::ASTERISK => {
                    self.nextToken();
                    left = self.parseInfixExpression(left);
                }
                token::EQ => {
                    self.nextToken();
                    left = self.parseInfixExpression(left);
                }
                token::NOT_EQ => {
                    self.nextToken();
                    left = self.parseInfixExpression(left);
                }
                token::LT => {
                    self.nextToken();
                    left = self.parseInfixExpression(left);
                }
                token::GT => {
                    self.nextToken();
                    left = self.parseInfixExpression(left);
                }
                token::LPAREN => {
                    self.nextToken();
                    left = self.parseCallExpression(left);
                }
                _ => {}
            }
        }

        left
    }

    fn parseIntergerLiteral(&mut self) -> ast::Expression {
        let value = self.curToken.Literal.parse();

        match value {
            Err(_) => {
                let msg = format!("could not parse {} as integer", self.curToken.Literal);
                self.errors.push(msg);
                ast::Expression::Nil
            }
            Ok(v) => ast::Expression::IntergerLiteral {
                Token: self.curToken.clone(),
                Value: v,
            },
        }
    }

    fn parsePrefixExpression(&mut self) -> ast::Expression {
        let token_temp = self.curToken.clone();
        let ope_temp = self.curToken.Literal.clone();
        self.nextToken();
        ast::Expression::PrefixExpression {
            Token: token_temp,
            Operator: ope_temp,
            Right: Box::new(self.parseExpression(PREFIX)),
        }
    }

    fn peekPrecedence(&self) -> i32 {
        let p = precedences.get(self.peekToken.Type);
        match p {
            Some(x) => *x,
            None => LOWEST,
        }
    }

    fn curPrecedence(&self) -> i32 {
        let p = precedences.get(self.curToken.Type);
        match p {
            Some(x) => *x,
            None => LOWEST,
        }
    }

    fn parseInfixExpression(&mut self, left: ast::Expression) -> ast::Expression {
        let token_temp = self.curToken.clone();
        let ope_temp = self.curToken.Literal.clone();
        let precedence = self.curPrecedence();
        self.nextToken();
        ast::Expression::InfixExpression {
            Token: token_temp,
            Operator: ope_temp,
            Left: Box::new(left),
            Right: Box::new(self.parseExpression(precedence)),
        }
    }

    fn parseBoolean(&self) -> ast::Expression {
        ast::Expression::Boolean {
            Token: self.curToken.clone(),
            Value: self.curTokenIs(token::TRUE),
        }
    }

    fn parseGroupedExpression(&mut self) -> ast::Expression {
        self.nextToken();
        let exp = self.parseExpression(LOWEST);

        if !self.expectPeek(token::RPAREN) {
            return ast::Expression::Nil;
        }

        exp
    }

    fn parseIfExpression(&mut self) -> ast::Expression {
        let temp_token = self.curToken.clone();

        if !self.expectPeek(token::LPAREN) {
            return ast::Expression::Nil;
        }

        self.nextToken();
        let temp_condition = self.parseExpression(LOWEST);

        if !self.expectPeek(token::RPAREN) {
            return ast::Expression::Nil;
        }

        if !self.expectPeek(token::LBRACE) {
            return ast::Expression::Nil;
        }

        let temp_consequence = self.parseBlockStatement();
        let mut temp_alternative = ast::Statement::Nil;

        if self.peekTokenIs(token::ELSE) {
            self.nextToken();
            if !self.expectPeek(token::LBRACE) {
                return ast::Expression::Nil;
            }
            temp_alternative = self.parseBlockStatement();
        }

        ast::Expression::IfExpression {
            Token: temp_token,
            Condition: Box::new(temp_condition),
            Consequence: Box::new(temp_consequence),
            Alternative: Box::new(temp_alternative),
        }
    }

    fn parseBlockStatement(&mut self) -> ast::Statement {
        let temp_token = self.curToken.clone();
        let mut temp_statements = vec![];

        self.nextToken();

        while !self.curTokenIs(token::RBRACE) && !self.curTokenIs(token::EOF) {
            let stmt = self.parseStatement();
            if let Some(x) = stmt {
                temp_statements.push(x);
            }
            self.nextToken();
        }

        ast::Statement::BlockStatement {
            Token: temp_token,
            Statements: temp_statements,
        }
    }

    fn parseFunctionLiteral(&mut self) -> ast::Expression {
        let temp_token = self.curToken.clone();
        if !self.expectPeek(token::LPAREN) {
            return ast::Expression::Nil;
        }
        let temp_parameters = match self.parseFunctionParameters() {
            Some(x) => x,
            None => {
                return ast::Expression::Nil;
            }
        };

        if !self.expectPeek(token::LBRACE) {
            return ast::Expression::Nil;
        }

        ast::Expression::FunctionLiteral {
            Token: temp_token,
            Parameters: temp_parameters,
            Body: Box::new(self.parseBlockStatement()),
        }
    }

    fn parseFunctionParameters(&mut self) -> Option<Vec<ast::Expression>> {
        let mut identifiers: Vec<ast::Expression> = vec![];

        if self.peekTokenIs(token::RPAREN) {
            self.nextToken();
            return Some(identifiers);
        }

        self.nextToken();

        let ident = ast::Expression::Identifier(ast::Identifier {
            Token: self.curToken.clone(),
            Value: self.curToken.Literal.clone(),
        });
        identifiers.push(ident);

        while self.peekTokenIs(token::COMMA) {
            self.nextToken();
            self.nextToken();
            let ident = ast::Expression::Identifier(ast::Identifier {
                Token: self.curToken.clone(),
                Value: self.curToken.Literal.clone(),
            });
            identifiers.push(ident);
        }

        if !self.expectPeek(token::RPAREN) {
            return None;
        }

        Some(identifiers)
    }

    fn parseCallExpression(&mut self, function: ast::Expression) -> ast::Expression {
        match self.parseCallArguments() {
            Some(x) => ast::Expression::CallExpression {
                Token: self.curToken.clone(),
                Function: Box::new(function),
                Arguments: x,
            },
            None => ast::Expression::Nil,
        }
    }

    fn parseCallArguments(&mut self) -> Option<Vec<ast::Expression>> {
        let mut args = vec![];
        if self.peekTokenIs(token::RPAREN) {
            self.nextToken();
            return Some(args);
        }
        self.nextToken();
        args.push(self.parseExpression(LOWEST));
        while self.peekTokenIs(token::COMMA) {
            self.nextToken();
            self.nextToken();
            args.push(self.parseExpression(LOWEST));
        }

        if !self.expectPeek(token::RPAREN) {
            return None;
        }
        Some(args)
    }
}
