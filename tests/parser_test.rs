extern crate monkey_interpreter;
use monkey_interpreter::{ast, lexer, parser};

struct ExpectedIdentifiers {
    expectedIdentifier: String,
}

#[test]
fn TestLetStatements() {
    let input = String::from("\
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ");

    let l = lexer::New(input);
    let mut p = l.New();

    let program = p.ParseProgram();
    p.checkParserErrors();

    assert_eq!(3, program.Statements.len(), "program.Statesments does not contain 3 statements. got={}", program.Statements.len());
    
    let tests = vec![
        ExpectedIdentifiers{expectedIdentifier: String::from("x")},
        ExpectedIdentifiers{expectedIdentifier: String::from("y")},
        ExpectedIdentifiers{expectedIdentifier: String::from("foobar")},
    ];

    for (i, tt) in tests.iter().enumerate() {
        let stmt = &program.Statements[i];
        testLetStatement(stmt, &tt.expectedIdentifier);
    }
}

fn testLetStatement(s: &ast::Statement, name: &String) -> bool{
    if let ast::Statement::LetStatement{Token, Name, ..} = s {
        if Token.Literal != String::from("let") {
            println!("LetStatement.Token.Literal not 'let'. got={}", Token.Literal);
            return false;
        }

        if Name.Value != *name {
            println!("Letstatement.Name.Value not {}, got={}", name, Name.Value);
            return false;
        }

        if Name.Token.Literal != *name {
            println!("LetStatement.Name.Token.Literal not {}. got={}", name, Name.Token.Literal);
            return false;
        }

        return true;
    } 
    false
}

