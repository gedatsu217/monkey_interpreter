use crate::token;
/*
pub trait Node {
    fn TokenLiteral(&self) -> String;
}

pub trait Statement: Node {
    fn statementNode(&self);
    fn whether_let(&self) -> bool;
}

pub trait Expression: Node {
    fn expressionNode(&self);
}

pub struct Program {
    pub Statements: Vec<Box<Statement>>,
}

impl Node for Program {
    fn TokenLiteral(&self) -> String {
        if self.Statements.len() > 0 {
            self.Statements[0].TokenLiteral()
        } else {
            String::from("")
        }
    }
}

pub struct LetStatement {
    Token: token::Token,
    Name: Identifier,
    Value: Expression,
}

impl Node for LetStatement {
    fn TokenLiteral(&self) -> String {self.Token.Literal.clone()}
}

impl Statement for LetStatement {
    fn statementNode(&self) {}
    fn whether_let(&self) -> bool{
        true
    }
}

pub struct Identifier {
    Token: token::Token,
    Value: String,
}

impl Node for Identifier {
    fn TokenLiteral(&self) -> String {self.Token.Literal.clone()}
}

impl Expression for Identifier {
    fn expressionNode(&self) {}
}
*/

pub enum Statement {
    LetStatement{Token: token::Token, Name: Identifier, Value: Expression},
    dummy(String),
}

pub struct Identifier {
    pub Token: token::Token,
    pub Value: String,
}

pub enum Expression {
    Nil,
    
}

pub struct Program {
    pub Statements: Vec<Statement>,
}

