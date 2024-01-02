use crate::parser::literal::parse_struct_init;

use super::super::*;
use super::debug::*;
use super::function::parse_fn_call;
use super::keyword::parse_repeat_stmnt;
use super::literal::parse_array_access;
use super::literal::{parse_array_initializer, parse_digits};
use super::*;

pub fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_logical(tokens, index)?;

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::LogicalAnd | TokenKind::LogicalOr => {
                *index += 1;
                let right = parse_logical(tokens, index)?;
                left = Node::LogicalExpression {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
                };
            }
            TokenKind::Dot => {
                *index += 1;
                let right = parse_accessor(tokens, index)?;
                left = Node::BinaryOperation {
                    lhs: Box::new(left),
                    op: TokenKind::Dot,
                    rhs: Box::new(right),
                };
            }
            TokenKind::CloseParenthesis
            | TokenKind::CloseBracket
            | TokenKind::OpenCurlyBrace
            | TokenKind::CloseCurlyBrace
            | TokenKind::Pipe
            | TokenKind::Newline
            | TokenKind::Comma
            | TokenKind::Assignment
            | TokenKind::ColonEquals
            | TokenKind::Colon
            | TokenKind::Eof => break,
            _ => Err(PrsErr {
                message: dbgmsg!("expression err: unexpected token"),
                token: current_token(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: None,
            })?,
        }
    }

    Ok(Node::Expression(Box::new(left)))
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
                *index += 1;
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
                *index += 1;
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
                *index += 1;
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
            *index += 1;
            let node = parse_dot(tokens, index)?;
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
        _ => parse_dot(tokens, index),
    }
}

pub fn parse_dot(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let left = parse_accessor(tokens, index)?;
    let op = current_token(tokens, index);
    match op.kind {
        TokenKind::Dot => {
            *index += 1; // consume '.' operator.
            let rhs = parse_accessor(tokens, index)?;
            Ok(Node::BinaryOperation {
                lhs: Box::new(left),
                op: TokenKind::Dot,
                rhs: Box::new(rhs),
            })
        }
        _ => Ok(left),
    }
}

pub fn parse_accessor(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let left = parse_operand(tokens, index)?;
    let op = current_token(tokens, index);
    
    match op.kind {
        TokenKind::OpenParenthesis => {
            if let Node::Identifier(id) = &left {
                parse_fn_call(index, tokens, &id).expect("Expected function call node, got")
            } else {
                Err(PrsErr {
                    message: dbgmsg!("accessor err: Expected identifier"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                })
            }
        }
        TokenKind::OpenBracket => {
            if let Node::Identifier(id) = left {
                *index += 1; // move past [
                Ok(parse_array_access(index, tokens, &id)?)
            } else {
                Err(PrsErr {
                    message: dbgmsg!("accessor err: Expected identifier"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                })
            }
        }
        _ => Ok(left),
    }
}

pub fn parse_operand(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let identifier = tokens
        .get(*index)
        .expect("Unexpected end of tokens, {tokens}");
    *index += 1;

    match identifier.kind {
        // todo: make these all safer.
        TokenKind::Number => parse_digits(identifier),
        TokenKind::Bool => Ok(Node::Bool(identifier.value.parse::<bool>().unwrap())),
        
        TokenKind::Identifier => Ok(Node::Identifier(identifier.value.clone())),
        TokenKind::New => {
            let struct_name = current_token(tokens, index).clone();
            consume_next_if_type(tokens, index, TokenKind::Identifier);
            consume_next_if_any(tokens, index, vec![TokenKind::OpenParenthesis, TokenKind::OpenCurlyBrace]);
            parse_struct_init(tokens, index, &struct_name)
        }
        TokenKind::String => Ok(Node::String(identifier.value.clone())),
        TokenKind::OpenBracket => {
            let init = parse_array_initializer(tokens, index)?;
            Ok(Node::Array {
                typename: ARRAY_TNAME.to_string(),
                elements: init.clone(),
                init_capacity: init.len(),
                mutable: true,
                elements_mutable: true,
            })
        }
        TokenKind::OpenParenthesis => {
            let node = parse_expression(tokens, index)?;
            consume_next_if_type(tokens, index, TokenKind::CloseParenthesis);
            Ok(node)
        }
        
        TokenKind::Repeat => parse_repeat_stmnt(current_token(tokens, index), index, tokens),
        _ => Err(PrsErr {
            message: dbgmsg!("operand err: Unexpected token"),
            token: current_token(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
            inner_err: None,
        }),
    }
}


