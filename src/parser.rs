use rand::Rng;

use crate::ast::{BOOL_TNAME, DOUBLE_TNAME, STRING_TNAME, ARRAY_TNAME, INT_TNAME};

use super::{
    ast::Node,
    lexer::{Token, TokenFamily, TokenKind},
};

#[derive(Debug)]
pub enum ErrType {
    UnexpectedToken,
    UnexpectedEof,
}
#[derive(Debug)]
pub struct PrsErr {
    pub message: String,
    pub token: Token,
    pub type_: ErrType,
    pub index: usize,
}

// function helpers
pub fn parse_parameters(tokens: &Vec<Token>, index: &mut usize) -> Result<Vec<Node>, PrsErr> {
    *index += 1; // discard open_paren

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
            Err(PrsErr{
                message: String::from("Expected identifier"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
            })?;
        }

        let varname = parse_operand(tokens, index)?;

        token = get_current(tokens, index);

        //parsing colon
        // varname^: Typename
        match token.kind {
            TokenKind::ColonEquals => {
                return Err(PrsErr{
                    message: String::from("implicit typed / default value parameters are not yet implemented. coming soon B)"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                });
            }
            TokenKind::Colon => {
                // got our valid case.
                *index += 1;
            }
            _ => {
                return Err(PrsErr{
                    message: String::from("Expected colon token after variable name in parameter declaration"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
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
        let body = parse_block(tokens, index);

        let Ok(body) = body else {
            return Some(Err(PrsErr {
                message: String::from("Expected function body"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
            }));
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
pub fn create_default_value_for_type(target_type: &String, mutable: bool) -> Result<Node, PrsErr> {
    let default_value_expression = match target_type.as_str() {
        DOUBLE_TNAME => Node::Expression(Box::new(Node::Double(0.0))),
        STRING_TNAME => Node::Expression(Box::new(Node::String(String::from("")))),
        BOOL_TNAME => Node::Expression(Box::new(Node::Bool(false))),
        ARRAY_TNAME => {
            let elements = Vec::new();
            let init_capacity = elements.len();
            let typename = String::from("dynamic");
            let elements_mutable = mutable;
            let arr = new_array(typename, init_capacity, elements, mutable, elements_mutable);
            Node::Expression(Box::new(arr))
        }
        INT_TNAME => Node::Expression(Box::new(Node::Int(0))),
        _ => Node::Expression(Box::new(Node::Undefined())),
    };
    Ok(default_value_expression)
}
fn parse_fn_call(
    index: &mut usize,
    tokens: &Vec<Token>,
    token: &String,
) -> Option<Result<Node, PrsErr>> {
    let arguments = parse_arguments(tokens, index);

    let Ok(arguments) = arguments else {
        return Some(Err(PrsErr {
            message: String::from("Expected arguments"),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
        }));
    };

    let node = Node::FunctionCall {
        id: token.clone(),
        arguments: Some(arguments),
    };
    Some(Ok(node))
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
pub fn parse_array_access(
    index: &mut usize,
    tokens: &Vec<Token>,
    id: &str,
) -> Result<Node, PrsErr> {
    let accessor = parse_expression(tokens, index);

    let Ok(accessor) = accessor else {
        return Err(PrsErr {
            message: String::from(
                "invalid expression in array subscript accessor, ie [...this expression..]",
            ),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
        });
    };

    let mut token = consume_newlines(index, tokens);

    if token.kind == TokenKind::CloseBracket {
        *index += 1; // move past ]
        token = get_current(tokens, index);
    }

    let mut node = Node::ArrayAccessExpr {
        id: id.to_string(),
        index_expr: Box::new(accessor),
        expression: None,
        assignment: false,
    };

    if token.kind != TokenKind::Assignment {
        return Ok(node);
    }

    match token.kind {
        TokenKind::Assignment => {
            *index += 1;
            if let Node::ArrayAccessExpr {
                id,
                index_expr,
                expression: _,
                assignment: _,
            } = node
            {
                let expression = parse_expression(tokens, index);

                let Ok(expression) = expression else {
                    return Err(PrsErr {
                        message: String::from("invalid expression in array subscript accessor, ie [...this expression..]"),
                        token: get_current(tokens, index).clone(),
                        type_: ErrType::UnexpectedToken,
                        index : *index,
                    });
                };

                node = Node::ArrayAccessExpr {
                    id,
                    index_expr,
                    expression: Option::Some(Box::new(expression)),
                    assignment: true,
                };
            }
            Ok(node)
        }
        _ => Err(PrsErr {
            message: String::from("Expected assignment operator"),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
        }),
    }
}

pub fn get_current<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> &'a Token {
    if let Some(token) = tokens.get(*index) {
        return token;
    } else {
        panic!("Unexpected end of tokens");
    }
}
pub fn consume_newlines<'a>(index: &mut usize, tokens: &'a Vec<Token>) -> &'a Token {
    let mut current = get_current(tokens, index);
    while current.kind == TokenKind::Newline {
        *index += 1;
        current = get_current(tokens, index);
    }
    return current;
}
pub fn consume_normal_expr_delimiter(tokens: &Vec<Token>, index: &mut usize) {
    let current = get_current(tokens, index).kind;
    match current {
        TokenKind::OpenCurlyBrace | TokenKind::Comma => {
            dbg!(current);
            panic!("expected newline or ) token");
        }
        TokenKind::Newline => {
            *index += 1;
        }
        TokenKind::CloseParenthesis => {
            *index += 1;
        }
        _ => {
            // continue
        }
    }
}

// keywords
fn parse_repeat_stmnt(
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
fn parse_if_else(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    *index += 1; // discard 'if'
    let if_condition = parse_expression(tokens, index)?;

    if get_current(tokens, index).kind != TokenKind::OpenCurlyBrace {
        return Err(PrsErr{
            message: String::from("Expected open curly brace after if condition"),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
        });
    }

    *index += 1; // skip open brace

    let if_block = parse_block(tokens, index)?;

    let else_or_end = consume_newlines(index, tokens);

    // if, no else.
    if else_or_end.kind == TokenKind::Else {
        let else_node = parse_else(tokens, index);

        let Ok(else_node) = else_node else {
            return Err(PrsErr {
                message: String::from("Expected else statement"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
            });
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
fn parse_else(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    *index += 1; // discard 'else'

    let _ = consume_newlines(index, tokens);

    // if else with no comparison -> if ... {} else {}
    if get_current(tokens, index).kind == TokenKind::OpenCurlyBrace {
        let else_block = parse_block(tokens, index)?;

        // Check for another else after this block
        if get_current(tokens, index).kind == TokenKind::Else {
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
        let cur = get_current(tokens, index);

        match cur.kind {
            TokenKind::OpenCurlyBrace | TokenKind::CloseParenthesis => {
                *index += 1; // skip open brace
            }
            _ => {
                // continue.
            }
        }

        let else_block = parse_block(tokens, index)?;

        if get_current(tokens, index).kind == TokenKind::Else {
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

// declarations
fn parse_decl(
    token: &Token,
    index: &mut usize,
    tokens: &Vec<Token>,
    mutable: bool,
) -> Result<Node, PrsErr> {
    // varname : type = default;
    let id = token.value.clone();

    *index += 1;

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
                    message: String::from("Expected function call"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                });
            };
            node
        }

        _ => {
            return Err(PrsErr{
                message: String::from("Unexpected token"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
            });
        }
    }
}
fn parse_implicit_decl(
    index: &mut usize,
    tokens: &Vec<Token>,
    id: &String,
    mutable: bool,
) -> Result<Node, PrsErr> {
    *index += 1;

    if let Some(Ok(value)) = parse_function_decl_stmnt(tokens, index, id, mutable) {
        return Ok(value);
    }

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
fn parse_function_decl_stmnt(
    tokens: &Vec<Token>,
    index: &mut usize,
    id: &String,
    mutable: bool,
) -> Option<Result<Node, PrsErr>> {
    if get_current(tokens, index).kind == TokenKind::OpenCurlyBrace {
        let bdy = parse_block(tokens, index);

        let Ok(body) = bdy else {
            return Some(bdy);
        };

        //dbg!(&body);
        let node = Node::FnDeclStmnt {
            id: id.clone(),
            body: Box::new(body),
            params: Vec::new(),
            return_type: String::from("dynamic"),
            mutable,
        };
        return Some(Ok(node));
    }
    if get_current(tokens, index).kind == TokenKind::OpenParenthesis {
        let mut temp_index = *index + 2;
        if get_current(tokens, &mut temp_index).kind == TokenKind::Colon {
            let params = parse_parameters(tokens, index);

            let Ok(params) = params else {
                return Some(Err(PrsErr {
                    message: String::from("Expected function parameters"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                }));
            };

            let body = parse_block(tokens, index);

            let Ok(body) = body else {
                return Some(Err(PrsErr {
                    message: String::from("Expected function body"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                }));
            };

            let node = Node::FnDeclStmnt {
                id: id.clone(),
                body: Box::new(body),
                params,
                return_type: String::from("dynamic"),
                mutable,
            };
            return Some(Ok(node));
        } else {
            return Some(Err(PrsErr {
                message: String::from("Expected colon token after function parameters"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
            }));
        }
    } else {
        return None;
    }
}
fn parse_explicit_decl(
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

    if target_type == "fn" {
        *index += 1;
        let params = parse_parameters(tokens, index)?;

        if get_current(tokens, index).kind == TokenKind::Arrow {
            *index += 1;

            if get_current(tokens, index).kind != TokenKind::Identifier {
                return Err(PrsErr{
                    message: String::from("Expected type identifier"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                });
            }

            let cur = get_current(tokens, index);

            if cur.kind != TokenKind::Identifier {
                return Err(PrsErr{
                    message: String::from("Expected type identifier"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                });
            }

            let return_type = cur.value.clone();

            *index += 1;

            let Some(val) = parse_fn_decl(
                &params,
                tokens,
                index,
                &id,
                return_type.to_string(),
                mutable,
            ) else {
                return Err(PrsErr{
                    message: String::from("Expected function body"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                });
            };
            
            let Ok(fn_def) = &val else {
                return Err(PrsErr{
                    message: String::from("Expected function body"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                });
            };

            *index += 1;

            return Ok(fn_def.clone());
        }
    }

    *index += 1;

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

pub fn generate_random_function_name() -> String {
    let mut rng = rand::thread_rng();
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let name: String = (0..8)
        .map(|_| letters[rng.gen_range(0..letters.len())])
        .collect();
    name
}

fn consume_next_if_type(tokens: &Vec<Token>, index: &mut usize, expected: TokenKind) {
    let current = get_current(tokens, index);
    if current.kind != expected {
        panic!("Expected {:?}, got {:?}", expected, current.kind);
    }
    *index += 1;
}

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
            Err(_) => {
                if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                    break; // ignore newlines.
                }
                return Err(PrsErr{
                    message: String::from("Expected statement node"),
                    token: get_current(tokens, &mut index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index,
                });
            }
        }
    }
    Ok(Node::Program(statements))
}
fn parse_block(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    *index += 1;
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
            Err(_) => {
                if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
                    break; // ignore newlines.
                }
                return Err(PrsErr{
                    message: String::from("Expected statement node"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                });
            }
        }
    }
    Ok(Node::Block(statements))
}
fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Option<Result<Node, PrsErr>> {
    if *index >= tokens.len() {
        return Some(Err(PrsErr {
            message: String::from("Unexpected end of tokens"),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedEof,
            index: *index,
        }));
    }

    let first = consume_newlines(index, tokens);

    if *index + 1 >= tokens.len() {
        return None;
        // newline, eof.
        // this is old but should never get called, if it does we have some work to do on it.
    }

    let second = tokens.get(*index + 1).unwrap();

    match first.family {
        TokenFamily::Keyword => Some(parse_keyword_ops(first, index, second, tokens)),
        TokenFamily::Identifier => match second.kind {
            TokenKind::ColonEquals | TokenKind::Colon | TokenKind::Assignment => {
                let decl = parse_decl(first, index, tokens, false);
                Some(decl)
            }
            TokenKind::DubColon => Some(parse_type_assoc_block(first, index, tokens)),
            _ => Some(parse_expression(tokens, index)),
        },
        TokenFamily::Operator | TokenFamily::Value => Some(parse_expression(tokens, index)),
        _ => {
            Some(Err(PrsErr{
                message: String::from("Expected statement node"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
            }))
        }
    }
}

fn parse_type_assoc_block(
    typename: &Token,
    index: &mut usize,
    tokens: &Vec<Token>,
) -> Result<Node, PrsErr> {
    *index += 1; // move past Typename
    consume_next_if_type(tokens, index, TokenKind::DubColon);
    consume_next_if_type(tokens, index, TokenKind::OpenCurlyBrace);
    let mut statements = Vec::new();
    parse_type_assoc_decl_block(index, tokens, &mut statements);
    Ok(Node::TypeAssocBlock {
        typename: typename.value.clone(),
        block: Box::new(Node::Block(statements)),
    })
}
fn parse_keyword_ops(
    keyword: &Token,
    index: &mut usize,
    next_token: &Token,
    tokens: &Vec<Token>,
) -> Result<Node, PrsErr> {
    match keyword.kind {
        TokenKind::Const => parse_const(index, next_token, tokens, keyword),
        TokenKind::Var => parse_var(index, next_token, tokens, keyword),
        TokenKind::Break => parse_break(index, next_token, tokens),
        TokenKind::Repeat => parse_repeat_stmnt(next_token, index, tokens),
        TokenKind::If => Ok(parse_if_else(tokens, index)?),
        TokenKind::Else => {
            return Err(PrsErr{
                message: String::from("Unexpected else statement.. else must follow an if."),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
            });
        }
        TokenKind::Struct => parse_struct_decl(index, next_token, tokens),
        _ => {
            return Err(PrsErr{
                message: String::from("unexpected token"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
            })
        }
    }
}
fn parse_struct_decl(
    index: &mut usize,
    identifier: &Token,
    tokens: &Vec<Token>,
) -> Result<Node, PrsErr> {
    *index += 2; // consume 'struct && identifier'

    let id = identifier.value.clone();
    let mut token = get_current(tokens, index);

    if token.kind != TokenKind::Pipe {
        return Err(PrsErr{
            message: String::from("Expected pipe token after struct identifier"),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
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
fn parse_type_assoc_decl_block(
    index: &mut usize,
    tokens: &Vec<Token>,
    statements: &mut Vec<Box<Node>>,
) {
    while *index < tokens.len() {
        let mut token = consume_newlines(index, tokens);

        let mutable = if token.family == TokenFamily::Keyword && token.kind == TokenKind::Var {
            *index += 1;
            consume_newlines(index, tokens);
            true
        } else {
            false
        };

        if token.kind == TokenKind::CloseCurlyBrace {
            *index += 1;
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
            Err(_) => panic!("Expected statement node"),
        }

        token = get_current(tokens, index);

        if token.kind == TokenKind::Comma || token.kind == TokenKind::Newline {
            *index += 1;
        }
    }
}
fn parse_struct_decl_block(
    index: &mut usize,
    tokens: &Vec<Token>,
    statements: &mut Vec<Box<Node>>,
) {
    while *index < tokens.len() {
        let mut token = consume_newlines(index, tokens);

        let mutable = if token.family == TokenFamily::Keyword && token.kind == TokenKind::Var {
            *index += 1;
            consume_newlines(index, tokens);
            true
        } else {
            false
        };

        if token.kind == TokenKind::Pipe {
            *index += 1;
            break;
        }

        match parse_decl(token, index, tokens, mutable) {
            Ok(node) => statements.push(Box::new(node)),
            Err(_) => panic!("Expected statement node"),
        }

        token = get_current(tokens, index);

        if token.kind == TokenKind::Comma {
            *index += 1;
        }
    }
}
fn parse_var(
    index: &mut usize,
    second: &Token,
    tokens: &Vec<Token>,
    first: &Token,
) -> Result<Node, PrsErr> {
    // consume 'var'
    *index += 1;
    parse_decl(second, index, tokens, true).map_err(|_| {
        PrsErr{
            message: String::from("Expected declaration statement"),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
        }
    })
}

fn parse_break(index: &mut usize, second: &Token, tokens: &Vec<Token>) -> Result<Node, PrsErr> {
    *index += 1;
    // discard break
    match second.kind {
        TokenKind::Newline => Ok(Node::BreakStmnt(None)),
        _ if second.kind != TokenKind::CloseCurlyBrace => {
            let value = parse_expression(tokens, index)?;
            Ok(Node::BreakStmnt(Some(Box::new(value))))
        }
        _ => Err(PrsErr{
            message: String::from("unexpected token in break statement"),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
        }),
    }
}
fn parse_const(
    index: &mut usize,
    second: &Token,
    tokens: &Vec<Token>,
    first: &Token,
) -> Result<Node, PrsErr> {
    // consume 'const'
    *index += 1;
    let varname = second;
    match parse_decl(varname, index, tokens, false) {
        Ok(node) => Ok(node),
        Err(_) => {
            Err(PrsErr{
                message: String::from("Expected declaration statement"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
            })
        }
    }
}
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_logical(tokens, index)?;

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::LogicalAnd | TokenKind::LogicalOr => {
                *index += 1;
                let right = parse_logical(tokens, index)?;
                left = Node::LogicalExpression {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
                };
            }
            TokenKind::Dot => {
                *index += 1;
                let right = parse_accessor(tokens, index)?;
                left = Node::BinaryOperation {
                    lhs: Box::new(left),
                    op: TokenKind::Dot,
                    rhs: Box::new(right),
                };
            }
            TokenKind::CloseParenthesis
            | TokenKind::CloseBracket
            | TokenKind::OpenCurlyBrace
            | TokenKind::CloseCurlyBrace
            | TokenKind::Pipe
            | TokenKind::Newline
            | TokenKind::Comma
            | TokenKind::Eof => break,
            _ => Err(PrsErr{
                message: String::from("Unexpected token"),
                token: get_current(tokens, index).clone(),
                type_: ErrType::UnexpectedToken,
                index: *index,
            })?,
        }
    }

    Ok(Node::Expression(Box::new(left)))
}
fn parse_logical(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_relational(tokens, index)?;
    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::LogicalAnd | TokenKind::LogicalOr => {
                *index += 1;
                let right = parse_relational(tokens, index)?;
                left = Node::LogicalExpression {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(left)
}
fn parse_relational(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_bin_op(tokens, index)?;

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Equals
            | TokenKind::NotEquals
            | TokenKind::LessThanEquals
            | TokenKind::GreaterThanEquals
            | TokenKind::LeftAngle
            | TokenKind::RightAngle => {
                *index += 1;
                let right = parse_bin_op(tokens, index)?;
                left = Node::RelationalExpression {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(left)
}
fn parse_bin_op(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_term(tokens, index)?;
    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Add | TokenKind::Subtract => {
                *index += 1;
                let right = parse_term(tokens, index)?;
                left = Node::BinaryOperation {
                    op: token.kind,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(left)
}
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let mut left = parse_unary(tokens, index)?;

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Multiply | TokenKind::Divide => {
                *index += 1;
                let right = parse_unary(tokens, index)?;
                left = Node::BinaryOperation {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Ok(left)
}
fn parse_unary(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let op = get_current(tokens, index);

    match op.kind {
        TokenKind::Subtract | TokenKind::Not => {
            *index += 1;
            let node = parse_dot(tokens, index)?;
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
        _ => parse_dot(tokens, index),
    }
}
fn parse_dot(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let left = parse_accessor(tokens, index)?;
    let op = get_current(tokens, index);
    match op.kind {
        TokenKind::Dot => {
            *index += 1; // consume '.' operator.
            let rhs = parse_accessor(tokens, index)?;
            Ok(Node::BinaryOperation {
                lhs: Box::new(left),
                op: TokenKind::Dot,
                rhs: Box::new(rhs),
            })
        }
        _ => Ok(left),
    }
}
fn parse_accessor(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let left = parse_operand(tokens, index)?;
    let op = get_current(tokens, index);

    match op.kind {
        TokenKind::OpenParenthesis => {
            if let Node::Identifier(id) = &left {
                parse_fn_call(index, tokens, &id).expect("Expected function call node, got")
            } else {
                Err(PrsErr {
                    message: String::from("Expected identifier"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                })
            }
        }
        TokenKind::OpenBracket => {
            if let Node::Identifier(id) = left {
                *index += 1; // move past [
                Ok(parse_array_access(index, tokens, &id)?)
            } else {
                Err(PrsErr {
                    message: String::from("Expected identifier"),
                    token: get_current(tokens, index).clone(),
                    type_: ErrType::UnexpectedToken,
                    index: *index,
                })
            }
        }
        _ => Ok(left),
    }
}
fn parse_operand(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, PrsErr> {
    let identifier = tokens
        .get(*index)
        .expect("Unexpected end of tokens, {tokens}");
    *index += 1;

    match identifier.kind {
        // todo: make these all safer.
        TokenKind::Number => parse_digits(identifier),
        TokenKind::Bool => Ok(Node::Bool(identifier.value.parse::<bool>().unwrap())),

        TokenKind::Identifier => Ok(Node::Identifier(identifier.value.clone())),
        TokenKind::New => {
            let token = get_current(tokens, index);
            assert_eq!(
                token.kind,
                TokenKind::Identifier,
                "Expected identifier token, instead got {:?}",
                token
            );

            let structname = token.clone();
            *index += 1;

            let token = get_current(tokens, index);
            assert!(
                token.kind == TokenKind::OpenCurlyBrace || token.kind == TokenKind::OpenParenthesis,
                "Expected open curly brace token"
            );
            *index += 1;

            parse_struct_init(tokens, index, &structname)
        }
        TokenKind::String => Ok(Node::String(identifier.value.clone())),
        TokenKind::OpenBracket => {
            let init = parse_array_initializer(tokens, index)?;
            Ok(new_array(
                "dynamic".to_string(),
                init.len(),
                init.clone(),
                true,
                false,
            ))
        }
        TokenKind::OpenParenthesis => {
            let node = parse_expression(tokens, index)?;
            assert_eq!(
                tokens.get(*index).map(|t| t.kind),
                Some(TokenKind::CloseParenthesis),
                "Expected close parenthesis token"
            );
            *index += 1;
            Ok(node)
        }

        TokenKind::Repeat => parse_repeat_stmnt(get_current(tokens, index), index, tokens),
        _ => Err(PrsErr {
            message: String::from("Unexpected token"),
            token: get_current(tokens, index).clone(),
            type_: ErrType::UnexpectedToken,
            index: *index,
        }),
    }
}

fn parse_digits(identifier: &Token) -> Result<Node, PrsErr> {
    let int = identifier.value.parse::<i32>();
    let float = identifier.value.parse::<f64>();

    Ok(int
        .map(Node::Int)
        .unwrap_or_else(|_| float.map(Node::Double).unwrap()))
}
fn parse_struct_init(
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
