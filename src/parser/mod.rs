pub mod debug;
pub mod declaration;
pub mod expression;
pub mod function;
pub mod keyword;
pub mod literal;

use crate::ast::{ARRAY_TNAME, BOOL_TNAME, DOUBLE_TNAME, DYNAMIC_TNAME, INT_TNAME, STRING_TNAME};
use rand::Rng;

use self::{
    debug::{ErrType, PrsErr},
    declaration::parse_decl_stmnt,
    expression::parse_expression,
    keyword::parse_keyword_ops,
    literal::new_array,
};

use super::*;
use super::{
    ast::Node,
    lexer::{Token, TokenFamily, TokenKind},
};

pub fn parse_program(tokens: &Vec<Token>) -> Result<Node, PrsErr> {
    let mut index = 0;
    let mut statements = Vec::new();
    while index < tokens.len() {
        let token = consume_newlines(&mut index, tokens);
        if token.kind == TokenKind::Eof {
            break;
        }
        let statement = parse_statement(tokens, &mut index);

        let Some(statement) = statement else {
            break; // end of input, undetected before this call.
        };

        match statement {
            Ok(node) => statements.push(Box::new(node)),
            Err(inner_err) => {
                if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                    break; // ignore newlines.
                }
                return Err(PrsErr {
                    message: dbgmsg!("program err: invalid statement"),
                    token: current_token(tokens, &mut index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index,
                    inner_err: Some(Box::new(inner_err)),
                });
            }
        }
    }
    Ok(Node::Program(statements))
}
pub fn parse_block(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    *index += 1;
    let mut statements = Vec::new();
    while *index < tokens.len() {
        let token = consume_newlines(index, tokens);
        if token.kind == TokenKind::CloseCurlyBrace {
            *index += 1;
            break;
        }
        let statement = parse_statement(tokens, index);
        let Some(statement) = statement else {
            break; // end of input, undetected before this call.
        };
        match statement {
            Ok(node) => statements.push(Box::new(node)),
            Err(inner_err) => {
                if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                    break; // ignore newlines.
                }
                return Err(PrsErr {
                    message: dbgmsg!("block err: invalid statement"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: Some(Box::new(inner_err)),
                });
            }
        }
    }
    Ok(Node::Block(statements))
}
pub fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Option<Result<Node, PrsErr>> {
    if *index >= tokens.len() {
        return Some(Err(PrsErr {
            message: dbgmsg!("Unexpected end of tokens"),
            token: current_token(tokens, index).clone(),
            type_: ErrType::UnexpectedEof,
            index: *index,
            inner_err: None,
        }));
    }

    let first = consume_newlines(index, tokens);

    if *index + 1 >= tokens.len() {
        return None;
    }

    let second = tokens.get(*index + 1).unwrap();

    match first.family {
        TokenFamily::Keyword => Some(parse_keyword_ops(first, index, second, tokens)),

        TokenFamily::Operator | TokenFamily::Value | TokenFamily::Identifier => {
            let left = match parse_expression(tokens, index) {
                Ok(node) => node,
                Err(inner_err) => {
                    return Some(Err(PrsErr {
                        message: dbgmsg!("statement err: left side could not parse"),
                        token: current_token(tokens, index).clone(),
                        type_: ErrType::UnexpectedToken,
                        index: *index,
                        inner_err: Some(Box::new(inner_err)),
                    }));
                }
            };
            return match current_token(tokens, index).kind {
                TokenKind::ColonEquals | TokenKind::Colon => {
                    let decl = parse_decl_stmnt(first, index, tokens, false);
                    Some(decl)
                }
                TokenKind::Assignment => {
                    consume_next_if_type(tokens, index, TokenKind::Assignment);
                    let expression = match parse_expression(tokens, index) {
                        Ok(node) => node,
                        Err(inner_err) => {
                            return Some(Err(PrsErr {
                                message: dbgmsg!("statement err: right side could not parse"),
                                token: current_token(tokens, index).clone(),
                                type_: ErrType::UnexpectedToken,
                                index: *index,
                                inner_err: Some(Box::new(inner_err)),
                            }));
                        }
                    };
                    consume_delimiter(tokens, index);
                    Some(Ok(Node::AssignStmnt {
                        id: Box::new(left),
                        expression: Box::new(expression),
                    }))
                }
                _ => Some(Ok(left)),
            };
        }
        _ => Some(Err(PrsErr {
            message: dbgmsg!("statement err: unexpected token"),
            token: current_token(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
            inner_err: None,
        })),
    }
}

pub fn default_of_t(target_type: &String, mutable: bool) -> Result<Node, PrsErr> {
    let default_value_expression = match target_type.as_str() {
        DOUBLE_TNAME => Node::Expression(Box::new(Node::Double(0.0))),
        STRING_TNAME => Node::Expression(Box::new(Node::String(String::from("")))),
        BOOL_TNAME => Node::Expression(Box::new(Node::Bool(false))),
        ARRAY_TNAME => {
            let elements = Vec::new();
            let init_capacity = elements.len();
            let typename = String::from(DYNAMIC_TNAME);
            let elements_mutable = mutable;
            let arr = new_array(typename, init_capacity, elements, mutable, elements_mutable);
            Node::Expression(Box::new(arr))
        }
        INT_TNAME => Node::Expression(Box::new(Node::Int(0))),
        _ => Node::Expression(Box::new(Node::Undefined())),
    };
    Ok(default_value_expression)
}
pub fn current_token<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> &'a Token {
    if let Some(token) = tokens.get(*index) {
        return token;
    } else {
        panic!("Unexpected end of tokens");
    }
}
pub fn consume_newlines<'a>(index: &mut usize, tokens: &'a Vec<Token>) -> &'a Token {
    let mut current = current_token(tokens, index);
    while current.kind == TokenKind::Newline {
        *index += 1;
        current = current_token(tokens, index);
    }
    return current;
}
pub fn consume_delimiter(tokens: &Vec<Token>, index: &mut usize) {
    let current = current_token(tokens, index).kind;
    match current {
        TokenKind::OpenCurlyBrace | TokenKind::Comma => {
            dbg!(current);
            panic!("expected newline or ) token");
        }
        TokenKind::Newline => {
            *index += 1;
        }
        TokenKind::CloseParenthesis => {
            *index += 1;
        }
        _ => {
            // continue
        }
    }
}
pub fn generate_random_function_name() -> String {
    let mut rng = rand::thread_rng();
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let name: String = (0..8)
        .map(|_| letters[rng.gen_range(0..letters.len())])
        .collect();
    name
}
pub fn consume_next_if_type(tokens: &Vec<Token>, index: &mut usize, expected: TokenKind) {
    let current = current_token(tokens, index);
    if current.kind != expected {
        panic!("Expected {:?}, got {:?}", expected, current.kind);
    }
    *index += 1;
}
pub fn consume_next_if_any(tokens: &Vec<Token>, index: &mut usize, vec: Vec<TokenKind>) {
    let current = current_token(tokens, index);
    if !vec.contains(&current.kind) {
        panic!("Expected {:?}, got {:?}", vec, current.kind);
    }
    *index += 1;
}
