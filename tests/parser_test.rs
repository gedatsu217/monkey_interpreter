extern crate monkey_interpreter;
use monkey_interpreter::{ast, lexer, parser, token, token::Token};

struct ExpectedIdentifiers {
    expectedIdentifier: String,
}

#[test]
fn TestLetStatements() {
    struct tests_struct {
        input: String,
        expectedIdentifier: String,
        expectedValue: ast::Expression,
    }

    let tests = vec![
        tests_struct {
            input: String::from("let x = 5;"),
            expectedIdentifier: String::from("x"),
            expectedValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5 as i64,
            },
        },
        tests_struct {
            input: String::from("let y = true;"),
            expectedIdentifier: String::from("y"),
            expectedValue: ast::Expression::Boolean {
                Token: Token {
                    Type: token::TRUE,
                    Literal: String::from("true"),
                },
                Value: true,
            },
        },
        tests_struct {
            input: String::from("let foobar = y;"),
            expectedIdentifier: String::from("foobar"),
            expectedValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("y"),
                },
                Value: String::from("y"),
            }),
        },
    ];

    for tt in tests.iter() {
        let l = lexer::New(tt.input.clone());
        let mut p = l.New();

        let program = p.ParseProgram();
        p.checkParserErrors();

        assert_eq!(
            1,
            program.Statements.len(),
            "program.Statesments does not contain 1 statements. got={}",
            program.Statements.len()
        );

        let stmt = &program.Statements[0];
        assert_eq!(true, testLetStatement(stmt, &tt.expectedIdentifier));

        if let ast::Statement::LetStatement { Token, Name, Value } = stmt {
            assert_eq!(
                true,
                testLiteralExpression(Value, &tt.expectedValue),
                "Value not expected"
            );
        } else {
            panic!("stmt not ast::Statement::LetStatement");
        }
    }
}

fn testLetStatement(s: &ast::Statement, name: &String) -> bool {
    if let ast::Statement::LetStatement { Token, Name, .. } = s {
        if Token.Literal != String::from("let") {
            println!(
                "LetStatement.Token.Literal not 'let'. got={}",
                Token.Literal
            );
            return false;
        }

        if Name.Value != *name {
            println!("Letstatement.Name.Value not {}, got={}", name, Name.Value);
            return false;
        }

        if Name.Token.Literal != *name {
            println!(
                "LetStatement.Name.Token.Literal not {}. got={}",
                name, Name.Token.Literal
            );
            return false;
        }

        return true;
    }
    false
}

