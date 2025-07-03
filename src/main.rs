pub mod lexer;
pub mod repl;
mod parser;
mod ast;
pub mod environment;
mod interpreter;

use std::env;
use std::path::Path;
use repl::repl;

#[derive(Debug)]
pub enum Error {
    LexerError(lexer::LexerError),
    ParserError(parser::parser::ParseError),
    ASTError(ast::ASTError),
    RuntimeError(environment::environment::RuntimeError),
    TupleError(ast::tuple::TupleError),
    TypeError(ast::type_node::TypeError),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut print_ast = false;
    let mut print_tokens = false;
    let mut file_path: Option<String> = None;
    for arg in &args[1..] {
        match arg.as_str() {
            "--ast" => print_ast = true,
            "--tokens" => print_tokens = true,
            _ => {
                if !arg.starts_with("--") && file_path.is_none() {
                    file_path = Some(arg.clone());
                }
            }
        }
    }
    if let Some(path_str) = file_path {
        let path = Path::new(&path_str);
        interpreter::interpreter::run_file(path);
    } else {
        repl::repl(print_ast, print_tokens);
    }
}
