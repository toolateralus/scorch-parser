use lexer::{create_tokenizer, TokenProcessor};
use std::{fs::File, io::Read};

use crate::parser::expression;
pub mod ast;
pub mod lexer;
pub mod parser;
fn main() {
    let mut file = File::open("src/tests.scorch").expect("Failed to open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read file");
    let mut tokenizer = create_tokenizer();
    tokenizer.tokenize(&input);
    let ast_root = expression::parse_program(&tokenizer.tokens);
    dbg!(&ast_root);
}
#[cfg(test)]
mod tests {
    use lexer::{create_tokenizer, TokenProcessor};
    
    use super::*;

    #[test]
    fn test_main() {
        // todo: make testing much much better in this project.
        // tests for both tokenizer and parser, individual rules & actual unit size tests.

        let mut file = File::open("src/tests.scorch").expect("Failed to open file");
        let mut input = String::new();
        file.read_to_string(&mut input)
            .expect("Failed to read file");
        let mut tokenizer = create_tokenizer();
        tokenizer.tokenize(input.as_str());
        
        let ast_root = expression::parse_program(&tokenizer.tokens);
        
        match ast_root {
            Ok(_) => {}
            Err(err) => panic!("{:#?}", err),
        }
        if let Ok(ast_root) = ast_root {
            panic!("{:#?}", ast_root);
        };
    }
}
