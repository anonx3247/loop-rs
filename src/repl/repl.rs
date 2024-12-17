use std::io::*;
use crate::lexer::Lexer;
use crate::lexer::token::*;
use crate::parser::ASTNode;
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
                            if let Ok(ast) = parser.parse() {
                                println!("{}", ast.to_string(0));
                                let eval = eval(ast);
                                println!("{}", eval);
                            }
                            else {
                                println!("Error parsing");
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

pub fn eval(ast: ASTNode) -> f32 {
    match ast.token {
        Token::Literal(Literal::Int(i)) => i as f32,
        Token::Literal(Literal::Float(f)) => f as f32,
        Token::Operator(Operator::Add) => eval(ast.children[1].clone()) + eval(ast.children[0].clone()),
        Token::Operator(Operator::Sub) => eval(ast.children[1].clone()) - eval(ast.children[0].clone()),
        Token::Operator(Operator::Mul) => eval(ast.children[1].clone()) * eval(ast.children[0].clone()),
        Token::Operator(Operator::Div) => eval(ast.children[1].clone()) / eval(ast.children[0].clone()),
        _ => panic!("Invalid AST node"),
    }
}
