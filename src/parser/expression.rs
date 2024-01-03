use crate::parser::literal::parse_struct_init;

use super::super::*;
use super::debug::*;
use super::function::parse_tuple;
use super::keyword::parse_repeat_stmnt;
use super::literal::{parse_array_initializer, parse_digits};
use super::*;

pub fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    parse_logical(tokens, index)
}

pub fn parse_logical(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_relational(tokens, index)?;

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::LogicalAnd | TokenKind::LogicalOr => {
                *index += 1;
                let right = parse_relational(tokens, index)?;
                left = Node::LogicalExpression {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(left)
}

pub fn parse_relational(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_bin_op(tokens, index)?;

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Equals
            | TokenKind::NotEquals
            | TokenKind::LessThanEquals
            | TokenKind::GreaterThanEquals
            | TokenKind::LeftAngle
            | TokenKind::RightAngle => {
                consume_next_if_any(tokens, index, vec![TokenKind::Equals, TokenKind::NotEquals, TokenKind::LessThanEquals, TokenKind::GreaterThanEquals, TokenKind::LeftAngle, TokenKind::RightAngle ]);
                let right = parse_bin_op(tokens, index)?;
                left = Node::RelationalExpression {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(left)
}

pub fn parse_bin_op(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_term(tokens, index)?;

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Add | TokenKind::Subtract => {
                consume_next_if_any(tokens, index, vec![TokenKind::Add, TokenKind::Subtract]);
                let right = parse_term(tokens, index)?;
                left = Node::BinaryOperation {
                    op: token.kind,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(left)
}

pub fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_unary(tokens, index)?;

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Multiply | TokenKind::Divide => {
                consume_next_if_any(tokens, index, vec![TokenKind::Multiply, TokenKind::Divide]);
                let right = parse_unary(tokens, index)?;
                left = Node::BinaryOperation {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Ok(left)
}
pub fn parse_unary(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let op = current_token(tokens, index);
    match op.kind {
        TokenKind::Subtract | TokenKind::Not => {
            consume_next_if_any(tokens, index, vec![TokenKind::Subtract, TokenKind::Not]);
            let node = parse_operand(tokens, index)?;
            let node_type = if op.kind == TokenKind::Subtract {
                Node::NegOp
            } else {
                Node::NotOp
            };

            assert!(
                !(matches!(node, Node::NegOp(_)) || matches!(node, Node::NotOp(_))),
                "Double not operations are not allowed"
            );

            Ok(node_type(Box::new(node)))
        }
        _ => parse_operand(tokens, index),
    }
}
pub fn parse_operand(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let token = current_token(tokens, index);
    let mut node = match token.kind {
        TokenKind::Number => {
            consume(tokens,index,TokenKind::Number);
            parse_digits(token)
        },
        TokenKind::Bool => {
            consume(tokens,index,TokenKind::Bool);
            Ok(Node::Bool(token.value.parse::<bool>().unwrap()))
        },
        TokenKind::Identifier => {
            consume(tokens,index,TokenKind::Identifier);
            Ok(Node::Identifier(token.value.clone()))
        },
        TokenKind::String => {
            consume(tokens,index,TokenKind::String);
            Ok(Node::String(token.value.clone()))
        },
        TokenKind::New => {
            consume(tokens, index, TokenKind::New);
            consume_next_if_any(
                tokens,
                index,
                vec![TokenKind::OpenParenthesis, TokenKind::OpenCurlyBrace],
            );
            parse_struct_init(tokens, index, &token)
        }
        TokenKind::OpenBracket => {
            consume(tokens,index,TokenKind::OpenBracket);
            let init = parse_array_initializer(tokens, index)?;
            Ok(Node::Array {
                typename: ARRAY_TNAME.to_string(),
                elements: init.clone(),
                init_capacity: init.len(),
                mutable: true,
                elements_mutable: true,
            })
        }
        TokenKind::Repeat => parse_repeat_stmnt(token, index, tokens),
        _ => {
            let message = format!("
parse_operand error: unexpected token {:?}, 
            
Expected:
    number, boolean, identifier, string, 'new' expression,
    '[]' array literal, '.' dot operation, '()' call operation,
    or a returning repeat statement.", token);
            
            return Err(PrsErr {
                message: dbgmsg!(message.as_str()),
                token: token.clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: None,
            });
        }
    }?;
    
    // optionally, get chained operations that behave much like unary operations
    while let Some(op) = tokens.get(*index) {
        match op.kind {
            TokenKind::OpenParenthesis => {
                let tuple = parse_tuple(tokens, index)?;
                node = Node::BinaryOperation {
                    lhs: Box::new(node),
                    op: TokenKind::Call,
                    rhs: Box::new(tuple),
                };
            }
            TokenKind::Dot => {
                consume(tokens, index, TokenKind::Dot);
                let right = parse_operand(tokens, index)?;
                node = Node::BinaryOperation {
                    lhs: Box::new(node),
                    op: TokenKind::Dot,
                    rhs: Box::new(right),
                };
            }
            TokenKind::OpenBracket => {
                consume(tokens, index, TokenKind::OpenBracket);
                node = Node::BinaryOperation {
                    lhs: Box::new(node),
                    op: TokenKind::Subscript,
                    rhs: Box::new(parse_expression(tokens, index)?),
                }
            }
            _ => return Ok(node),
        }
    }

    Ok(node)
}
