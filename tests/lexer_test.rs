extern crate monkey_interpreter;
use monkey_interpreter::lexer;
use monkey_interpreter::token;

#[test]
fn TestNextToken() {
    let input = String::from("=+(){},;");
    struct TokenTest {
        expectedType: token::TokenType,
        expectedLiteral: String,
    }

    let tests = vec![
        TokenTest{expectedType: token::ASSIGN, expectedLiteral: String::from("=")},
        TokenTest{expectedType: token::PLUS, expectedLiteral: String::from("+")},
        TokenTest{expectedType: token::LPAREN, expectedLiteral: String::from("(")},
        TokenTest{expectedType: token::RPAREN, expectedLiteral: String::from(")")},
        TokenTest{expectedType: token::LBRACE, expectedLiteral: String::from("{")},
        TokenTest{expectedType: token::RBRACE, expectedLiteral: String::from("}")},
        TokenTest{expectedType: token::COMMA, expectedLiteral: String::from(",")},
        TokenTest{expectedType: token::SEMICOLON, expectedLiteral: String::from(";")},
        TokenTest{expectedType: token::EOF, expectedLiteral: String::from("")},
    ];

    let mut l = lexer::New(input);

    for tt in tests.iter(){
        let tok = l.NextToken();
        assert_eq!(tok.Type, tt.expectedType);
        assert_eq!(tok.Literal, tt.expectedLiteral);
    }

    let input = String::from("\
    let five = 5;
    let ten = 10;

    let add = fn(x,y) {
        x + y;
    };

    let result = add(five, ten);
    ");

    let tests = vec![
        TokenTest{expectedType: token::LET, expectedLiteral: String::from("let")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("five")},
        TokenTest{expectedType: token::ASSIGN, expectedLiteral: String::from("=")},
        TokenTest{expectedType: token::INT, expectedLiteral: String::from("5")},
        TokenTest{expectedType: token::SEMICOLON, expectedLiteral: String::from(";")},
        TokenTest{expectedType: token::LET, expectedLiteral: String::from("let")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("ten")},
        TokenTest{expectedType: token::ASSIGN, expectedLiteral: String::from("=")},
        TokenTest{expectedType: token::INT, expectedLiteral: String::from("10")},
        TokenTest{expectedType: token::SEMICOLON, expectedLiteral: String::from(";")},
        TokenTest{expectedType: token::LET, expectedLiteral: String::from("let")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("add")},
        TokenTest{expectedType: token::ASSIGN, expectedLiteral: String::from("=")},
        TokenTest{expectedType: token::FUNCTION, expectedLiteral: String::from("fn")},
        TokenTest{expectedType: token::LPAREN, expectedLiteral: String::from("(")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("x")},
        TokenTest{expectedType: token::COMMA, expectedLiteral: String::from(",")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("y")},
        TokenTest{expectedType: token::RPAREN, expectedLiteral: String::from(")")},
        TokenTest{expectedType: token::LBRACE, expectedLiteral: String::from("{")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("x")},
        TokenTest{expectedType: token::PLUS, expectedLiteral: String::from("+")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("y")},
        TokenTest{expectedType: token::SEMICOLON, expectedLiteral: String::from(";")},
        TokenTest{expectedType: token::RBRACE, expectedLiteral: String::from("}")},
        TokenTest{expectedType: token::SEMICOLON, expectedLiteral: String::from(";")},
        TokenTest{expectedType: token::LET, expectedLiteral: String::from("let")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("result")},
        TokenTest{expectedType: token::ASSIGN, expectedLiteral: String::from("=")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("add")},
        TokenTest{expectedType: token::LPAREN, expectedLiteral: String::from("(")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("five")},
        TokenTest{expectedType: token::COMMA, expectedLiteral: String::from(",")},
        TokenTest{expectedType: token::IDENT, expectedLiteral: String::from("ten")},
        TokenTest{expectedType: token::RPAREN, expectedLiteral: String::from(")")},
        TokenTest{expectedType: token::SEMICOLON, expectedLiteral: String::from(";")},
        TokenTest{expectedType: token::EOF, expectedLiteral: String::from("")},
    ];

    let mut l = lexer::New(input);
    let mut x =0;
    for tt in tests.iter(){
        let tok = l.NextToken();
        println!("{}", x);
        x += 1;
        assert_eq!(tok.Type, tt.expectedType);
        assert_eq!(tok.Literal, tt.expectedLiteral);
    }
}