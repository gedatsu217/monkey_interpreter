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

