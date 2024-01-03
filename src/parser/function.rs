use std::rc::Rc;

use crate::{
    ast::Node,
    dbgmsg,
    lexer::{Token, TokenFamily, TokenKind},
};

use super::{
    consume, current_token,
    debug::*,
    expression::{parse_expression, parse_operand},
    parse_block,
};
// function helpers
pub fn parse_parameters(tokens: &Vec<Token>, index: &mut usize) -> Result<Vec<Node>, PrsErr> {
    let mut params = Vec::new();

    loop {
        let mut token = current_token(tokens, index);

        if token.kind == TokenKind::CloseParenthesis {
            *index += 1;
            break;
        }

        // parsing varname
        // ^varname: Typename
        if token.family != TokenFamily::Identifier {
            Err(PrsErr {
                message: dbgmsg!("parameter err: expected identifier"),
                token: current_token(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: None,
            })?;
        }

        let varname = parse_operand(tokens, index)?;

        token = current_token(tokens, index);

        //parsing colon
        // varname^: Typename
        match token.kind {
            TokenKind::ColonEquals => {
                return Err(PrsErr{
                    message: dbgmsg!("implicit typed / default value parameters are not yet implemented. coming soon B)"),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
					inner_err: None
                });
            }
            TokenKind::Colon => {
                // got our valid case.
                *index += 1;
            }
            _ => {
                return Err(PrsErr {
                    message: dbgmsg!(
                        "Expected colon token after variable name in parameter declaration"
                    ),
                    token: current_token(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: None,
                });
            }
        }

        // parsing type
        // varname: ^Typename
        let typename = parse_operand(tokens, index)?;

        // consume comma if there is one.
        if current_token(tokens, index).kind == TokenKind::Comma {
            *index += 1;
        }
        
        let param_decl_node = Node::ParamDecl {
            varname: Box::new(varname),
            typename: Box::new(typename),
        };

        params.push(param_decl_node);
    }
    Ok(params)
}
pub fn parse_tuple(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    consume(tokens, index, TokenKind::OpenParenthesis);

    let mut args = Vec::new();

    loop {
        let token = current_token(tokens, index);
        // empty tuple
        if token.kind == TokenKind::CloseParenthesis {
            *index += 1;
            break;
        }
        // accumulate parameter expressions
        let arg = parse_expression(tokens, index)?;

        // skip commas
        if current_token(tokens, index).kind == TokenKind::Comma {
            *index += 1;
        }

        args.push(Box::new(arg));
    }

    Ok(Node::Tuple(Rc::new(args)))
}

// great name
pub fn parse_fn_block_ret_decl_stmnt_node(
    params: &Vec<Node>,
    tokens: &Vec<Token>,
    index: &mut usize,
    id: &String,
    return_type: String,
    mutable: bool,
) -> Option<Result<Node, PrsErr>> {
    let block = parse_block(tokens, index);
    let Ok(block) = block else {
        return None;
    };
    let node = Node::FuncDeclStmnt {
        id: id.clone(),
        body: Box::new(block),
        params: params.clone(),
        return_type,
        mutable,
    };
    return Some(Ok(node));
}