#[test]
fn TestReturnStatement() {
    struct tests_struct {
        input: String,
        expectedValue: ast::Expression,
    }

    let tests = vec![
        tests_struct {
            input: String::from("return 5;"),
            expectedValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
        },
        tests_struct {
            input: String::from("return true;"),
            expectedValue: ast::Expression::Boolean {
                Token: Token {
                    Type: token::TRUE,
                    Literal: String::from("true"),
                },
                Value: true,
            },
        },
        tests_struct {
            input: String::from("return foobar;"),
            expectedValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
        },
    ];

    for tt in tests.iter() {
        let l = lexer::New(tt.input.clone());
        let mut p = l.New();

        let program = p.ParseProgram();
        p.checkParserErrors();

        assert_eq!(
            1,
            program.Statements.len(),
            "program.Statements does not contain 1 statements. got={}",
            program.Statements.len()
        );

        let stmt = &program.Statements[0];

        if let ast::Statement::ReturnStatement { Token, ReturnValue } = stmt {
            assert_eq!(
                Token.Literal,
                String::from("return"),
                "Token.Literal not return. got={}",
                Token.Literal
            );
            assert_eq!(testLiteralExpression(ReturnValue, &tt.expectedValue), true);
        } else {
            panic!("stmt not returnStatement. got={}", stmt);
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

    assert_eq!(
        1,
        program.Statements.len(),
        "program has not enough statements. got={}",
        program.Statements.len()
    );

    let stmt = &program.Statements[0];

    if let ast::Statement::ExpressionStatement { Token, Expression } = stmt {
        if let ast::Expression::Identifier(x) = Expression {
            assert_eq!(x.Value, String::from("foobar"));
            assert_eq!(x.Token.Literal, String::from("foobar"))
        } else {
            panic!("exp not ast::Expression::Identifier. got={}", Expression);
        }
    } else {
        panic!(
            "program.Statements[0] is not ast::Statement::ExpressionStatement. got={}",
            program.Statements[0]
        );
    }
}

#[test]
fn TestIntegerLiteralExpression() {
    let input = String::from("5;");
    let l = lexer::New(input);
    let mut p = l.New();
    let program = p.ParseProgram();
    p.checkParserErrors();

    assert_eq!(
        1,
        program.Statements.len(),
        "program has not enough statements. got={}",
        program.Statements.len()
    );

    let stmt = &program.Statements[0];

    if let ast::Statement::ExpressionStatement { Token, Expression } = stmt {
        if let ast::Expression::IntergerLiteral { Token, Value } = Expression {
            assert_eq!(*Value, 5);
            assert_eq!(Token.Literal, String::from("5"))
        } else {
            panic!(
                "exp not ast::Expression::IntegerLiteral. got={}",
                Expression
            );
        }
    } else {
        panic!(
            "program.Statements[0] is not ast::Statement::ExpressionStatement. got={}",
            program.Statements[0]
        );
    }
}

#[test]
fn TestParsingPrefixExpression() {
    struct prefixTests_struct {
        input: String,
        operator: String,
        value: ast::Expression,
    }

    let prefixTests = vec![
        prefixTests_struct {
            input: String::from("!5;"),
            operator: String::from("!"),
            value: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
        },
        prefixTests_struct {
            input: String::from("-15;"),
            operator: String::from("-"),
            value: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("15"),
                },
                Value: 15,
            },
        },
        prefixTests_struct {
            input: String::from("!foobar;"),
            operator: String::from("!"),
            value: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
        },
        prefixTests_struct {
            input: String::from("-foobar;"),
            operator: String::from("-"),
            value: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
        },
        prefixTests_struct {
            input: String::from("!true"),
            operator: String::from("!"),
            value: ast::Expression::Boolean {
                Token: Token {
                    Type: token::TRUE,
                    Literal: String::from("true"),
                },
                Value: true,
            },
        },
        prefixTests_struct {
            input: String::from("!false"),
            operator: String::from("!"),
            value: ast::Expression::Boolean {
                Token: Token {
                    Type: token::FALSE,
                    Literal: String::from("false"),
                },
                Value: false,
            },
        },
    ];

    for tt in prefixTests.iter() {
        let l = lexer::New(tt.input.clone());
        let mut p = l.New();
        let program = p.ParseProgram();
        p.checkParserErrors();

        assert_eq!(
            1,
            program.Statements.len(),
            "program.Statements does not contain 1 statements. got={}",
            program.Statements.len()
        );

        let stmt = &program.Statements[0];
        if let ast::Statement::ExpressionStatement { Token, Expression } = stmt {
            if let ast::Expression::PrefixExpression {
                Token,
                Operator,
                Right,
            } = Expression
            {
                assert_eq!(
                    *Operator, tt.operator,
                    "Operator is not {}. got={}",
                    tt.operator, Operator
                );
                assert_eq!(testLiteralExpression(Right, &tt.value), true);
            } else {
                panic!(
                    "stmt is not ast::Expression::PrefixExpression. got={}",
                    Expression
                );
            }
        } else {
            panic!(
                "program.Statements[0] is not ast::Statement::ExpressionStatement. got={}",
                program.Statements[0]
            );
        }
    }
}

