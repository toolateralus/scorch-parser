use crate::{
    ast::Node,
    dbgmsg,
    lexer::{Token, TokenFamily, TokenKind},
};

use super::{
    debug::*,
    expression::{parse_expression, parse_operand},
    get_current, parse_block,
};
// function helpers
pub fn parse_parameters(tokens: &Vec<Token>, index: &mut usize) -> Result<Vec<Node>, PrsErr> {
    let mut params = Vec::new();

    loop {
        let mut token = get_current(tokens, index);

        if token.kind == TokenKind::CloseParenthesis {
            *index += 1;
            break;
        }

        // parsing varname
        // ^varname: Typename
        if token.family != TokenFamily::Identifier {
            Err(PrsErr {
                message: dbgmsg!("parameter err: expected identifier"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: None,
            })?;
        }

        let varname = parse_operand(tokens, index)?;

        token = get_current(tokens, index);

        //parsing colon
        // varname^: Typename
        match token.kind {
            TokenKind::ColonEquals => {
                return Err(PrsErr{
                    message: dbgmsg!("implicit typed / default value parameters are not yet implemented. coming soon B)"),
                    token: get_current(tokens, index).clone(),
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
                    token: get_current(tokens, index).clone(),
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
        if get_current(tokens, index).kind == TokenKind::Comma {
            *index += 1;
        }

        let param_decl_node = Node::ParamDeclNode {
            varname: Box::new(varname),
            typename: Box::new(typename),
        };

        params.push(param_decl_node);
    }
    Ok(params)
}
pub fn parse_arguments(tokens: &Vec<Token>, index: &mut usize) -> Result<Vec<Node>, PrsErr> {
    *index += 1; // discard open_paren

    let mut args = Vec::new();

    loop {
        let token = get_current(tokens, index);
        // paramless.
        if token.kind == TokenKind::CloseParenthesis {
            *index += 1;
            break;
        }
        // accumulate parameter expressions
        let arg = parse_expression(tokens, index)?;

        // skip commas
        if get_current(tokens, index).kind == TokenKind::Comma {
            *index += 1;
        }

        args.push(arg);
    }
    Ok(args)
}
pub fn parse_fn_decl(
    params: &Vec<Node>,
    tokens: &Vec<Token>,
    index: &mut usize,
    id: &String,
    return_type: String,
    mutable: bool,
) -> Option<Result<Node, PrsErr>> {
    let token = get_current(tokens, index);
    let kind = token.kind;
    if kind == TokenKind::OpenCurlyBrace {
        let body = match parse_block(tokens, index) {
            Ok(body) => body,
            Err(inner_err) => {
                return Some(Err(PrsErr {
                    message: dbgmsg!("fn decl err: invalid block"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                    inner_err: Some(Box::new(inner_err)),
                }));
            }
        };

        let node = Node::FnDeclStmnt {
            id: id.clone(),
            body: Box::new(body),
            params: params.clone(),
            return_type,
            mutable,
        };
        return Some(Ok(node));
    }
    None
}
pub fn parse_fn_call(
    index: &mut usize,
    tokens: &Vec<Token>,
    token: &String,
) -> Option<Result<Node, PrsErr>> {
    let arguments = match parse_arguments(tokens, index) {
        Ok(arguments) => arguments,
        Err(inner_err) => {
            return Some(Err(PrsErr {
                message: dbgmsg!("Expected arguments"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
                inner_err: Some(Box::new(inner_err)),
            }));
        }
    };

    let node = Node::FunctionCall {
        id: token.clone(),
        arguments: Some(arguments),
    };
    Some(Ok(node))
}
