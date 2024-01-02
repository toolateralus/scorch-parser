// declarations

use crate::{lexer::{Token, TokenFamily, TokenKind}, ast::Node};

use super::{*, expression::parse_expression};
use super::super::*;

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

        match parse_decl(token, index, tokens, mutable) {
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
            Err(inner_err) => panic!("type assoc decl block err: invalid declaration\ninner err:\n{:#?}", inner_err),
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
        
        match parse_decl(token, index, tokens, mutable) {
            Ok(node) => statements.push(Box::new(node)),
            Err(inner_err) => panic!("struct decl block err: invalid declaration\ninner err:\n{:#?}", inner_err),
        }

        token = get_current(tokens, index);

        if token.kind == TokenKind::Comma {
            *index += 1;
        }
    }
}

pub fn parse_decl(
    token: &Token,
    index: &mut usize,
    tokens: &Vec<Token>,
    mutable: bool,
) -> Result<Node, PrsErr> {
   
    let id = token.value.clone();
    
    let operator = get_current(tokens, index);
    
    match operator.kind {
        // varname := default;
        // declaring a variable with implicit type.
        TokenKind::ColonEquals => parse_implicit_decl(index, tokens, &id, mutable),
        // declaraing a variable with explicit type.
        TokenKind::Colon => parse_explicit_decl(index, tokens, token, id, mutable),
        // assigning a value to an already declared variable.
        TokenKind::Assignment => {
            *index += 1;
            let id = Node::Identifier(token.value.clone());
            let expression = parse_expression(tokens, index)?;
            consume_normal_expr_delimiter(tokens, index);
            Ok(Node::AssignStmnt {
                id: Box::new(id),
                expression: Box::new(expression),
            })
        }
        TokenKind::OpenBracket => {
            *index += 1; // discard [
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
					inner_err: None
                });
            };
            node
        }
        
        _ => {
            return Err(PrsErr{
                message: dbgmsg!("decl err: invalid operator"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
				inner_err: None
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
    *index += 1;
    
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
    // skip id token
    *index += 1;

    // varname :^ type = default;
    // todo: check for valid type / builtins
    let target_type_tkn = get_current(tokens, index);
    let target_type = target_type_tkn.value.clone();
    
    *index += 1;
    
    let token = get_current(tokens, index);
    if token.kind == TokenKind::OpenParenthesis {
        let return_type = target_type;
        let params = parse_parameters(tokens, index)?; //(x : int..)..
        let Some(val) = parse_fn_decl(
            &params,
            tokens,
            index,
            &id,
            return_type.to_string(),
            mutable,
        ) else {
            return Err(PrsErr{
                message: dbgmsg!("Expected function body"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: None
            });
        };
        
        let fn_def = match &val {
            Ok(fn_def) => fn_def,
            Err(_inner_err) => {
                return Err(PrsErr{
                    message: dbgmsg!("explicit decl err: Expected function body (INNER EXCEPTION HIDDEN DUE TO OWNERSHIP ISSUES)"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            }
        };
            
        // consume body end?        
        *index += 1;
            
        return Ok(fn_def.clone());
    }
    
    // varname : type^ = default;
    
    let token = get_current(tokens, index);

    // varname : type
    // uninitialized ((default for now))
    if token.kind == TokenKind::Newline {
        *index += 1;

        let default_value_expression = create_default_value_for_type(&target_type, mutable)?;

        return Ok(Node::DeclStmt {
            target_type,
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
        target_type,
        id,
        expression: Box::new(expression),
        mutable,
    })
}

pub fn parse_type_assoc_block(
    index: &mut usize,
    tokens: &Vec<Token>,
) -> Result<Node, PrsErr> {
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
    *index += 2; // consume 'struct && identifier'

    let id = identifier.value.clone();
    let mut token = get_current(tokens, index);

    if token.kind != TokenKind::Pipe {
        return Err(PrsErr{
            message: dbgmsg!("Expected pipe token after struct identifier"),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
			inner_err: None
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