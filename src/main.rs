pub mod lexer;
pub mod repl;
mod parser;
use repl::*;
mod ast;

fn main() {
    repl();
}
