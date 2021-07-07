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

impl Statement {
    pub fn into_string(&self) -> String {
        match self {
            Statement::LetStatement{..} => {"".to_string()},
            Statement::ReturnStatement{..} => {"".to_string()},
            Statement::ExpressionStatement{Token, Expression} => {Expression.into_string()}
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
    IntergerLiteral{Token: token::Token, Value: i64},
    PrefixExpression{Token: token::Token, Operator: String, Right: Box<Expression>},
    InfixExpression{Token: token::Token, Left: Box<Expression>, Operator: String, Right: Box<Expression>}

}

impl Expression {
    pub fn into_string(&self) -> String{
        match self {
            Expression::Nil => {String::from("")},
            Expression::Identifier(Identifier) => {Identifier.Value.clone()},
            Expression::IntergerLiteral{Token ,Value} => {Token.Literal.clone()},
            Expression::PrefixExpression{Token, Operator, Right} => {String::from("(") + Operator + Right.into_string().as_str() + ")"},
            Expression::InfixExpression{Token, Left, Operator, Right} => {String::from("(") + Left.into_string().as_str() + " " + Operator + " " + Right.into_string().as_str() + ")"},
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "Expression"}
    }
}

pub struct Program {
    pub Statements: Vec<Statement>,
}

impl Program {
    pub fn into_string(&self) -> String {
        let mut res = "".to_string();
        for s in self.Statements.iter() {
            res += &s.into_string();
        }
        res
    } 
}

