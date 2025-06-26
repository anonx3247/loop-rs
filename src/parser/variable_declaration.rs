use crate::parser::parser::{Parser, ParseError};
use crate::lexer::token;
use crate::ast::variable_declaration;
use crate::environment::environment::Type;

impl Parser {
    pub fn parse_variable_declaration_expr(&mut self, tokens: &[token::Token]) -> (Result<variable_declaration::VariableDeclaration, ParseError>, usize) {
        let pos = tokens.len() - 1;
        // [mut] identifier: type
        if pos < 2 {
            return (Err(ParseError::InvalidToken), pos+1);
        } else if let Some(token::Token::Type(type_)) = tokens.get(pos) {
            if let Some(token::Token::Punctuation(token::Punctuation::Colon)) = tokens.get(pos - 1) {
                if let Some(token::Token::Identifier(identifier)) = tokens.get(pos - 2) {
                    if pos < 3 {
                        let node = variable_declaration::VariableDeclaration::new(identifier.clone(), Type::from_token_type(type_.clone()), false);
                        return (Ok(node), pos+1);
                    } else if let Some(token::Token::VariableDeclaration(token::VariableDeclaration::Mut)) = tokens.get(pos - 3) {
                        let node = variable_declaration::VariableDeclaration::new(identifier.clone(), Type::from_token_type(type_.clone()), true);
                        return (Ok(node), pos+1);
                    } else {
                        let node = variable_declaration::VariableDeclaration::new(identifier.clone(), Type::from_token_type(type_.clone()), false);
                        return (Ok(node), pos+1);
                    }
                }
            }
        }
        return (Err(ParseError::InvalidToken), pos+1);
    }
}