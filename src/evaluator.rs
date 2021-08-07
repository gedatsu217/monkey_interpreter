use crate::{ast, object};

const TRUE: object::Object = object::Object::Boolean { Value: true };
const FALSE: object::Object = object::Object::Boolean { Value: false };
const NULL: object::Object = object::Object::Null;

pub fn Eval(node: ast::Program, env: &mut object::Environment) -> object::Object {
    evalStatements(&node.Statements, env)
}

fn evalStatements(stmts: &Vec<ast::Statement>, env: &mut object::Environment) -> object::Object {
    let mut result = object::Object::Null;
    for statement in stmts.iter() {
        result = evalStatement(statement, env);

        if let object::Object::ReturnValue { Value } = result {
            return *Value;
        } else if let object::Object::Error { Message } = &result {
            return result;
        }
    }
    result
}

fn evalStatement(stmt: &ast::Statement, env: &mut object::Environment) -> object::Object {
    match stmt {
        ast::Statement::ExpressionStatement { Token, Expression } => {
            evalExpression(Expression, env)
        }
        ast::Statement::BlockStatement { Token, Statements } => evalBlockStatement(stmt, env),
        ast::Statement::ReturnStatement { Token, ReturnValue } => {
            let val = evalExpression(ReturnValue, env);
            if isError(&val) {
                return val;
            }
            object::Object::ReturnValue {
                Value: Box::new(val),
            }
        }
        ast::Statement::LetStatement { Token, Name, Value } => {
            let val = evalExpression(Value, env);
            if isError(&val) {
                return val;
            }
            env.Set(&Name.Value, val)
        }
        _ => object::Object::Null,
    }
}

