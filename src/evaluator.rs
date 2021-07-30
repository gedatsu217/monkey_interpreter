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
        } else if let object::Object::Error{Message} = &result {
            return result;
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
            if isError(&val) {return val;}
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
            if isError(&right) {return right}
            if let object::Object::Null = right {
                object::Object::Null
            } else {
                evalPrefixExpression(Operator, right)
            }
        },
        ast::Expression::InfixExpression{Token, Left, Operator, Right} => {
            let left = evalExpression(Left);
            if isError(&left) {return left;}
            let right = evalExpression(Right);
            if isError(&right) {return right;}
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
        _ => newError(format!("unknown operator: {}{}", operator, right.Type())),
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
        newError(format!("unknown operator: -{}", right.Type()))
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

    if left.Type() != right.Type() {
        return newError(format!("type mismatch: {} {} {}", left.Type(), operator, right.Type()));
    }

    newError(format!("unknown operator: {} {} {}", left.Type(), operator, right.Type()))
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
        _ => newError(format!("unknown operator: {} {} {}", object::INTEGER_OBJ, operator, object::INTEGER_OBJ)),
    }
}

fn evalIfExpression(ie: &ast::Expression) -> object::Object {
    if let ast::Expression::IfExpression{Token, Condition, Consequence, Alternative} = ie {
        let condition = evalExpression(Condition);
        if isError(&condition) {return condition;}
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

            if result != object::Object::Null {
                let rt = result.Type();
                if rt == object::RETURN_VALUE_OBJ || rt == object::ERROR_OBJ {
                    return result;
                }
            }
        }
        result
    } else {
        object::Object::Null
    }
}

fn newError(format: String) -> object::Object {
    object::Object::Error{Message: format}
}

fn isError(obj: &object::Object) -> bool {
    obj.Type() == object::ERROR_OBJ
}