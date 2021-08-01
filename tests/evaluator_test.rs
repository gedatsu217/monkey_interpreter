extern crate monkey_interpreter;
use monkey_interpreter::{ast, evaluator, lexer, object, parser};

#[test]
fn TestEvalIntegerExpression() {
    struct tests_struct {
        input: String,
        expected: i64,
    }
    let tests = vec![
        tests_struct {
            input: String::from("5"),
            expected: 5,
        },
        tests_struct {
            input: String::from("10"),
            expected: 10,
        },
        tests_struct {
            input: String::from("-5"),
            expected: -5,
        },
        tests_struct {
            input: String::from("-10"),
            expected: -10,
        },
        tests_struct {
            input: String::from("5 + 5 + 5 + 5 - 10"),
            expected: 10,
        },
        tests_struct {
            input: String::from("2 * 2 * 2 * 2 * 2"),
            expected: 32,
        },
        tests_struct {
            input: String::from("-50 + 100 - 50"),
            expected: 0,
        },
        tests_struct {
            input: String::from("5 * 2 + 10"),
            expected: 20,
        },
        tests_struct {
            input: String::from("5 + 2 * 10"),
            expected: 25,
        },
        tests_struct {
            input: String::from("20 + 2 * -10"),
            expected: 0,
        },
        tests_struct {
            input: String::from("50 / 2 * 2 + 10"),
            expected: 60,
        },
        tests_struct {
            input: String::from("2 * (5 + 10)"),
            expected: 30,
        },
        tests_struct {
            input: String::from("3 * 3 * 3 + 10"),
            expected: 37,
        },
        tests_struct {
            input: String::from("3 * (3 * 3) + 10"),
            expected: 37,
        },
        tests_struct {
            input: String::from("(5 + 10 * 2 + 15 / 3) * 2 - 10"),
            expected: 50,
        },
    ];

    for tt in tests.iter() {
        let evaluated = testEval(&tt.input);
        let evaluated = evaluated;
        assert_eq!(true, testIntegerObject(&evaluated, tt.expected));
    }
}

fn testEval(input: &String) -> object::Object {
    let l = lexer::New(input.clone());
    let mut p = l.New();
    let program = p.ParseProgram();
    let mut env = object::NewEnvironment();
    evaluator::Eval(program, &mut env)
}

fn testIntegerObject(obj: &object::Object, expected: i64) -> bool {
    if let object::Object::Integer { Value } = obj {
        if *Value != expected {
            println!("object has wrong value. got={}, want={}", Value, expected);
            false
        } else {
            true
        }
    } else {
        println!("object is not Integer. got={}", obj);
        false
    }
}

#[test]
fn TestEvalBooleanExpression() {
    struct tests_struct {
        input: String,
        expected: bool,
    }
    let tests = vec![
        tests_struct {
            input: String::from("true"),
            expected: true,
        },
        tests_struct {
            input: String::from("false"),
            expected: false,
        },
        tests_struct {
            input: String::from("1 < 2"),
            expected: true,
        },
        tests_struct {
            input: String::from("1 > 2"),
            expected: false,
        },
        tests_struct {
            input: String::from("1 < 1"),
            expected: false,
        },
        tests_struct {
            input: String::from("1 > 1"),
            expected: false,
        },
        tests_struct {
            input: String::from("1 == 1"),
            expected: true,
        },
        tests_struct {
            input: String::from("1 != 1"),
            expected: false,
        },
        tests_struct {
            input: String::from("1 == 2"),
            expected: false,
        },
        tests_struct {
            input: String::from("1 != 2"),
            expected: true,
        },
        tests_struct {
            input: String::from("true == true"),
            expected: true,
        },
        tests_struct {
            input: String::from("false == false"),
            expected: true,
        },
        tests_struct {
            input: String::from("true == false"),
            expected: false,
        },
        tests_struct {
            input: String::from("true != false"),
            expected: true,
        },
        tests_struct {
            input: String::from("false != true"),
            expected: true,
        },
        tests_struct {
            input: String::from("(1 < 2) == true"),
            expected: true,
        },
        tests_struct {
            input: String::from("(1 < 2) == false"),
            expected: false,
        },
        tests_struct {
            input: String::from("(1 > 2) == true"),
            expected: false,
        },
        tests_struct {
            input: String::from("(1 > 2) == false"),
            expected: true,
        },
    ];

    for tt in tests.iter() {
        let evaluated = testEval(&tt.input);
        assert_eq!(true, testBooleanObject(&evaluated, tt.expected));
    }
}

fn testBooleanObject(obj: &object::Object, expected: bool) -> bool {
    if let object::Object::Boolean { Value } = obj {
        if *Value != expected {
            println!("object has wrong value. got={}, want={}", Value, expected);
            false
        } else {
            true
        }
    } else {
        println!("object is not Boolean. got={}", obj);
        false
    }
}