fn evalExpression(exp: &ast::Expression, env: &mut object::Environment) -> object::Object {
    match exp {
        ast::Expression::IntergerLiteral { Token, Value } => {
            object::Object::Integer { Value: *Value }
        }
        ast::Expression::Boolean { Token, Value } => {
            if *Value {
                TRUE
            } else {
                FALSE
            }
        }
        ast::Expression::PrefixExpression {
            Token,
            Operator,
            Right,
        } => {
            let right = evalExpression(Right, env);
            if isError(&right) {
                return right;
            }
            if let object::Object::Null = right {
                object::Object::Null
            } else {
                evalPrefixExpression(Operator, right)
            }
        }
        ast::Expression::InfixExpression {
            Token,
            Left,
            Operator,
            Right,
        } => {
            let left = evalExpression(Left, env);
            if isError(&left) {
                return left;
            }
            let right = evalExpression(Right, env);
            if isError(&right) {
                return right;
            }
            evalInfixExpression(Operator, left, right)
        }
        ast::Expression::IfExpression {
            Token,
            Condition,
            Consequence,
            Alternative,
        } => evalIfExpression(exp, env),
        ast::Expression::Identifier(idt) => evalIdentifier(idt, env),
        ast::Expression::FunctionLiteral {
            Token,
            Parameters,
            Body,
        } => object::Object::Function {
            Parameters: Parameters.clone(),
            Body: Body.clone(),
            Env: env.clone(),
        },
        ast::Expression::CallExpression {
            Token,
            Function,
            Arguments,
        } => {
            let function = evalExpression(Function, env);
            if isError(&function) {
                return function;
            }
            let mut args = evalExpressions(Arguments, env);
            if args.len() == 1 && isError(&args[0]) {
                return args[0].clone();
            }

            applyFunction(function, args)
        }
        _ => object::Object::Null,
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
    if let object::Object::Integer { Value } = right {
        object::Object::Integer { Value: -Value }
    } else {
        newError(format!("unknown operator: -{}", right.Type()))
    }
}

fn evalInfixExpression(
    operator: &String,
    left: object::Object,
    right: object::Object,
) -> object::Object {
    if let object::Object::Integer { Value: lv } = left {
        if let object::Object::Integer { Value: rv } = right {
            return evalIntegerInfixExpression(operator, lv, rv);
        }
    }

    if *operator == String::from("==") {
        return if left == right { TRUE } else { FALSE };
    } else if *operator == String::from("!=") {
        return if left != right { TRUE } else { FALSE };
    }

    if left.Type() != right.Type() {
        return newError(format!(
            "type mismatch: {} {} {}",
            left.Type(),
            operator,
            right.Type()
        ));
    }

    newError(format!(
        "unknown operator: {} {} {}",
        left.Type(),
        operator,
        right.Type()
    ))
}

fn evalIntegerInfixExpression(operator: &String, left: i64, right: i64) -> object::Object {
    match operator.as_ref() {
        "+" => object::Object::Integer {
            Value: left + right,
        },
        "-" => object::Object::Integer {
            Value: left - right,
        },
        "*" => object::Object::Integer {
            Value: left * right,
        },
        "/" => object::Object::Integer {
            Value: left / right,
        },
        "<" => {
            if left < right {
                TRUE
            } else {
                FALSE
            }
        }
        ">" => {
            if left > right {
                TRUE
            } else {
                FALSE
            }
        }
        "==" => {
            if left == right {
                TRUE
            } else {
                FALSE
            }
        }
        "!=" => {
            if left != right {
                TRUE
            } else {
                FALSE
            }
        }
        _ => newError(format!(
            "unknown operator: {} {} {}",
            object::INTEGER_OBJ,
            operator,
            object::INTEGER_OBJ
        )),
    }
}

fn evalIfExpression(ie: &ast::Expression, env: &mut object::Environment) -> object::Object {
    if let ast::Expression::IfExpression {
        Token,
        Condition,
        Consequence,
        Alternative,
    } = ie
    {
        let condition = evalExpression(Condition, env);
        if isError(&condition) {
            return condition;
        }
        if isTruthy(&condition) {
            return evalStatement(Consequence, env);
        }
        if let ast::Statement::Nil = Alternative.as_ref() {
            return object::Object::Null;
        } else {
            return evalStatement(Alternative, env);
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

fn evalBlockStatement(block: &ast::Statement, env: &mut object::Environment) -> object::Object {
    if let ast::Statement::BlockStatement { Token, Statements } = block {
        let mut result: object::Object = object::Object::Null;
        for statement in Statements.iter() {
            result = evalStatement(statement, env);

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
    object::Object::Error { Message: format }
}

fn isError(obj: &object::Object) -> bool {
    obj.Type() == object::ERROR_OBJ
}

fn evalIdentifier(node: &ast::Identifier, env: &mut object::Environment) -> object::Object {
    let val = env.Get(&node.Value);
    match val {
        Some(s) => s.clone(),
        None => newError(format!("identifier not found: {}", node.Value)),
    }
}

fn evalExpressions(
    exps: &Vec<ast::Expression>,
    env: &mut object::Environment,
) -> Vec<object::Object> {
    let mut result = vec![];
    for e in exps.iter() {
        let evaluated = evalExpression(e, env);
        if isError(&evaluated) {
            return vec![evaluated];
        }
        result.push(evaluated);
    }

    result
}

fn applyFunction(f: object::Object, args: Vec<object::Object>) -> object::Object {
    if let object::Object::Function {
        Parameters,
        Body,
        Env,
    } = &f
    {
        let mut extendedEnv = extendFunctionEnv(f.clone(), args);
        let evaluated = evalStatement(Body, &mut extendedEnv);
        unwrapReturnValue(evaluated)
    } else {
        return newError(format!("not a function: {}", f.Type()));
    }
}

fn extendFunctionEnv(f: object::Object, args: Vec<object::Object>) -> object::Environment {
    if let object::Object::Function {
        Parameters,
        Body,
        Env,
    } = f
    {
        let mut env = object::NewEnclosedEnvironment(Env);
        for (i, param) in Parameters.iter().enumerate() {
            if let ast::Expression::Identifier(x) = param {
                env.Set(&x.Value, args[i].clone());
            } else {
                panic!();
            }
        }
        env
    } else {
        panic!();
    }
}

fn unwrapReturnValue(obj: object::Object) -> object::Object {
    if let object::Object::ReturnValue { Value } = obj {
        *Value
    } else {
        obj
    }
}
