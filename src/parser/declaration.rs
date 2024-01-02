// declarations

use super::super::*;
use super::{
    debug::*,
    function::{parse_fn_call, parse_fn_decl, parse_parameters},
};
use super::{expression::parse_expression, *};
use crate::{
    ast::Node,
    lexer::{Token, TokenFamily, TokenKind},
};

pub fn parse_type_assoc_decl_block(
    index: &mut usize,
    tokens: &Vec<Token>,
    statements: &mut Vec<Box<Node>>,
) {
    while *index < tokens.len() {
        let mut token = consume_newlines(index, tokens);
        *index += 1;

        let mutable = if token.family == TokenFamily::Keyword && token.kind == TokenKind::Var {
            consume_newlines(index, tokens);
            true
        } else {
            false
        };

        if token.kind == TokenKind::CloseCurlyBrace {
            break;
        }

        match parse_decl_stmnt(token, index, tokens, mutable) {
            Ok(node) => {
                let is_valid = match node {
                    Node::FnDeclStmnt { .. } => true,
                    _ => false,
                };

                if !is_valid {
                    panic!("Expected function declaration statement in associated block, got {:?}, \n\n from : {:?}", node, statements);
                }

                statements.push(Box::new(node))
            }
            Err(inner_err) => panic!(
                "type assoc decl block err: invalid declaration\ninner err:\n{:#?}",
                inner_err
            ),
        }

        token = get_current(tokens, index);

        if token.kind == TokenKind::Comma || token.kind == TokenKind::Newline {
            *index += 1;
        }
    }
}

pub fn parse_struct_decl_block(
    index: &mut usize,
    tokens: &Vec<Token>,
    statements: &mut Vec<Box<Node>>,
) {
    while *index < tokens.len() {
        let mut token = consume_newlines(index, tokens);
        *index += 1;

        if token.kind == TokenKind::Pipe {
            break;
        }

        let mutable = if token.kind == TokenKind::Var {
            token = consume_newlines(index, tokens);
            true
        } else {
            false
        };

        match parse_decl_stmnt(token, index, tokens, mutable) {
            Ok(node) => statements.push(Box::new(node)),
            Err(inner_err) => panic!(
                "struct decl block err: invalid declaration\ninner err:\n{:#?}",
                inner_err
            ),
        }

        token = get_current(tokens, index);

        if token.kind == TokenKind::Comma {
            *index += 1;
        }
    }
}

pub fn parse_decl_stmnt(
    token: &Token,
    index: &mut usize,
    tokens: &Vec<Token>,
    mutable: bool,
) -> Result<Node, PrsErr> {
    let id = token.value.clone();

    let operator = get_current(tokens, index);

    match operator.kind {
        TokenKind::ColonEquals => parse_implicit_decl(index, tokens, &id, mutable),
        TokenKind::Colon => parse_explicit_decl(index, tokens, token, id, mutable),
        TokenKind::Assignment => {
            consume_next_if_type(tokens, index, TokenKind::Assignment);
            let id = Node::Identifier(token.value.clone());
            let expression = parse_expression(tokens, index)?;
            consume_normal_expr_delimiter(tokens, index);
            Ok(Node::AssignStmnt {
                id: Box::new(id),
                expression: Box::new(expression),
            })
        }
        TokenKind::OpenBracket => {
            consume_next_if_type(tokens, index, TokenKind::OpenBracket);
            let access = parse_array_access(index, tokens, token.value.as_str());
            access
        }

        // function call
        TokenKind::OpenParenthesis => {
            let Some(node) = parse_fn_call(index, tokens, &token.value.clone()) else {
                return Err(PrsErr {
                    message: dbgmsg!("decl err: Expected function call"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            };
            node
        }

        _ => {
            return Err(PrsErr {
                message: dbgmsg!("decl err: invalid operator"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: None,
            });
        }
    }
}

pub fn parse_implicit_decl(
    index: &mut usize,
    tokens: &Vec<Token>,
    id: &String,
    mutable: bool,
) -> Result<Node, PrsErr> {
    consume_next_if_type(tokens, index, TokenKind::ColonEquals);

    if get_current(tokens, index).kind == TokenKind::Newline {
        let _token = consume_newlines(index, tokens);
    }

    // implicit variable declaration
    let value = parse_expression(tokens, index)?;

    consume_normal_expr_delimiter(tokens, index);

    Ok(Node::DeclStmt {
        target_type: String::from("dynamic"),
        id: id.clone(),
        expression: Box::new(value),
        mutable,
    })
}

pub fn parse_explicit_decl(
    index: &mut usize,
    tokens: &Vec<Token>,
    _token: &Token,
    id: String,
    mutable: bool,
) -> Result<Node, PrsErr> {
    consume_next_if_type(tokens, index, TokenKind::Colon);

    let type_token = get_current(tokens, index);
    let target_t = type_token.value.clone();

    consume_next_if_type(tokens, index, TokenKind::Identifier);

    let token = get_current(tokens, index);

    if token.kind == TokenKind::OpenParenthesis {
        consume_next_if_type(tokens, index, TokenKind::OpenParenthesis);

        let return_type = target_t.to_string();
        let params = parse_parameters(tokens, index)?;

        let Some(Ok(val)) = parse_fn_decl(&params, tokens, index, &id, return_type, mutable) else {
            return Err(PrsErr {
                message: dbgmsg!("decl err: Expected function declaration"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: None,
            });
        };

        return Ok(val.clone());
    }

    // varname : type^ = default;

    let token = get_current(tokens, index);

    // varname : type
    // uninitialized ((default for now))
    if token.kind == TokenKind::Newline {
        *index += 1;

        let default_value_expression = create_default_value_for_type(&target_t, mutable)?;

        return Ok(Node::DeclStmt {
            target_type: target_t,
            id,
            expression: Box::new(default_value_expression),
            mutable,
        });
    }

    *index += 1;

    // varname : type = ^default;

    let expression = parse_expression(tokens, index)?;
    consume_normal_expr_delimiter(tokens, index);
    Ok(Node::DeclStmt {
        target_type: target_t,
        id,
        expression: Box::new(expression),
        mutable,
    })
}

pub fn parse_type_assoc_block(index: &mut usize, tokens: &Vec<Token>) -> Result<Node, PrsErr> {
    consume_next_if_type(tokens, index, TokenKind::Within);
    let typename = get_current(tokens, index);
    consume_next_if_type(tokens, index, TokenKind::Identifier);
    consume_next_if_type(tokens, index, TokenKind::OpenCurlyBrace);
    let mut statements = Vec::new();
    parse_type_assoc_decl_block(index, tokens, &mut statements);
    Ok(Node::TypeAssocBlock {
        typename: typename.value.clone(),
        block: Box::new(Node::Block(statements)),
    })
}

pub fn parse_struct_decl(
    index: &mut usize,
    identifier: &Token,
    tokens: &Vec<Token>,
) -> Result<Node, PrsErr> {
    consume_next_if_type(tokens, index, TokenKind::Struct);
    consume_next_if_type(tokens, index, TokenKind::Identifier);

    let id = identifier.value.clone();
    let mut token = get_current(tokens, index);

    if token.kind != TokenKind::Pipe {
        return Err(PrsErr {
            message: dbgmsg!("Expected pipe token after struct identifier"),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
            inner_err: None,
        });
    }

    let mut statements = Vec::new();

    token = consume_newlines(index, tokens);

    if token.kind == TokenKind::Pipe {
        *index += 1;
    }

    parse_struct_decl_block(index, tokens, &mut statements);

    Ok(Node::StructDecl {
        id,
        block: Box::new(Node::Block(statements)),
    })
}