#[test]
fn TestBangOperator() {
    struct tests_struct {
        input: String,
        expected: bool,
    }
    let tests = vec![
        tests_struct {
            input: String::from("!true"),
            expected: false,
        },
        tests_struct {
            input: String::from("!false"),
            expected: true,
        },
        tests_struct {
            input: String::from("!5"),
            expected: false,
        },
        tests_struct {
            input: String::from("!!true"),
            expected: true,
        },
        tests_struct {
            input: String::from("!!false"),
            expected: false,
        },
        tests_struct {
            input: String::from("!!5"),
            expected: true,
        },
    ];

    for tt in tests.iter() {
        let evaluated = testEval(&tt.input);
        assert_eq!(true, testBooleanObject(&evaluated, tt.expected));
    }
}

#[test]
fn TestIfElseExpressions() {
    struct tests_struct {
        input: String,
        expected: object::Object,
    }

    let tests = vec![
        tests_struct {
            input: String::from("if (true) {10}"),
            expected: object::Object::Integer { Value: 10 },
        },
        tests_struct {
            input: String::from("if (false) {10}"),
            expected: object::Object::Null,
        },
        tests_struct {
            input: String::from("if (1) {10}"),
            expected: object::Object::Integer { Value: 10 },
        },
        tests_struct {
            input: String::from("if (1 < 2) {10}"),
            expected: object::Object::Integer { Value: 10 },
        },
        tests_struct {
            input: String::from("if (1 > 2) {10}"),
            expected: object::Object::Null,
        },
        tests_struct {
            input: String::from("if (1 > 2) {10} else {20}"),
            expected: object::Object::Integer { Value: 20 },
        },
        tests_struct {
            input: String::from("if (1 < 2) {10} else {20}"),
            expected: object::Object::Integer { Value: 10 },
        },
    ];

    for tt in tests.iter() {
        let evaluated = testEval(&tt.input);
        if let object::Object::Integer { Value } = tt.expected {
            assert_eq!(true, testIntegerObject(&evaluated, Value));
        } else {
            assert_eq!(true, testNullObject(&evaluated));
        }
    }
}

fn testNullObject(obj: &object::Object) -> bool {
    if let object::Object::Null = obj {
        true
    } else {
        println!("object is not NULL. got={}", obj);
        false
    }
}

#[test]
fn TestReturnStatements() {
    struct tests_struct {
        input: String,
        expected: i64,
    }

    let tests = vec![
        tests_struct {
            input: String::from("return 10;"),
            expected: 10,
        },
        tests_struct {
            input: String::from("return 10; 9"),
            expected: 10,
        },
        tests_struct {
            input: String::from("return 2 * 5; 9"),
            expected: 10,
        },
        tests_struct {
            input: String::from("9; return 2 * 5; 9;"),
            expected: 10,
        },
        tests_struct {
            input: String::from(
                "\
        if (10 > 1) {
            if (10 > 1) {
                return 10;
            }
            return 1;
        }",
            ),
            expected: 10,
        },
    ];

    for tt in tests.iter() {
        let evaluated = testEval(&tt.input);
        assert_eq!(true, testIntegerObject(&evaluated, tt.expected));
    }
}

#[test]
fn TestErrorHandling() {
    struct tests_struct {
        input: String,
        expected: String,
    }

    let tests = vec![
        tests_struct {
            input: String::from("5 + true;"),
            expected: String::from("type mismatch: INTEGER + BOOLEAN"),
        },
        tests_struct {
            input: String::from("5 + true; 5;"),
            expected: String::from("type mismatch: INTEGER + BOOLEAN"),
        },
        tests_struct {
            input: String::from("-true;"),
            expected: String::from("unknown operator: -BOOLEAN"),
        },
        tests_struct {
            input: String::from("true + false;"),
            expected: String::from("unknown operator: BOOLEAN + BOOLEAN"),
        },
        tests_struct {
            input: String::from("5; true + false; 5"),
            expected: String::from("unknown operator: BOOLEAN + BOOLEAN"),
        },
        tests_struct {
            input: String::from("if (10 > 1) {true + false; }"),
            expected: String::from("unknown operator: BOOLEAN + BOOLEAN"),
        },
        tests_struct {
            input: String::from(
                "\
        if (10 > 1) {
            if (10 > 1) {
                return true + false;
            }
            return 1;
        }
        ",
            ),
            expected: String::from("unknown operator: BOOLEAN + BOOLEAN"),
        },
        tests_struct {
            input: String::from("foobar"),
            expected: String::from("identifier not found: foobar"),
        },
    ];

    for tt in tests.iter() {
        let evaluated = testEval(&tt.input);
        if let object::Object::Error { Message } = evaluated {
            assert_eq!(tt.expected, Message);
        } else {
            println!("no error object returned. got={}", evaluated);
            continue;
        }
    }
}

#[test]
fn TestLetStatements() {
    struct tests_struct {
        input: String,
        expected: i64,
    }

    let tests = vec![
        tests_struct {
            input: String::from("let a = 5; a;"),
            expected: 5,
        },
        tests_struct {
            input: String::from("let a = 5 * 5; a;"),
            expected: 25,
        },
        tests_struct {
            input: String::from("let a = 5; let b = a; b;"),
            expected: 5,
        },
        tests_struct {
            input: String::from("let a = 5; let b = a; let c = a + b + 5; c;"),
            expected: 15,
        },
    ];

    for tt in tests.iter() {
        assert_eq!(true, testIntegerObject(&testEval(&tt.input), tt.expected));
    }
}
