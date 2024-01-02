use super::super::*;
use super::debug::{ErrType, PrsErr};
use super::declaration::{parse_decl_stmnt, parse_struct_decl, parse_type_assoc_block};
use super::expression::parse_expression;
use super::*;
use super::function::{parse_fn_decl, parse_parameters};
use super::function::{parse_fn_decl, parse_parameters};
// keywords

pub fn parse_return(
    index: &mut usize,
    second: &Token,
    tokens: &Vec<Token>,
) -> Result<Node, PrsErr> {
    *index += 1;
    // discard break
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

pub fn parse_repeat_stmnt(
    next: &Token,
    index: &mut usize,
    tokens: &Vec<Token>,
) -> Result<Node, PrsErr> {
    // style::
    // repeat i < 200 {...}
    if next.family == TokenFamily::Identifier {
        let id = next.value.clone();
        *index += 1; // skip repeat, leaev identifier in expression.
        let condition = parse_expression(tokens, index)?;
        let block = parse_block(tokens, index)?;
        let node = Node::RepeatStmnt {
            iterator_id: Some(id),
            condition: Some(Box::new(condition)),
            block: Box::new(block),
        };
        return Ok(node);
    }

    *index += 1; // skip repeat
                 // style::
                 // repeat {... }
    let block = parse_block(tokens, index)?;

    //*index += 1;

    Ok(Node::RepeatStmnt {
        iterator_id: Option::None,
        condition: Option::None,
        block: Box::new(block),
    })
}

pub fn parse_keyword_ops(
    keyword: &Token,
    index: &mut usize,
    next_token: &Token,
    tokens: &Vec<Token>,
) -> Result<Node, PrsErr> {
    match keyword.kind {
        TokenKind::Override => {
            consume_next_if_type(tokens, index, TokenKind::Override);
            consume_next_if_type(tokens, index, TokenKind::OpenBracket);
            
            let fam = current_token(tokens, index).family;
            
            if fam != TokenFamily::Operator {
                return Err(PrsErr {
                    message: dbgmsg!("Expected operator in operator overload function definition"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            }
            
            let op = current_token(tokens, index).clone();
            
            consume_next_if_type(tokens, index, TokenKind::Colon);
            
            consume_next_if_type(tokens, index, TokenKind::OpenParenthesis);
            
            let params = parse_parameters(tokens, index)?;
            
            if params.len() != 2 {
                return Err(PrsErr {
                    message: dbgmsg!("Expected two parameters in operator overload function definition"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            }
            
            let lhs_t = params[0].clone();
            
            let Node::ParamDeclNode { varname, typename } = lhs_t else {
                return Err(PrsErr {
                    message: dbgmsg!("Expected parameter declaration in operator overload function definition"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            };
            let (lhs_t, lhs_n) = (varname, typename);
            
            let Node::Identifier(lhs_typename) = lhs_t.as_ref() else {
                return Err(PrsErr {
                    message: dbgmsg!("Expected identifier in operator overload function definition"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            };
            
            let Node::Identifier(lhs_varname) = lhs_n.as_ref() else {
                return Err(PrsErr {
                    message: dbgmsg!("Expected identifier in operator overload function definition"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            };
            
            let rhs_t = params[1].clone();
            
            let Node::ParamDeclNode { varname, typename } = rhs_t else {
                return Err(PrsErr {
                    message: dbgmsg!("Expected parameter declaration in operator overload function definition"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            };
            
            let (rhs_t, rhs_n) = (varname, typename);
            
            let Node::Identifier(rhs_typename) = rhs_t.as_ref() else {
                return Err(PrsErr {
                    message: dbgmsg!("Expected identifier in operator overload function definition"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            };
            
            let Node::Identifier(rhs_varname) = rhs_n.as_ref() else {
                return Err(PrsErr {
                    message: dbgmsg!("Expected identifier in operator overload function definition"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            };
            
            let Some(Ok(func)) = parse_fn_decl(&params, tokens, index, &String::from("op_overload"), DYNAMIC_TNAME.to_string(), false) else {
                return Err(PrsErr {
                    message: dbgmsg!("Expected function declaration in operator overload function definition"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            };
            
            Ok(Node::OpOverrideDecl {
                op: op.kind,
                func:Box::new(func),
                lhs_tname: lhs_typename.clone(),
                lhs_varname: lhs_varname.clone(),
                rhs_tname: rhs_typename.clone(),
                rhs_varname: rhs_varname.clone(),
            })
        }
        TokenKind::Const => {
            consume_next_if_type(tokens, index, TokenKind::Const);
            consume_next_if_type(tokens, index, TokenKind::Identifier);
            parse_decl_stmnt(next_token, index, tokens, false)
        }
        TokenKind::Var => {
            consume_next_if_type(tokens, index, TokenKind::Var);
            consume_next_if_type(tokens, index, TokenKind::Identifier);
            parse_decl_stmnt(next_token, index, tokens, true)
        }
        TokenKind::Return => parse_return(index, next_token, tokens),
        TokenKind::Repeat => parse_repeat_stmnt(next_token, index, tokens),
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
        TokenKind::Struct => parse_struct_decl(index, next_token, tokens),
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

pub fn parse_if_else(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    *index += 1; // discard 'if'
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

    *index += 1; // skip open brace

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

        match cur.kind {
            TokenKind::OpenCurlyBrace | TokenKind::CloseParenthesis => {
                *index += 1; // skip open brace
            }
            _ => {
                // continue.
            }
        }

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