#[test]
fn TestReturnStatement() {
    let input  =String::from("\
    return 5;
    return 10;
    return 993322;
    ");

    let l = lexer::New(input);
    let mut p = l.New();

    let program = p.ParseProgram();
    p.checkParserErrors();

    assert_eq!(3, program.Statements.len(), "program.Statements does not contain 3 statements. got={}", program.Statements.len());

    for stmt in program.Statements.iter() {
        if let ast::Statement::ReturnStatement{Token, ReturnValue} = stmt {
            if Token.Literal != String::from("return") {
                println!("ReturnStatement.Token.Literal not 'return'. got {}", Token.Literal);
            }
        } else {
            println!("stmt not returnStatement. got={}", stmt);
        }
    }
}

#[test]
fn TestIdentifierExpression() {
    let input = String::from("foobar;");
    let l = lexer::New(input);
    let mut p = l.New();
    let mut program = p.ParseProgram();
    p.checkParserErrors();
    
    assert_eq!(1, program.Statements.len(), "program has not enough statements. got={}", program.Statements.len());

    let stmt = &program.Statements[0];

    if let ast::Statement::ExpressionStatement{Token, Expression} = stmt {
        if let ast::Expression::Identifier(x) = Expression {
            if x.Value != String::from("foobar") {
                println!("x.Value not foobar. got={}", x.Value);
            }
            if x.Token.Literal != String::from("foobar") {
                println!("x.Token.Literal not foobar. got={}", x.Token.Literal);
            }
        } else {
            println!("exp not ast::Expression::Identifier. got={}", Expression);
            panic!();
        }
    } else {
        println!("program.Statements[0] is not ast::Statement::ExpressionStatement. got={}", program.Statements[0]);
        panic!();
    }
}

#[test]
fn TestIntegerLiteralExpression() {
    let input = String::from("5;");
    let l = lexer::New(input);
    let mut p = l.New();
    let program = p.ParseProgram();
    p.checkParserErrors();

    assert_eq!(1, program.Statements.len(), "program has not enough statements. got={}", program.Statements.len());

    let stmt = &program.Statements[0];

    if let ast::Statement::ExpressionStatement{Token, Expression} = stmt {
        if let ast::Expression::IntergerLiteral{Token, Value} = Expression {
            if *Value != 5 {
                println!("Value not foobar. got={}", Value);
            }
            if Token.Literal != String::from("5") {
                println!("Token.Literal not 5. got={}", Token.Literal);
            }
        } else {
            println!("exp not ast::Expression::IntegerLiteral. got={}", Expression);
            panic!();
        }
    } else {
        println!("program.Statements[0] is not ast::Statement::ExpressionStatement. got={}", program.Statements[0]);
        panic!();
    }
}

#[test]
fn TestParsingPrefixExpression() {
    struct prefixTests_struct {
        input: String,
        operator: String,
        integerValue: i64,
    }

    let prefixTests = vec![
        prefixTests_struct{input: String::from("!5;"), operator: String::from("!"), integerValue: 5},
        prefixTests_struct{input: String::from("-15;"), operator: String::from("-"), integerValue: 15},
    ];

    for tt in prefixTests.iter() {
        let l = lexer::New(tt.input.clone());
        let mut p = l.New();
        let program = p.ParseProgram();
        p.checkParserErrors();

        assert_eq!(1, program.Statements.len(), "program.Statements does not contain 1 statements. got={}", program.Statements.len());

        let stmt = &program.Statements[0];
        if let ast::Statement::ExpressionStatement{Token, Expression} = stmt {
            if let ast::Expression::PrefixExpression{Token, Operator, Right} = Expression {
                if *Operator != tt.operator {
                    panic!("Operator is not {}. got={}", tt.operator, Operator);
                }
                if !testIntegerLiteral(Right, tt.integerValue) {
                    return
                }
            } else {
                println!("stmt is not ast::Expression::PrefixExpression. got={}", Expression);
                panic!();
            }
        } else {
            println!("program.Statements[0] is not ast::Statement::ExpressionStatement. got={}", program.Statements[0]);
            panic!();
        }
    }

}

fn testIntegerLiteral(il: &ast::Expression, value: i64) -> bool {
    if let ast::Expression::IntergerLiteral{Token, Value} = il {
        if *Value == value {
            if Token.Literal == value.to_string() {
                return true;
            } else {
                println!("Token.Literal not {}. got={}", value, Token.Literal);
            }
        } else {
            println!("Value not {}. got={}", value, Value);
        }
    } else {
        println!("il not ast::Expression::IntegerLiteral. got={}", il);
    }
    false
}

#[test]
fn TestParsingInfixExpressions() {
    struct infixTests_struct {
        input: String,
        leftValue: i64,
        operator: String,
        rightValue: i64,
    }
    let infixTests = vec![
        infixTests_struct{input: String::from("5 + 5;"), leftValue: 5, operator: String::from("+"), rightValue: 5},
        infixTests_struct{input: String::from("5 - 5;"), leftValue: 5, operator: String::from("-"), rightValue: 5},
        infixTests_struct{input: String::from("5 * 5;"), leftValue: 5, operator: String::from("*"), rightValue: 5},
        infixTests_struct{input: String::from("5 / 5;"), leftValue: 5, operator: String::from("/"), rightValue: 5},
        infixTests_struct{input: String::from("5 > 5;"), leftValue: 5, operator: String::from(">"), rightValue: 5},
        infixTests_struct{input: String::from("5 < 5;"), leftValue: 5, operator: String::from("<"), rightValue: 5},
        infixTests_struct{input: String::from("5 == 5;"), leftValue: 5, operator: String::from("=="), rightValue: 5},
        infixTests_struct{input: String::from("5 != 5;"), leftValue: 5, operator: String::from("!="), rightValue: 5},
    ];

    for tt in infixTests.iter() {
        let l = lexer::New(tt.input.clone());
        let mut p = l.New();
        let program = p.ParseProgram();
        p.checkParserErrors();

        assert_eq!(1, program.Statements.len(), "program.Statements does not contain 1 statements. got={}", program.Statements.len());

        let stmt = &program.Statements[0];

        if let ast::Statement::ExpressionStatement{Token, Expression} = stmt {
            if let ast::Expression::InfixExpression{Token, Left, Operator, Right} = Expression {
                if !testIntegerLiteral(Left, tt.leftValue) {
                    return
                }

                if *Operator != tt.operator {
                    panic!("Operator is not {}. got={}", tt.operator, Operator);
                }

                if !testIntegerLiteral(Right, tt.rightValue) {
                    return
                }
            } else {
                panic!("Expression is not ast::Expression::InfixExpression. got={}", Expression);
            }
        } else {
            panic!("program.Statements[0] is not ast::Statement::ExpressionStatement. got={}", program.Statements[0]);
        }
    }

}
#[test]
fn TestOperatorPrecedenceParsing() {
    struct tests_struct {
        input: String,
        expected: String,
    }

    let tests = vec![
        tests_struct{input: String::from("-a * b"), expected: String::from("((-a) * b)")},
        tests_struct{input: String::from("!-a"), expected: String::from("(!(-a))")},
        tests_struct{input: String::from("a + b + c"), expected: String::from("((a + b) + c)")},
        tests_struct{input: String::from("a + b - c"), expected: String::from("((a + b) - c)")},
        tests_struct{input: String::from("a * b * c"), expected: String::from("((a * b) * c)")},
        tests_struct{input: String::from("a * b / c"), expected: String::from("((a * b) / c)")},
        tests_struct{input: String::from("a + b / c"), expected: String::from("(a + (b / c))")},
        tests_struct{input: String::from("a + b * c + d / e - f"), expected: String::from("(((a + (b * c)) + (d / e)) - f)")},
        tests_struct{input: String::from("3 + 4; -5 * 5"), expected: String::from("(3+4)((-5) * 5)")},
        tests_struct{input: String::from("5 > 4 == 3 < 4"), expected: String::from("((5 > 4) == (3 < 4))")},
        tests_struct{input: String::from("5 < 4 != 3 > 4"), expected: String::from("((5 < 4) != (3 > 4))")},
        tests_struct{input: String::from("3 + 4 * 5 == 3 * 1 + 4 * 5"), expected: String::from("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))")},
    ];

    for tt in tests.iter() {
        let l = lexer::New(tt.input.clone());
        let mut p = l.New();
        let program = p.ParseProgram();
        p.checkParserErrors();

        let actual = program.into_string();
        if actual != tt.expected {
            println!("expected={}, got={}", tt.expected, actual);
        }

    }
}

