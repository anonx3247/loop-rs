pub mod lexer;
pub mod repl;
mod parser;
use repl::*;

fn main() {
    repl();
}
