use crate::parser::expression::parse_type_name;

use super::super::*;
use super::debug::{ErrType, PrsErr};
use super::declaration::{parse_decl_or_expr, parse_struct_decl, parse_type_assoc_block};
use super::expression::{parse_block, parse_expression, parse_operand};
use super::*;
use super::function::parse_parameters;
// keywords

pub fn parse_return(index: &mut usize, tokens: &Vec<Token>) -> Result<Node, PrsErr> {
    consume(tokens, index, TokenKind::Return);
    let second = current_token(tokens, index);
    match second.kind {
        TokenKind::Newline => Ok(Node::ReturnStmnt(None)),
        _ if second.kind != TokenKind::CloseCurlyBrace => {
            let value = parse_expression(tokens, index)?;
            Ok(Node::ReturnStmnt(Some(Box::new(value))))
        }
        _ => Err(PrsErr {
            message: dbgmsg!("break err: Unexpected token"),
            token: current_token(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
            inner_err: None,
        }),
    }
}

pub fn parse_while_stmnt(index: &mut usize, tokens: &Vec<Token>) -> Result<Node, PrsErr> {
    consume(tokens, index, TokenKind::While);
    
    let next = current_token(tokens, index);
    if next.family == TokenFamily::Identifier {
        let condition = parse_expression(tokens, index)?;
        let block = parse_block(tokens, index)?;
        let node = Node::WhileStmnt {
            condition: Some(Box::new(condition)),
            block: Box::new(block),
        };
        return Ok(node);
    }
    
    let block = parse_block(tokens, index)?;

    Ok(Node::WhileStmnt {
        condition: None,
        block: Box::new(block),
    })
}

pub fn parse_keyword_ops(
    keyword: &Token,
    index: &mut usize,
    tokens: &Vec<Token>,
) -> Result<Node, PrsErr> {
    match keyword.kind {
        TokenKind::Const => {
            consume(tokens, index, TokenKind::Const);
            parse_decl_or_expr(index, tokens, false)
        }
        TokenKind::Var => {
            consume(tokens, index, TokenKind::Var);
            parse_decl_or_expr(index, tokens, true)
        }
        TokenKind::Fn => parse_function(index, tokens),
        TokenKind::Return => parse_return(index, tokens),
        TokenKind::While => parse_while_stmnt(index, tokens),
        TokenKind::If => Ok(parse_if_else(tokens, index)?),
        TokenKind::Within => parse_type_assoc_block(index, tokens),
        TokenKind::Else => {
            return Err(PrsErr {
                message: dbgmsg!("Unexpected else statement.. else must follow an if."),
                token: current_token(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: None,
            });
        }
        TokenKind::Struct => parse_struct_decl(index, tokens),
        _ => {
            return Err(PrsErr {
                message: dbgmsg!("unexpected token"),
                token: current_token(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: None,
            })
        }
    }
}

fn parse_function(index: &mut usize, tokens: &Vec<Token>) -> Result<Node, PrsErr> {
    dbg!("starting function declaration.");
    
    consume(tokens, index, TokenKind::Fn);
    let id = parse_operand(tokens, index)?;
    
    consume(tokens, index, TokenKind::Colon);
    let type_id = parse_type_name(tokens, index)?;
    
    consume(tokens, index, TokenKind::OpenParenthesis);
    let params = parse_parameters(tokens, index)?;
    
    consume(tokens, index, TokenKind::CloseParenthesis);
    
    // parse block handles its own delimiters.
    let block = parse_block(tokens, index)?;
    
    dbg!("end function declaration.");
    return Ok(Node::FuncDeclStmnt {
        id: Box::new(id),
        params : params,
        return_t: Box::new(type_id),
        body: Box::new(block),
        mutable: false, // todo :: allow mutable function declarations?
    });
    
    
}

pub fn parse_if_else(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    consume(tokens, index, TokenKind::If);
    let if_condition = parse_expression(tokens, index)?;
    
    if current_token(tokens, index).kind != TokenKind::OpenCurlyBrace {
        return Err(PrsErr {
            message: dbgmsg!("Expected open curly brace after if condition"),
            token: current_token(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
            inner_err: None,
        });
    }
    
    let if_block = parse_block(tokens, index)?;
    
    let else_or_end = consume_newlines(index, tokens);
    
    // if, no else.
    if else_or_end.kind == TokenKind::Else {
        let else_node = match parse_else(tokens, index) {
            Ok(else_node) => else_node,
            Err(inner_err) => {
                return Err(PrsErr {
                    message: dbgmsg!("Expected else statement"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: Some(Box::new(inner_err)),
                });
            }
        };

        return Ok(Node::IfStmnt {
            condition: Box::new(if_condition),
            block: Box::new(if_block),
            else_stmnt: Option::Some(Box::new(else_node)),
        });
    } else {
        // an 'if' with no 'else.
        return Ok(Node::IfStmnt {
            condition: Box::new(if_condition),
            block: Box::new(if_block),
            else_stmnt: Option::None,
        });
    }
}

pub fn parse_else(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    *index += 1; // discard 'else'

    let _ = consume_newlines(index, tokens);

    // if else with no comparison -> if ... {} else {}
    if current_token(tokens, index).kind == TokenKind::OpenCurlyBrace {
        let else_block = parse_block(tokens, index)?;

        // Check for another else after this block
        if current_token(tokens, index).kind == TokenKind::Else {
            let nested_else = parse_else(tokens, index)?;
            return Ok(Node::ElseStmnt {
                condition: Option::None,
                block: Box::new(else_block),
                else_stmnt: Option::Some(Box::new(nested_else)),
            });
        } else {
            return Ok(Node::ElseStmnt {
                condition: Option::None,
                block: Box::new(else_block),
                else_stmnt: Option::None,
            });
        }
    }
    // if else with comparison -> if ... {} else ... {}
    else {
        let else_condition = parse_expression(tokens, index)?;
        let cur = current_token(tokens, index);

        let else_block = parse_block(tokens, index)?;

        if current_token(tokens, index).kind == TokenKind::Else {
            let nested_else = parse_else(tokens, index)?;
            return Ok(Node::ElseStmnt {
                condition: Option::Some(Box::new(else_condition)),
                block: Box::new(else_block),
                else_stmnt: Option::Some(Box::new(nested_else)),
            });
        } else {
            return Ok(Node::ElseStmnt {
                condition: Option::Some(Box::new(else_condition)),
                block: Box::new(else_block),
                else_stmnt: Option::None,
            });
        }
    }
}
