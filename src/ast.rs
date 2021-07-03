use crate::token;
use std::fmt;

pub enum Statement {
    LetStatement{Token: token::Token, Name: Identifier, Value: Expression},
    ReturnStatement{Token: token::Token, ReturnValue: Expression},
    ExpressionStatement{Token: token::Token, Expression: Expression},
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
    Identifier(Identifier),

}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "Expression"}
    }
}

pub struct Program {
    pub Statements: Vec<Statement>,
}

