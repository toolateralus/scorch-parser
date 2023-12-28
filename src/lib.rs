pub mod ast;
pub mod lexer;
pub mod parser;


#[cfg(test)]
mod tests {
    use lexer::{create_tokenizer, TokenProcessor};
    
    use super::*;
    
    #[test]
    fn test_main() {
        // todo: make testing much much better in this project.
        // tests for both tokenizer and parser, individual rules & actual unit size tests.
        let input = "
        x : Int = 1 + (2 * 3)
        funcy : Fn(x : Int) -> Bool {
            break 250 * x
        }
        [funcy(2 * x)].println()
        ";
        
        let mut tokenizer = create_tokenizer();   
        
        tokenizer.tokenize(input);
        
        let ast_root = parser::parse_program(&tokenizer.tokens);
        
        dbg!(&ast_root);
    }
}