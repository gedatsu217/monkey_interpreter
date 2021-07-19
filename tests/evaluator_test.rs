extern crate monkey_interpreter;
use monkey_interpreter::{lexer, object, parser, evaluator};

#[test]
fn TestEvalIntegerExpression() {
    struct tests_struct {
        input: String,
        expected: i64,
    }
    let tests = vec![
        tests_struct{input: String::from("5"), expected: 5},
        tests_struct{input: String::from("10"), expected: 10},
    ];

    for tt in tests.iter() {
        let evaluated = testEval(&tt.input);
        let evaluated = evaluated.unwrap();
        assert_eq!(true, testIntegerObject(&evaluated, tt.expected));
    }

}

fn testEval(input: &String) -> Option<object::Object> {
    let l = lexer::New(input.clone());
    let mut p = l.New();
    let program = p.ParseProgram();
    evaluator::Eval(program)
}

fn testIntegerObject(obj: &object::Object, expected: i64) -> bool {
    if let object::Object::Integer{Value} = obj {
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