use regex::Match;

use crate::{ast::Node, lexer::{Token, TokenFamily, TokenKind}, parser};

#[test]
fn test_add() {
    let lhs = Token {
        value: "1.0".to_string(),
        family: TokenFamily::Value,
        kind: TokenKind::Number,
        line: 0,
        column: 0,
    };
    let op = Token {
        value: "+".to_string(),
        family: TokenFamily::Operator,
        kind: TokenKind::Add,
        line: 0,
        column: 1,
    };
    let rhs = Token {
        value: "1.0".to_string(),
        family: TokenFamily::Value,
        kind: TokenKind::Number,
        line: 0,
        column: 2,
    };
    
    let tokens = vec![lhs, op, rhs, Token {value: "\n".to_string(), family: TokenFamily::Operator, kind: TokenKind::Eof, line: 0, column: 3}];
    let ast = parser::expression::parse_program(&tokens);
    
    let ast = match ast {
        Ok(ast) => ast,
        Err(err) => {
            panic!("Error: {:?}", err);
        }
    };
    
    assert_eq!(ast, Node::Program(vec![Box::new(Node::BinaryOperation {
        lhs: Box::new(Node::Double(1.0)),
        op: TokenKind::Add,
        rhs: Box::new(Node::Double(1.0)),
    })]));
    
}