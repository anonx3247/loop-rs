pub mod lexer;
pub mod repl;
mod parser;
mod ast;
pub mod environment;
mod interpreter;

use std::env;
use std::path::Path;
use repl::repl;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = Path::new(&args[1]);
        interpreter::interpreter::run_file(path);
    } else {
        repl();
    }
}
