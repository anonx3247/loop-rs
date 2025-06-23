use std::fs;
use std::path::Path;
use crate::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::environment::environment::Environment;

pub fn run_file(path: &Path) {
    let content = fs::read_to_string(path).expect("Unable to read file");

    let mut lexer = Lexer::new(content);
    match lexer.tokenize() {
        Ok(_) => {
            lexer.clean_tokens();
            let mut parser = Parser::new(lexer.tokens.clone());
            match parser.parse() {
                Ok(ast) => {
                    let mut env = Environment::new();
                    for child in ast.children() {
                        match child.eval(&mut env) {
                            Ok(value) => println!("{}", value.to_string()),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
} 