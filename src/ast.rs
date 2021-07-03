use crate::token;
use std::fmt;
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
    ReturnStatement{Token: token::Token, ReturnValue: Expression},
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            Statement::LetStatement{Token, Name, Value} => write!(f, "Statement::LetStatement{{Token: {}, Name: {}, Value: {}}}", Token, Name, Value),
            _ => write!(f, "unknown"),
        }
    }
}

pub struct Identifier {
    pub Token: token::Token,
    pub Value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "Identifier{{Token: {}, Value: {}}}", self.Token, self.Value}
    }
}

pub enum Expression {
    Nil,

}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "Expression"}
    }
}

pub struct Program {
    pub Statements: Vec<Statement>,
}