fn testIntegerLiteral(il: &ast::Expression, value: i64) -> bool {
    if let ast::Expression::IntergerLiteral { Token, Value } = il {
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
        leftValue: ast::Expression,
        operator: String,
        rightValue: ast::Expression,
    }
    let infixTests = vec![
        infixTests_struct {
            input: String::from("5 + 5;"),
            leftValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
            operator: String::from("+"),
            rightValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
        },
        infixTests_struct {
            input: String::from("5 - 5;"),
            leftValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
            operator: String::from("-"),
            rightValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
        },
        infixTests_struct {
            input: String::from("5 * 5;"),
            leftValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
            operator: String::from("*"),
            rightValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
        },
        infixTests_struct {
            input: String::from("5 / 5;"),
            leftValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
            operator: String::from("/"),
            rightValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
        },
        infixTests_struct {
            input: String::from("5 > 5;"),
            leftValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
            operator: String::from(">"),
            rightValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
        },
        infixTests_struct {
            input: String::from("5 < 5;"),
            leftValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
            operator: String::from("<"),
            rightValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
        },
        infixTests_struct {
            input: String::from("5 == 5;"),
            leftValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
            operator: String::from("=="),
            rightValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
        },
        infixTests_struct {
            input: String::from("5 != 5;"),
            leftValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
            operator: String::from("!="),
            rightValue: ast::Expression::IntergerLiteral {
                Token: Token {
                    Type: token::INT,
                    Literal: String::from("5"),
                },
                Value: 5,
            },
        },
        infixTests_struct {
            input: String::from("foobar + barfoo"),
            leftValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
            operator: String::from("+"),
            rightValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("barfoo"),
                },
                Value: String::from("barfoo"),
            }),
        },
        infixTests_struct {
            input: String::from("foobar - barfoo"),
            leftValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
            operator: String::from("-"),
            rightValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("barfoo"),
                },
                Value: String::from("barfoo"),
            }),
        },
        infixTests_struct {
            input: String::from("foobar * barfoo"),
            leftValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
            operator: String::from("*"),
            rightValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("barfoo"),
                },
                Value: String::from("barfoo"),
            }),
        },
        infixTests_struct {
            input: String::from("foobar / barfoo"),
            leftValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
            operator: String::from("/"),
            rightValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("barfoo"),
                },
                Value: String::from("barfoo"),
            }),
        },
        infixTests_struct {
            input: String::from("foobar > barfoo"),
            leftValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
            operator: String::from(">"),
            rightValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("barfoo"),
                },
                Value: String::from("barfoo"),
            }),
        },
        infixTests_struct {
            input: String::from("foobar < barfoo"),
            leftValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
            operator: String::from("<"),
            rightValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("barfoo"),
                },
                Value: String::from("barfoo"),
            }),
        },
        infixTests_struct {
            input: String::from("foobar == barfoo"),
            leftValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
            operator: String::from("=="),
            rightValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("barfoo"),
                },
                Value: String::from("barfoo"),
            }),
        },
        infixTests_struct {
            input: String::from("foobar != barfoo"),
            leftValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("foobar"),
                },
                Value: String::from("foobar"),
            }),
            operator: String::from("!="),
            rightValue: ast::Expression::Identifier(ast::Identifier {
                Token: Token {
                    Type: token::IDENT,
                    Literal: String::from("barfoo"),
                },
                Value: String::from("barfoo"),
            }),
        },
        infixTests_struct {
            input: String::from("true == true"),
            leftValue: ast::Expression::Boolean {
                Token: Token {
                    Type: token::TRUE,
                    Literal: String::from("true"),
                },
                Value: true,
            },
            operator: String::from("=="),
            rightValue: ast::Expression::Boolean {
                Token: Token {
                    Type: token::TRUE,
                    Literal: String::from("true"),
                },
                Value: true,
            },
        },
        infixTests_struct {
            input: String::from("true != false"),
            leftValue: ast::Expression::Boolean {
                Token: Token {
                    Type: token::TRUE,
                    Literal: String::from("true"),
                },
                Value: true,
            },
            operator: String::from("!="),
            rightValue: ast::Expression::Boolean {
                Token: Token {
                    Type: token::FALSE,
                    Literal: String::from("false"),
                },
                Value: false,
            },
        },
        infixTests_struct {
            input: String::from("false == false"),
            leftValue: ast::Expression::Boolean {
                Token: Token {
                    Type: token::FALSE,
                    Literal: String::from("false"),
                },
                Value: false,
            },
            operator: String::from("=="),
            rightValue: ast::Expression::Boolean {
                Token: Token {
                    Type: token::FALSE,
                    Literal: String::from("false"),
                },
                Value: false,
            },
        },
    ];

    for tt in infixTests.iter() {
        let l = lexer::New(tt.input.clone());
        let mut p = l.New();
        let program = p.ParseProgram();
        p.checkParserErrors();

        assert_eq!(
            1,
            program.Statements.len(),
            "program.Statements does not contain 1 statements. got={}",
            program.Statements.len()
        );

        let stmt = &program.Statements[0];

        if let ast::Statement::ExpressionStatement { Token, Expression } = stmt {
            if let ast::Expression::InfixExpression {
                Token,
                Left,
                Operator,
                Right,
            } = Expression
            {
                assert_eq!(
                    testInfixExpression(Expression, Left, Operator.clone(), Right),
                    true
                );
            } else {
                panic!(
                    "Expression is not ast::Expression::InfixExpression. got={}",
                    Expression
                );
            }
        } else {
            panic!(
                "program.Statements[0] is not ast::Statement::ExpressionStatement. got={}",
                program.Statements[0]
            );
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
        tests_struct {
            input: String::from("-a * b"),
            expected: String::from("((-a) * b)"),
        },
        tests_struct {
            input: String::from("!-a"),
            expected: String::from("(!(-a))"),
        },
        tests_struct {
            input: String::from("a + b + c"),
            expected: String::from("((a + b) + c)"),
        },
        tests_struct {
            input: String::from("a + b - c"),
            expected: String::from("((a + b) - c)"),
        },
        tests_struct {
            input: String::from("a * b * c"),
            expected: String::from("((a * b) * c)"),
        },
        tests_struct {
            input: String::from("a * b / c"),
            expected: String::from("((a * b) / c)"),
        },
        tests_struct {
            input: String::from("a + b / c"),
            expected: String::from("(a + (b / c))"),
        },
        tests_struct {
            input: String::from("a + b * c + d / e - f"),
            expected: String::from("(((a + (b * c)) + (d / e)) - f)"),
        },
        tests_struct {
            input: String::from("3 + 4; -5 * 5"),
            expected: String::from("(3 + 4)((-5) * 5)"),
        },
        tests_struct {
            input: String::from("5 > 4 == 3 < 4"),
            expected: String::from("((5 > 4) == (3 < 4))"),
        },
        tests_struct {
            input: String::from("5 < 4 != 3 > 4"),
            expected: String::from("((5 < 4) != (3 > 4))"),
        },
        tests_struct {
            input: String::from("3 + 4 * 5 == 3 * 1 + 4 * 5"),
            expected: String::from("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
        },
        tests_struct {
            input: String::from("true"),
            expected: String::from("true"),
        },
        tests_struct {
            input: String::from("false"),
            expected: String::from("false"),
        },
        tests_struct {
            input: String::from("3 > 5 == false"),
            expected: String::from("((3 > 5) == false)"),
        },
        tests_struct {
            input: String::from("3 < 5 == true"),
            expected: String::from("((3 < 5) == true)"),
        },
        tests_struct {
            input: String::from("1 + (2 + 3) + 4"),
            expected: String::from("((1 + (2 + 3)) + 4)"),
        },
        tests_struct {
            input: String::from("(5 + 5) * 2"),
            expected: String::from("((5 + 5) * 2)"),
        },
        tests_struct {
            input: String::from("2 / (5 + 5)"),
            expected: String::from("(2 / (5 + 5))"),
        },
        tests_struct {
            input: String::from("-(5 + 5)"),
            expected: String::from("(-(5 + 5))"),
        },
        tests_struct {
            input: String::from("!(true == true)"),
            expected: String::from("(!(true == true))"),
        },
        tests_struct {
            input: String::from("a + add(b * c) + d"),
            expected: String::from("((a + add((b * c))) + d)"),
        },
        tests_struct {
            input: String::from("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))"),
            expected: String::from("add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
        },
        tests_struct {
            input: String::from("add(a + b + c * d / f + g)"),
            expected: String::from("add((((a + b) + ((c * d) / f)) + g))"),
        },
    ];

    for tt in tests.iter() {
        let l = lexer::New(tt.input.clone());
        let mut p = l.New();
        let program = p.ParseProgram();
        p.checkParserErrors();

        let actual = program.into_string();
        assert_eq!(
            actual, tt.expected,
            "expected={}, got={}",
            tt.expected, actual
        );
    }
}

fn testIdentifier(exp: &ast::Expression, value: String) -> bool {
    if let ast::Expression::Identifier(x) = exp {
        if x.Value == value {
            if x.Token.Literal == value {
                true
            } else {
                println!("x.Token.Literal not {}. got={}", value, x.Token.Literal);
                false
            }
        } else {
            println!("x.Value not {}. got={}", value, x.Value);
            false
        }
    } else {
        println!("exp not ast::Expression::Identifier. got={}", exp);
        false
    }
}

fn testLiteralExpression(exp: &ast::Expression, expected: &ast::Expression) -> bool {
    match expected {
        ast::Expression::IntergerLiteral { Token, Value } => testIntegerLiteral(&exp, *Value),
        ast::Expression::Identifier(x) => testIdentifier(exp, x.Value.clone()),
        ast::Expression::Boolean { Token, Value } => testBooleanLiteral(exp, *Value),
        _ => {
            println!("type of exp not handled. got={}", exp);
            false
        }
    }
}

fn testInfixExpression(
    exp: &ast::Expression,
    left: &ast::Expression,
    operator: String,
    right: &ast::Expression,
) -> bool {
    if let ast::Expression::InfixExpression {
        Token,
        Left,
        Operator,
        Right,
    } = exp
    {
        if !testLiteralExpression(Left, left) {
            return false;
        }

        if *Operator != operator {
            println!("Operator is not {}. got={}", operator, Operator);
            return false;
        }

        if !testLiteralExpression(Right, right) {
            return false;
        }

        true
    } else {
        println!("exp is not ast::Expression::InfixExpression. got={}", exp);
        false
    }
}

#[test]
fn TestBooleanExpression() {
    struct tests_struct {
        input: String,
        expectedBoolean: bool,
    }

    let tests = vec![
        tests_struct {
            input: String::from("true;"),
            expectedBoolean: true,
        },
        tests_struct {
            input: String::from("false;"),
            expectedBoolean: false,
        },
    ];

    for tt in tests.iter() {
        let l = lexer::New(tt.input.clone());
        let mut p = l.New();

        let program = p.ParseProgram();
        p.checkParserErrors();

        assert_eq!(
            1,
            program.Statements.len(),
            "program.Statements does not contain 1 statements. got={}",
            program.Statements.len()
        );

        let stmt = &program.Statements[0];

        if let ast::Statement::ExpressionStatement { Token, Expression } = stmt {
            if let ast::Expression::Boolean { Token, Value } = Expression {
                assert_eq!(Value, &tt.expectedBoolean);
            } else {
                panic!("Expression not Boolean. got={}", Expression)
            }
        } else {
            panic!("stmt not ast::Statement::ExpressionStatement. got={}", stmt);
        }
    }
}

fn testBooleanLiteral(exp: &ast::Expression, value: bool) -> bool {
    if let ast::Expression::Boolean { Token, Value } = exp {
        if Value != &value {
            println!("Value not {}. got={}", value, Value);
            return false;
        }
        if Token.Literal != format!("{}", value) {
            println!("Token.Literal not {}. got={}", value, Token.Literal);
            return false;
        }

        true
    } else {
        println!("exp not ast::Expression::Boolean. got={}", exp);
        false
    }
}

#[test]
fn TestIfExpression() {
    let input = String::from("if (x < y) {x}");

    let l = lexer::New(input);
    let mut p = l.New();
    let program = p.ParseProgram();
    p.checkParserErrors();

    assert_eq!(
        1,
        program.Statements.len(),
        "program.Statements does not contain 1 statements. got={}",
        program.Statements.len()
    );

    let stmt = &program.Statements[0];

    if let ast::Statement::ExpressionStatement { Token, Expression } = stmt {
        if let ast::Expression::IfExpression {
            Token,
            Condition,
            Consequence,
            Alternative,
        } = Expression
        {
            assert_eq!(
                true,
                testInfixExpression(
                    &Condition,
                    &ast::Expression::Identifier(ast::Identifier {
                        Token: Token {
                            Type: token::IDENT,
                            Literal: String::from("x")
                        },
                        Value: String::from("x")
                    }),
                    String::from("<"),
                    &ast::Expression::Identifier(ast::Identifier {
                        Token: Token {
                            Type: token::IDENT,
                            Literal: String::from("y")
                        },
                        Value: String::from("y")
                    })
                )
            );
            if let ast::Statement::BlockStatement { Token, Statements } = Consequence.as_ref() {
                assert_eq!(
                    1,
                    Statements.len(),
                    "Consequence is not 1 statements. got={}",
                    Statements.len()
                );
                let consequence = &Statements[0];
                if let ast::Statement::ExpressionStatement { Token, Expression } = consequence {
                    assert_eq!(true, testIdentifier(&Expression, String::from("x")));
                    if let ast::Statement::Nil = Alternative.as_ref() {
                    } else {
                        println!("Alternative is not Nil. got={}", Alternative);
                    }
                } else {
                    panic!(
                        "Statements[0] is not ast::Statement::ExpressionStatement. got={}",
                        Statements[0]
                    );
                }
            } else {
                panic!(
                    "Consequence is not ast::Statement::BlockStatement. got={}",
                    Consequence
                );
            }
        } else {
            panic!(
                "Expression is not ast::Expression::IfExpression. got={}",
                Expression
            );
        }
    } else {
        panic!(
            "program.Statements[0] is not ast::Statement::ExpressionStatement. got={}",
            program.Statements[0]
        );
    }
}

#[test]
fn TestIfElseExpression() {
    let input = String::from("if (x < y) {x} else {y}");

    let l = lexer::New(input);
    let mut p = l.New();
    let program = p.ParseProgram();
    p.checkParserErrors();

    assert_eq!(
        1,
        program.Statements.len(),
        "program.Statements does not contain 1 statements. got={}",
        program.Statements.len()
    );

    let stmt = &program.Statements[0];

    if let ast::Statement::ExpressionStatement { Token, Expression } = stmt {
        if let ast::Expression::IfExpression {
            Token,
            Condition,
            Consequence,
            Alternative,
        } = Expression
        {
            assert_eq!(
                true,
                testInfixExpression(
                    &Condition,
                    &ast::Expression::Identifier(ast::Identifier {
                        Token: Token {
                            Type: token::IDENT,
                            Literal: String::from("x")
                        },
                        Value: String::from("x")
                    }),
                    String::from("<"),
                    &ast::Expression::Identifier(ast::Identifier {
                        Token: Token {
                            Type: token::IDENT,
                            Literal: String::from("y")
                        },
                        Value: String::from("y")
                    })
                )
            );
            if let ast::Statement::BlockStatement { Token, Statements } = Consequence.as_ref() {
                assert_eq!(
                    1,
                    Statements.len(),
                    "Consequence is not 1 statements. got={}",
                    Statements.len()
                );
                let consequence = &Statements[0];
                if let ast::Statement::ExpressionStatement { Token, Expression } = consequence {
                    assert_eq!(true, testIdentifier(&Expression, String::from("x")));
                } else {
                    panic!(
                        "Statements[0] is not ast::Statement::ExpressionStatement. got={}",
                        Statements[0]
                    );
                }
            } else {
                panic!(
                    "Consequence is not ast::Statement::BlockStatement. got={}",
                    Consequence
                );
            }

            if let ast::Statement::BlockStatement { Token, Statements } = Alternative.as_ref() {
                assert_eq!(
                    1,
                    Statements.len(),
                    "Alternative is not 1 statements. got={}",
                    Statements.len()
                );
                let alternative = &Statements[0];
                if let ast::Statement::ExpressionStatement { Token, Expression } = alternative {
                    assert_eq!(true, testIdentifier(&Expression, String::from("y")));
                } else {
                    panic!(
                        "Statements[0] is not ast::Statement::ExpressionStatement. got={}",
                        Statements[0]
                    );
                }
            } else {
                panic!(
                    "Alternative is not ast::Statement::BlockStatement. got={}",
                    Alternative
                );
            }
        } else {
            panic!(
                "Expression is not ast::Expression::IfExpression. got={}",
                Expression
            );
        }
    } else {
        panic!(
            "program.Statements[0] is not ast::Statement::ExpressionStatement. got={}",
            program.Statements[0]
        );
    }
}

#[test]
fn TestFunctionLiteralParsing() {
    let input = String::from("fn(x,y) {x + y;}");
    let l = lexer::New(input);
    let mut p = l.New();
    let program = p.ParseProgram();
    p.checkParserErrors();

    assert_eq!(
        1,
        program.Statements.len(),
        "program.Statements does not contain 1 statements. got={}",
        program.Statements.len()
    );

    let stmt = &program.Statements[0];
    if let ast::Statement::ExpressionStatement { Token, Expression } = stmt {
        if let ast::Expression::FunctionLiteral {
            Token,
            Parameters,
            Body,
        } = Expression
        {
            assert_eq!(
                2,
                Parameters.len(),
                "function literal parameters wrong. want 2, got={}",
                Parameters.len()
            );
            assert_eq!(
                true,
                testLiteralExpression(
                    &Parameters[0],
                    &ast::Expression::Identifier(ast::Identifier {
                        Token: Token {
                            Type: token::IDENT,
                            Literal: String::from("x"),
                        },
                        Value: String::from("x")
                    })
                )
            );
            assert_eq!(
                true,
                testLiteralExpression(
                    &Parameters[1],
                    &ast::Expression::Identifier(ast::Identifier {
                        Token: Token {
                            Type: token::IDENT,
                            Literal: String::from("y"),
                        },
                        Value: String::from("y")
                    })
                )
            );

            if let ast::Statement::BlockStatement { Token, Statements } = Body.as_ref() {
                assert_eq!(
                    1,
                    Statements.len(),
                    "Statements has not 1 statements. got={}",
                    Statements.len()
                );
                let bodystmt = &Statements[0];
                if let ast::Statement::ExpressionStatement { Token, Expression } = bodystmt {
                    assert_eq!(
                        true,
                        testInfixExpression(
                            &Expression,
                            &ast::Expression::Identifier(ast::Identifier {
                                Token: Token {
                                    Type: token::IDENT,
                                    Literal: String::from("x"),
                                },
                                Value: String::from("x")
                            }),
                            String::from("+"),
                            &ast::Expression::Identifier(ast::Identifier {
                                Token: Token {
                                    Type: token::IDENT,
                                    Literal: String::from("y"),
                                },
                                Value: String::from("y")
                            })
                        )
                    );
                } else {
                    panic!(
                        "function body stmt is not ast::Statement::ExpressionStatement. got={}",
                        bodystmt
                    );
                }
            } else {
                panic!("Body is not ast::Statement::BlockStatement. got={}", Body);
            }
        } else {
            panic!(
                "Expression is not ast::Expression::FunctionLiteral. got={}",
                Expression
            );
        }
    } else {
        panic!(
            "program.Statements[0] is not ast::Statement::ExpressionStatement. got={}",
            program.Statements[0]
        );
    }
}

#[test]
fn TestFunctionParameterParsing() {
    struct tests_struct {
        input: String,
        expectedParams: Vec<String>,
    }

    let tests = vec![
        tests_struct {
            input: String::from("fn() {};"),
            expectedParams: vec![],
        },
        tests_struct {
            input: String::from("fn(x) {};"),
            expectedParams: vec![String::from("x")],
        },
        tests_struct {
            input: String::from("fn(x, y, z) {};"),
            expectedParams: vec![String::from("x"), String::from("y"), String::from("z")],
        },
    ];

    for tt in tests.iter() {
        let l = lexer::New(tt.input.clone());
        let mut p = l.New();
        let program = p.ParseProgram();
        p.checkParserErrors();

        let stmt = &program.Statements[0];
        if let ast::Statement::ExpressionStatement { Token, Expression } = stmt {
            if let ast::Expression::FunctionLiteral {
                Token,
                Parameters,
                Body,
            } = Expression
            {
                assert_eq!(
                    Parameters.len(),
                    tt.expectedParams.len(),
                    "length parameters wrong. want {}, got={}",
                    tt.expectedParams.len(),
                    Parameters.len()
                );
                for (i, ident) in tt.expectedParams.iter().enumerate() {
                    assert_eq!(
                        true,
                        testLiteralExpression(
                            &Parameters[i],
                            &ast::Expression::Identifier(ast::Identifier {
                                Token: Token {
                                    Type: token::IDENT,
                                    Literal: ident.clone()
                                },
                                Value: ident.clone()
                            })
                        )
                    );
                }
            } else {
                panic!(
                    "Expression is not ast::Expression::FunctionLiteral. got={}",
                    Expression
                );
            }
        } else {
            panic!(
                "program.Statements[0] is not ast::Statement::ExpressionStatement. got={}",
                program.Statements[0]
            );
        }
    }
}

#[test]
fn TestCallExpressionParsing() {
    let input = String::from("add(1, 2 * 3, 4 + 5);");
    let l = lexer::New(input);
    let mut p = l.New();
    let program = p.ParseProgram();
    p.checkParserErrors();

    assert_eq!(
        1,
        program.Statements.len(),
        "program.Statements does not contain 1 statements. got={}",
        program.Statements.len()
    );

    let stmt = &program.Statements[0];
    if let ast::Statement::ExpressionStatement { Token, Expression } = stmt {
        if let ast::Expression::CallExpression {
            Token,
            Function,
            Arguments,
        } = Expression
        {
            assert_eq!(true, testIdentifier(Function, String::from("add")));
            assert_eq!(
                3,
                Arguments.len(),
                "wrong length of arguments. got={}",
                Arguments.len()
            );
            assert_eq!(
                true,
                testLiteralExpression(
                    &Arguments[0],
                    &ast::Expression::IntergerLiteral {
                        Token: Token {
                            Type: token::INT,
                            Literal: String::from("1")
                        },
                        Value: 1
                    }
                )
            );
            assert_eq!(
                true,
                testInfixExpression(
                    &Arguments[1],
                    &ast::Expression::IntergerLiteral {
                        Token: Token {
                            Type: token::INT,
                            Literal: String::from("2")
                        },
                        Value: 2
                    },
                    String::from("*"),
                    &ast::Expression::IntergerLiteral {
                        Token: Token {
                            Type: token::INT,
                            Literal: String::from("3")
                        },
                        Value: 3
                    }
                )
            );
            assert_eq!(
                true,
                testInfixExpression(
                    &Arguments[2],
                    &ast::Expression::IntergerLiteral {
                        Token: Token {
                            Type: token::INT,
                            Literal: String::from("4")
                        },
                        Value: 4
                    },
                    String::from("+"),
                    &ast::Expression::IntergerLiteral {
                        Token: Token {
                            Type: token::INT,
                            Literal: String::from("5")
                        },
                        Value: 5
                    }
                )
            );
        } else {
            panic!(
                "Expression is not ast::Expression::CallExpression. got={}",
                Expression
            );
        }
    } else {
        panic!(
            "program.Statements[0] is not ast::Statement::ExpressionStatement. got={}",
            program.Statements[0]
        );
    }
}
