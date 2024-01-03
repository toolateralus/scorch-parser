use super::*;

use super::debug::PrsErr;
use super::expression::parse_expression;
pub fn parse_digits(identifier: &Token) -> Result<Node, PrsErr> {
    let int = identifier.value.parse::<i32>();
    let float = identifier.value.parse::<f64>();

    Ok(int
        .map(Node::Int)
        .unwrap_or_else(|_| float.map(Node::Double).unwrap()))
}

// arrays
pub fn new_array(
    typename: String,
    init_capacity: usize,
    elements: Vec<Box<Node>>,
    mutable: bool,
    elements_mutable: bool,
) -> Node {
    Node::Array {
        typename,
        init_capacity,
        elements,
        mutable,
        elements_mutable, // todo: how do we want to qualify this?
    }
}

pub fn parse_array_initializer(
    tokens: &Vec<Token>,
    index: &mut usize,
) -> Result<Vec<Box<Node>>, PrsErr> {
    let mut args = Vec::new();
    loop {
        let token = current_token(tokens, index);
        // paramless.
        if token.kind == TokenKind::CloseBracket {
            *index += 1;
            break;
        }

        if token.kind == TokenKind::Newline {
            *index += 1;
        }

        // accumulate parameter expressions
        let arg = parse_expression(tokens, index)?;

        let cur = current_token(tokens, index).kind;

        // skip commas & newlines
        if cur == TokenKind::Comma || cur == TokenKind::Newline {
            *index += 1;
        }
        args.push(Box::new(arg));
    }
    Ok(args)
}

pub fn parse_struct_init(
    tokens: &Vec<Token>,
    index: &mut usize,
    identifier: &Token,
) -> Result<Node, PrsErr> {
    let mut args = Vec::new();

    loop {
        if *index >= tokens.len() {
            break;
        }

        let token = current_token(tokens, index);
        match token.kind {
            TokenKind::Newline => *index += 1,
            TokenKind::CloseCurlyBrace | TokenKind::CloseParenthesis => {
                *index += 1;
                break;
            }
            _ => {
                args.push(parse_expression(tokens, index)?);

                if current_token(tokens, index).kind == TokenKind::Comma {
                    *index += 1;
                }
            }
        }
    }

    return Ok(Node::StructInit {
        id: identifier.value.clone(),
        args,
    });
}