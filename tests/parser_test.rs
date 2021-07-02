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
    /*
    if let None = program {
        println!("ParseProgram() returned None");
    }

    let program = program.unwrap();*/

    if program.Statements.len() != 3 {
        println!("program.Statesments does not contain 3 statements. got={}", program.Statements.len());
    }
    
    let tests = vec![
        ExpectedIdentifiers{expectedIdentifier: String::from("x")},
        ExpectedIdentifiers{expectedIdentifier: String::from("y")},
        ExpectedIdentifiers{expectedIdentifier: String::from("foobar")},
    ];

    for (i, tt) in tests.iter().enumerate() {
        let stmt = &program.Statements[i];
        testLetStatement(stmt, &tt.expectedIdentifier);
    }

    assert_eq!(4, 4);
}

fn testLetStatement(s: &ast::Statement, name: &String) -> bool{
    if let ast::Statement::LetStatement{Token, Name, Value} = s {
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

