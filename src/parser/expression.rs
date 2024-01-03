use std::thread::current;

use super::super::*;
use super::debug::*;
use super::declaration::parse_decl_or_expr;
use super::function::parse_tuple;
use super::keyword::parse_while_stmnt;
use super::keyword::parse_return;
use super::literal::{parse_array_initializer, parse_digits};
use super::*;

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
    _ = consume_newlines(index, tokens);
    consume(tokens, index, TokenKind::OpenCurlyBrace);
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
    
    match first.family {
        TokenFamily::Keyword => Some(parse_keyword_ops(first, index, tokens)),
        TokenFamily::Operator | TokenFamily::Value | TokenFamily::Identifier => {
            let result = parse_decl_or_expr(index, tokens, false);
            Some(result)
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
pub fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let left = parse_logical(tokens, index)?;
    while let Some(op) = tokens.get(*index) {
        match op.kind {
            _ => break,
        }
    }
    Ok(left)
}
pub fn parse_logical(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_relational(tokens, index)?;
    
    while let Some(op) = tokens.get(*index) {
        match op.kind {
            TokenKind::LogicalAnd | TokenKind::LogicalOr => {
                consume_next_if_any(
                    tokens,
                    index,
                    vec![TokenKind::LogicalAnd, TokenKind::LogicalOr],
                );
                let right = parse_relational(tokens, index)?;
                bin_op(&mut left, op, &right);
            }
            _ => break,
        }
    }
    Ok(left)
}
pub fn parse_relational(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_bin_op(tokens, index)?;
    
    while let Some(op) = tokens.get(*index) {
        match op.kind {
            TokenKind::Equals
            | TokenKind::NotEquals
            | TokenKind::LessThanEquals
            | TokenKind::GreaterThanEquals
            | TokenKind::LeftAngle
            | TokenKind::RightAngle => {
                *index += 1;
                let right = parse_bin_op(tokens, index)?;
                bin_op(&mut left, op, &right);
            }
            _ => break,
        }
    }
    Ok(left)
}
pub fn parse_bin_op(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_term(tokens, index)?;
    
    while let Some(op) = tokens.get(*index) {
        match op.kind {
            TokenKind::Add | TokenKind::Subtract => {
                consume_next_if_any(tokens, index, vec![TokenKind::Add, TokenKind::Subtract]);
                let right = parse_term(tokens, index)?;
                bin_op(&mut left, op, &right);
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
                bin_op(&mut left, token, &right);
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
pub fn parse_type_name(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr>{
    let token =current_token(tokens, index);
    consume(tokens, index, TokenKind::Identifier);
    return Ok(Node::Identifier(token.value.clone()));
}

pub fn parse_operand(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let token = current_token(tokens, index);
    let mut left = match token.kind {
        TokenKind::Number => {
            consume(tokens, index, TokenKind::Number);
            parse_digits(token)
        }
        TokenKind::Bool => {
            consume(tokens, index, TokenKind::Bool);
            Ok(Node::Bool(token.value.parse::<bool>().unwrap()))
        }
        TokenKind::Identifier => {
            consume(tokens, index, TokenKind::Identifier);
            Ok(Node::Identifier(token.value.clone()))
        }
        TokenKind::String => {
            consume(tokens, index, TokenKind::String);
            Ok(Node::String(token.value.clone()))
        }  
        TokenKind::OpenParenthesis => {
            consume(tokens, index, TokenKind::OpenParenthesis);
            let expression = parse_tuple(tokens, index);
            consume(tokens, index, TokenKind::CloseParenthesis);
            // for parenthesized expressions
            if let Ok(Node::Tuple(elements)) = &expression {
                if elements.len() == 1 {
                    return Ok(Node::Expression(elements[0].clone()));
                }
            }
            expression
        }
        TokenKind::OpenBracket => {
            consume(tokens, index, TokenKind::OpenBracket);
            let init = parse_array_initializer(tokens, index)?;
            consume(tokens, index, TokenKind::CloseBracket);
            Ok(Node::Array {
                typename: Box::new(Node::Identifier(String::from(ARRAY_TNAME))),
                elements: init.clone(),
                init_capacity: init.len(),
                
                // todo: figure out a model for mutability.
                mutable: true,
                elements_mutable: true,
            })
        }
        // todo: move these keywords or use some kind of map for these so their behavour can change independently and
        // not clog up this file.
        TokenKind::New => {
            consume(tokens, index, TokenKind::New);
            let typename = parse_type_name(tokens, index)?;
            consume(tokens, index, TokenKind::OpenParenthesis);
            let args = parse_tuple(tokens,index)?;
            consume(tokens, index, TokenKind::CloseParenthesis);
            Ok(Node::BinaryOperation { 
                lhs: Box::new(typename), 
                op: TokenKind::New,
                rhs: Box::new(args)
            })
        }
        TokenKind::Return => parse_return(index, tokens), 
        TokenKind::While => parse_while_stmnt(index, tokens),
        _ => {
            panic!("{:#?}", token);
        }
    }?;
    
    // optionally, get chained operations
    while let Some(op) = tokens.get(*index) {
        match op.kind {
            // function calls
            TokenKind::OpenParenthesis => {
                consume(tokens, index, TokenKind::OpenParenthesis);
                // tuple declaration (i : string)
                if lookahead(tokens, index, 2).kind == TokenKind::Colon {
                    break;
                }
                let tuple = parse_tuple(tokens, index)?;
                consume(tokens, index, TokenKind::CloseParenthesis);
                bin_op(&mut left, op, &tuple)
            }
            TokenKind::Dot => {
                consume(tokens, index, TokenKind::Dot);
                let right = parse_operand(tokens, index)?;
                bin_op(&mut left, op, &right)
            }
            TokenKind::OpenBracket => {
                consume(tokens, index, TokenKind::OpenBracket);
                bin_op(&mut left, op, &parse_expression(tokens, index)?);
                consume(tokens, index, TokenKind::CloseBracket);
            }
            _ => return Ok(left),
        }
    }
    
    Ok(left)
}

pub fn bin_op<'a>(left: &'a mut Node, op: &'a Token, right: &'a Node) {
    *left = Node::BinaryOperation {
        lhs: Box::new(left.clone()),
        op: op.kind,
        rhs: Box::new(right.clone()),
    };
}
