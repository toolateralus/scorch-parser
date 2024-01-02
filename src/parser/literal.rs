use super::*;
use super::super::*;
use super::expression::parse_expression;
pub fn parse_digits(identifier: &Token) -> Result<Node, PrsErr> {
    let int = identifier.value.parse::<i32>();
    let float = identifier.value.parse::<f64>();

    Ok(int
        .map(Node::Int)
        .unwrap_or_else(|_| float.map(Node::Double).unwrap()))
}

pub fn parse_array_initializer(
    tokens: &Vec<Token>,
    index: &mut usize,
) -> Result<Vec<Box<Node>>, PrsErr> {
    let mut args = Vec::new();
    loop {
        let token = get_current(tokens, index);
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

        let cur = get_current(tokens, index).kind;

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

        let token = get_current(tokens, index);
        match token.kind {
            TokenKind::Newline => *index += 1,
            TokenKind::CloseCurlyBrace | TokenKind::CloseParenthesis => {
                *index += 1;
                break;
            }
            _ => {
                args.push(parse_expression(tokens, index)?);
                if get_current(tokens, index).kind == TokenKind::Comma {
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
