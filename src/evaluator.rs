use crate::{ast, object};

const TRUE: object::Object = object::Object::Boolean{Value: true};
const FALSE: object::Object = object::Object::Boolean{Value: false};
const NULL: object::Object = object::Object::Null;

pub fn Eval(node: ast::Program) -> Option<object::Object> {
    let mut result = None;
    for statement in node.Statements.iter() {
        result = match statement {
            ast::Statement::ExpressionStatement{Token, Expression} => evalExpression(Expression),
            _ => None,
        }
    }
    result
}

fn evalExpression(exp: &ast::Expression) -> Option<object::Object> {
    match exp {
        ast::Expression::IntergerLiteral{Token, Value} => Some(object::Object::Integer{Value: *Value}),
        ast::Expression::Boolean{Token, Value} => if *Value {Some(TRUE)} else {Some(FALSE)},
        ast::Expression::PrefixExpression{Token, Operator, Right} => {
            let right = evalExpression(Right);
            match right {
                Some(x) => evalPrefixExpression(Operator, x),
                None => None,
            }
        },
        ast::Expression::InfixExpression{Token, Left, Operator, Right} => {
            let left = evalExpression(Left);
            let right = evalExpression(Right);
            match left {
                Some(x) => {
                    match right {
                        Some(y) => evalInfixExpression(Operator, x, y),
                        None => None,
                    }
                }
                None => None,
            }
        },
        _ => None
    }
}

fn evalPrefixExpression(operator: &String, right: object::Object) -> Option<object::Object> {
    match operator.as_str() {
        "!" => Some(evalBangOperatorExpression(right)),
        "-" => evalMinusPrefixOperatorExpression(right),
        _ => None
    }
}

fn evalBangOperatorExpression(right: object::Object) -> object::Object {
    match right {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        _ => FALSE,
    }
}

fn evalMinusPrefixOperatorExpression(right: object::Object) -> Option<object::Object> {
    if let object::Object::Integer{Value} = right {
        Some(object::Object::Integer{Value: -Value})
    } else {
        None
    }
}

fn evalInfixExpression(operator: &String, left: object::Object, right: object::Object) -> Option<object::Object> {
    if let object::Object::Integer{Value: lv} = left {
        if let object::Object::Integer{Value: rv} = right {
            return evalIntegerInfixExpression(operator, lv, rv)
        }
    }

    if *operator == String::from("==") {
        return if left == right {Some(TRUE)} else {Some(FALSE)}
    } 
    else if *operator == String::from("!=") {
        return if left != right {Some(TRUE)} else {Some(FALSE)}
    }

    None
}

fn evalIntegerInfixExpression(operator: &String, left: i64, right:i64) -> Option<object::Object> {
    match operator.as_ref() {
        "+" => Some(object::Object::Integer{Value: left + right}),
        "-" => Some(object::Object::Integer{Value: left - right}),
        "*" => Some(object::Object::Integer{Value: left * right}),
        "/" => Some(object::Object::Integer{Value: left / right}),
        "<" => if left < right {Some(TRUE)} else {Some(FALSE)},
        ">" => if left > right {Some(TRUE)} else {Some(FALSE)},
        "==" => if left == right {Some(TRUE)} else {Some(FALSE)},
        "!=" => if left != right {Some(TRUE)} else {Some(FALSE)},
        _ => None,
    }
}