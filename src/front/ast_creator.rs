use crate::front::ast_creator::lexer::get_tokens;
use crate::front::ast_types::Module;

mod lexer;
mod token_types;
mod parser;

fn create_ast(src: &str) -> Module {
    // TODO: error handling
    let tokens = get_tokens(src).unwrap();
    let ast = parser::parse_tokens(tokens).unwrap();

    return ast;
}