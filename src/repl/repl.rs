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
                                    let eval = eval(ast);
                                    println!("{}", eval);
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

pub fn eval(ast: Box<dyn ASTNode>) -> f64 {
    match ast.eval() {
        Ok(Value::Int(i)) => i as f64,
        Ok(Value::Float(f)) => f,
        _ => panic!("Invalid AST node"),
    }
}
