pub mod debug;
pub mod declaration;
pub mod expression;
pub mod function;
pub mod keyword;
pub mod literal;

use crate::ast::{ARRAY_TNAME, BOOL_TNAME, DOUBLE_TNAME, DYNAMIC_TNAME, INT_TNAME, STRING_TNAME};
use rand::Rng;

use self::{
    debug::{ErrType, PrsErr},
    declaration::parse_decl_stmnt,
    expression::parse_expression,
    keyword::parse_keyword_ops,
    literal::new_array,
};

use super::{
    ast::Node,
    lexer::{Token, TokenFamily, TokenKind},
};

pub fn current_token<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> &'a Token {
    if let Some(token) = tokens.get(*index) {
        return token;
    } else {
        panic!("Unexpected end of tokens");
    }
}
pub fn consume_newlines<'a>(index: &mut usize, tokens: &'a Vec<Token>) -> &'a Token {
    let mut current = current_token(tokens, index);
    while current.kind == TokenKind::Newline {
        *index += 1;
        current = current_token(tokens, index);
    }
    return current;
}
pub fn consume_delimiter(tokens: &Vec<Token>, index: &mut usize) {
    let current = current_token(tokens, index).kind;
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
pub fn generate_random_function_name() -> String {
    let mut rng = rand::thread_rng();
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let name: String = (0..8)
        .map(|_| letters[rng.gen_range(0..letters.len())])
        .collect();
    name
}
pub fn consume(tokens: &Vec<Token>, index: &mut usize, expected: TokenKind) {
    let current = current_token(tokens, index);
    if current.kind != expected {
        panic!("Expected {:?}, got {:?}", expected, current.kind);
    }
    *index += 1;
}
pub fn consume_next_if_any(tokens: &Vec<Token>, index: &mut usize, vec: Vec<TokenKind>) {
    let current = current_token(tokens, index);
    if !vec.contains(&current.kind) {
        panic!("Expected {:?}, got {:?}", vec, current.kind);
    }
    *index += 1;
}
