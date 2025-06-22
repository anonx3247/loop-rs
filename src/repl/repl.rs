use std::io::*;
use crate::lexer::Lexer;
use crate::ast::*;
use crate::parser::Parser;
pub fn repl() {
    loop {
        let mut input = String::new();
        print!(":: ");
        
        stdout().flush().unwrap();
        
        match stdin().read_line(&mut input) {
            Ok(_) => {
                if input == "exit\n" {
                    break;
                }
                else if input.ends_with("\n") {
                    input.pop(); // Remove the last newline
                    let mut lexer = Lexer::new(input);
                    match lexer.tokenize() {
                        Ok(_) => {
                            let parser = Parser::new(lexer.tokens.clone());
                            match parser.parse() {
                                Ok(ast) => {
                                    println!("{}", ast.to_string(0));
                                    match ast.eval() {
                                        Ok(value) => println!("{}", value),
                                        Err(e) => println!("Error evaluating: {}", e),
                                    }
                                }
                                Err(e) => {
                                    println!("Error parsing: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error tokenizing: {}", e);
                        }
                    }
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            }
        }
    }
}
