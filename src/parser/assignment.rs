use crate::lexer::token;
use crate::ast::*;
use super::parser::{Parser, ParseError};

impl Parser {
    pub fn parse_assignment_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, ParseError>, usize) {
        let operators = [
            token::Token::Operator(token::Operator::Assign),
            token::Token::Operator(token::Operator::EqualSign)
        ];

        for op in operators.iter() {
            if let Ok(Some(pos)) = self.find_first_token_skip_brackets(&op, &tokens) {
                let (left, left_pos) = self.parse_expr(&tokens[..pos]);
                let left = match left {
                    Ok(node) => node,
                    Err(e) => return (Err(e), left_pos),
                };
                if op == &token::Token::Operator(token::Operator::Assign) {
                    // make sure the previous token is an identifier and the one before it is either a 'mut' or anything else

                    // [mut] identifier = expr
                    if let Some(token::Token::Identifier(identifier)) = tokens.get(pos + 1) {
                        if let Some(token::Token::VariableDeclaration(token::VariableDeclaration::Mut)) = tokens.get(pos + 2) {
                            let node = assignment::VariableAssignment {
                                mutable: true,
                                type_: None,
                                name: identifier.clone(),
                                expr: left,
                            };
                            return (Ok(Box::new(node)), pos + 3);
                        } else {
                            let node = assignment::VariableAssignment {
                                mutable: false,
                                type_: None,
                                name: identifier.clone(),
                                expr: left,
                            };
                            return (Ok(Box::new(node)), pos + 2);
                        }
                    }
                } else {
                    // [mut] identifier: type = expr
                    if let Some(token::Token::Type(type_)) = tokens.get(pos + 1) {
                        if let Some(token::Token::Punctuation(token::Punctuation::Colon)) = tokens.get(pos + 2) {
                            if let Some(token::Token::Identifier(identifier)) = tokens.get(pos + 3) {
                                if let Some(token::Token::VariableDeclaration(token::VariableDeclaration::Mut)) = tokens.get(pos + 4) {
                                    let node = assignment::VariableAssignment {
                                        mutable: true,
                                        type_: Some(type_.clone()),
                                        name: identifier.clone(),
                                        expr: left,
                                    };
                                    return (Ok(Box::new(node)), pos + 5);
                                } else {
                                    let node = assignment::VariableAssignment {
                                        mutable: false,
                                        type_: Some(type_.clone()),
                                        name: identifier.clone(),
                                        expr: left,
                                    };
                                    return (Ok(Box::new(node)), pos + 3);
                                }
                            }
                        } else {
                            return (Err(ParseError::InvalidToken), pos + 2);
                        }
                    } else {
                        return (Err(ParseError::InvalidToken), pos + 1);
                    }
                }
            }
        }

        (Err(ParseError::InvalidExpression), 0)
        
    }
}