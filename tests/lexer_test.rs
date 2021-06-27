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
        //println!("{}", tok.Type);
        assert_eq!(tok.Type, tt.expectedType);
        assert_eq!(tok.Literal, tt.expectedLiteral);
    }
}