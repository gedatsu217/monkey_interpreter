use crate::{ast, object};

const TRUE: object::Object = object::Object::Boolean{Value: true};
const FALSE: object::Object = object::Object::Boolean{Value: false};
const NULL: object::Object = object::Object::Null;

pub fn Eval(node: ast::Program) -> object::Object {
    evalStatements(&node.Statements)
}

fn evalStatements(stmts: &Vec<ast::Statement>) -> object::Object {
    let mut result = object::Object::Null;
    for statement in stmts.iter() {
        result = evalStatement(statement);
        
        if let object::Object::ReturnValue{Value} = result {
            return *Value;
        }
    }
    result
}

fn evalStatement(stmt: &ast::Statement) -> object::Object {
    match stmt {
        ast::Statement::ExpressionStatement{Token, Expression} => evalExpression(Expression),
        ast::Statement::BlockStatement{Token, Statements} => evalBlockStatement(stmt),
        ast::Statement::ReturnStatement{Token, ReturnValue} => {
            let val = evalExpression(ReturnValue);
            object::Object::ReturnValue{Value: Box::new(val)}
        },
        _ => object::Object::Null,
    }
}

fn evalExpression(exp: &ast::Expression) -> object::Object {
    match exp {
        ast::Expression::IntergerLiteral{Token, Value} => object::Object::Integer{Value: *Value},
        ast::Expression::Boolean{Token, Value} => if *Value {TRUE} else {FALSE},
        ast::Expression::PrefixExpression{Token, Operator, Right} => {
            let right = evalExpression(Right);
            if let object::Object::Null = right {
                object::Object::Null
            } else {
                evalPrefixExpression(Operator, right)
            }
        },
        ast::Expression::InfixExpression{Token, Left, Operator, Right} => {
            let left = evalExpression(Left);
            let right = evalExpression(Right);
            evalInfixExpression(Operator, left, right)
        },
        ast::Expression::IfExpression{Token, Condition, Consequence, Alternative} => {
            evalIfExpression(exp)
        },
        _ => object::Object::Null
    }
}

fn evalPrefixExpression(operator: &String, right: object::Object) -> object::Object {
    match operator.as_str() {
        "!" => evalBangOperatorExpression(right),
        "-" => evalMinusPrefixOperatorExpression(right),
        _ => object::Object::Null
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

fn evalMinusPrefixOperatorExpression(right: object::Object) -> object::Object {
    if let object::Object::Integer{Value} = right {
        object::Object::Integer{Value: -Value}
    } else {
        object::Object::Null
    }
}

fn evalInfixExpression(operator: &String, left: object::Object, right: object::Object) -> object::Object {
    if let object::Object::Integer{Value: lv} = left {
        if let object::Object::Integer{Value: rv} = right {
            return evalIntegerInfixExpression(operator, lv, rv)
        }
    }

    if *operator == String::from("==") {
        return if left == right {TRUE} else {FALSE}
    } 
    else if *operator == String::from("!=") {
        return if left != right {TRUE} else {FALSE}
    }

    object::Object::Null
}

fn evalIntegerInfixExpression(operator: &String, left: i64, right:i64) -> object::Object {
    match operator.as_ref() {
        "+" => object::Object::Integer{Value: left + right},
        "-" => object::Object::Integer{Value: left - right},
        "*" => object::Object::Integer{Value: left * right},
        "/" => object::Object::Integer{Value: left / right},
        "<" => if left < right {TRUE} else {FALSE},
        ">" => if left > right {TRUE} else {FALSE},
        "==" => if left == right {TRUE} else {FALSE},
        "!=" => if left != right {TRUE} else {FALSE},
        _ => object::Object::Null,
    }
}

fn evalIfExpression(ie: &ast::Expression) -> object::Object {
    if let ast::Expression::IfExpression{Token, Condition, Consequence, Alternative} = ie {
        let condition = evalExpression(Condition);
        if isTruthy(&condition) {
            return evalStatement(Consequence);
        } 
        if let ast::Statement::Nil = Alternative.as_ref() {
            return object::Object::Null;
        } else {
            return evalStatement(Alternative);
        }
    } else {
        panic!("ie is not ast::Expression::IfExpression. got={}", ie);
    }
}

fn isTruthy(obj: &object::Object) -> bool {
    match *obj {
        object::Object::Null => false,
        TRUE => true,
        FALSE => false,
        _ => true,
    }
}

fn evalBlockStatement(block: &ast::Statement) -> object::Object {
    if let ast::Statement::BlockStatement{Token, Statements} = block {
        let mut result: object::Object = object::Object::Null;
        for statement in Statements.iter() {
            result = evalStatement(statement);

            if result != object::Object::Null && result.Type() == object::RETURN_VALUE_OBJ{
                return result;
            }
        }
        result
    } else {
        object::Object::Null
    }
}