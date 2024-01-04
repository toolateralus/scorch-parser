// declarations

use super::super::*;
use super::expression::{parse_operand};
use super::{debug::*};
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

        let mutable = if token.family == TokenFamily::Keyword && token.kind == TokenKind::Var {
            consume_newlines(index, tokens);
            true
        } else {
            false
        };

        if token.kind == TokenKind::CloseCurlyBrace {
            break;
        }

        match parse_decl_or_expr(index, tokens, mutable) {
            Ok(node) => {
                let is_valid = match node {
                    Node::FuncDeclStmnt { .. } => true,
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

        token = current_token(tokens, index);

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
        
        if token.kind == TokenKind::Pipe {
            break;
        }

        let mutable = if token.kind == TokenKind::Var {
            token = consume_newlines(index, tokens);
            true
        } else {
            false
        };
        
        match parse_decl_or_expr(index, tokens, mutable) {
            Ok(node) => statements.push(Box::new(node)),
            Err(inner_err) => panic!(
                "struct decl block err: invalid declaration\ninner err:\n{:#?}",
                inner_err
            ),
        }

        token = current_token(tokens, index);

        if token.kind == TokenKind::Comma {
            *index += 1;
        }
    }
}

// This is the main entry point for parsing declarations & expressions.
pub fn parse_decl_or_expr(
    index: &mut usize,
    tokens: &Vec<Token>,
    mutable: bool,
) -> Result<Node, PrsErr> {
    
    let identifier = match parse_expression(tokens, index) {
        Ok(node) => node,
        Err(inner_err) => {
            return Err(PrsErr {
                message: dbgmsg!("statement err: left side could not parse"),
                token: current_token(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: Some(Box::new(inner_err)),
            });
        }
    };
    
    let operator = consume_newlines(index, tokens);
    
    return match operator.kind {
        TokenKind::Colon => {
            consume(tokens, index, TokenKind::Colon);
            parse_explicit_decl(index, tokens, Box::new(identifier), mutable)
        }
        TokenKind::ColonEquals => {
            consume(tokens, index, TokenKind::ColonEquals);
            parse_implicit_decl(index, tokens, Box::new(identifier), mutable)
        }
        TokenKind::Assignment => {
            consume(tokens, index, TokenKind::Assignment);
            let expression = parse_expression(tokens, index)?;
            Ok(Node::AssignStmnt {
                id: Box::new(identifier),
                expression: Box::new(expression),
            })
        }
        _ => Ok(identifier),
    };
}

pub fn parse_implicit_decl(
    index: &mut usize,
    tokens: &Vec<Token>,
    id: Box<Node>,
    mutable: bool,
) -> Result<Node, PrsErr> {
    
    if current_token(tokens, index).kind == TokenKind::Newline {
        let _token = consume_newlines(index, tokens);
    }
    
    // implicit variable declaration
    let value = parse_expression(tokens, index)?;
    
    consume_delimiter(tokens, index);
    
    Ok(Node::DeclStmt {
        target_type: Box::new(Node::Identifier(String::from(DYNAMIC_TNAME))),
        target_id: id,
        expression: Some(Box::new(value)),
        mutable,
    })
}

pub fn parse_explicit_decl(
    index: &mut usize,
    tokens: &Vec<Token>,
    id: Box<Node>,
    mutable: bool,
) -> Result<Node, PrsErr> {
    
    let target_t = parse_operand(tokens, index)?;
    
    dbg!(target_t.clone());
    
    let token = current_token(tokens, index);
    
    // uninitialized ((default for now))
    if token.kind == TokenKind::Newline {
        consume(tokens, index, TokenKind::Newline);

        return Ok(Node::DeclStmt {
            target_type: Box::new(target_t),
            target_id: id,
            expression: Some(Box::new(Node::Identifier("none".to_string()))),
            mutable,
        });
    }
    
    consume(tokens, index, TokenKind::Assignment);
    
    // varname : type = ^default;

    let expression = parse_expression(tokens, index)?;
    consume_delimiter(tokens, index);
    Ok(Node::DeclStmt {
        target_type: Box::new(target_t),
        target_id: id,
        expression: Some(Box::new(expression)),
        mutable,
    })
}

pub fn parse_type_assoc_block(index: &mut usize, tokens: &Vec<Token>) -> Result<Node, PrsErr> {
    consume(tokens, index, TokenKind::Within);
    let typename = parse_operand(tokens, index)?;
    consume(tokens, index, TokenKind::Identifier);
    consume(tokens, index, TokenKind::OpenCurlyBrace);
    let mut statements = Vec::new();
    parse_type_assoc_decl_block(index, tokens, &mut statements);
    Ok(Node::TypeAssocBlockStmnt {
        typename: Box::new(typename),
        block: Box::new(Node::Block(statements)),
    })
}

pub fn parse_struct_decl(index: &mut usize, tokens: &Vec<Token>) -> Result<Node, PrsErr> {
    consume(tokens, index, TokenKind::Struct);

    let id = parse_operand(tokens, index);
    let token = current_token(tokens, index);

    if token.kind != TokenKind::Pipe {
        return Err(PrsErr {
            message: dbgmsg!("Expected pipe token after struct identifier"),
            token: current_token(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
            inner_err: None,
        });
    }
    
    let mut statements = Vec::new();
    
    _= consume_newlines(index, tokens);
    
    consume(tokens, index, TokenKind::Pipe);
    
    _= consume_newlines(index, tokens);
    
    parse_struct_decl_block(index, tokens, &mut statements);
    
    consume(tokens, index, TokenKind::Pipe);
    Ok(Node::StructDecl {
        id: Box::new(id?),
        block: Box::new(Node::Block(statements)),
    })
}
