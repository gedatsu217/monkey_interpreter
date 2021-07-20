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