use crate::lexer::token;
use crate::ast::*;
use super::parser::{Parser, ParseError};

impl Parser {
    pub fn parse_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, ParseError>, usize) {
        if !tokens.is_empty() && tokens[0] == token::Token::Bracket(token::Bracket::CloseParen)
        && match self.find_matching_bracket(&tokens, 0) {
            Ok(pos) => pos == tokens.len() - 1,
            _ => false,
        } {
            return (self.parse_expr(&tokens[1..tokens.len()-1]).0, tokens.len());
        }

        if tokens.len() == 1 {
            match tokens[0] {
                token::Token::Identifier(_) => {
                    let identifier = identifier::Identifier::from_token(tokens[0].clone())
                        .map_err(|e| ParseError::Error(e.to_string()));
                    return (match identifier {
                        Ok(identifier) => Ok(Box::new(identifier)),
                        Err(e) => Err(e),
                    }, 1);
                }
                _ => {
                    let literal = literal::Literal::from_token(tokens[0].clone())
                        .map_err(|e| ParseError::Error(e.to_string()));
                    return (match literal {
                        Ok(literal) => Ok(Box::new(literal)),
                        Err(e) => Err(e),
                    }, 1);
                }
            }
        }

        let assign_tokens = [
            token::Token::Operator(token::Operator::Assign),
            token::Token::Operator(token::Operator::EqualSign),
            token::Token::VariableDeclaration(token::VariableDeclaration::Mut)
        ];

        let bool_tokens = [
            token::Token::Operator(token::Operator::And),
            token::Token::Operator(token::Operator::Or),
            token::Token::Operator(token::Operator::Not),
            token::Token::Operator(token::Operator::Eq),
            token::Token::Operator(token::Operator::Neq),
            token::Token::Operator(token::Operator::Gt),
            token::Token::Operator(token::Operator::Gte),
            token::Token::Operator(token::Operator::Lt),
            token::Token::Operator(token::Operator::Lte),
        ];

        let conditional_tokens = [
            token::Token::Conditional(token::Conditional::If),
            token::Token::Conditional(token::Conditional::Else),
            token::Token::Conditional(token::Conditional::Elif),
        ];

        for op in assign_tokens.iter() {
            if let Ok(Some(_)) = self.find_first_token_skip_brackets(&op, &tokens) {
                return self.parse_assignment_expr(&tokens);
            }
        }

        for op in conditional_tokens.iter() {
            if let Ok(Some(_)) = self.find_first_token_skip_brackets(&op, &tokens) {
                let (node, pos) = self.parse_conditional_expr(&tokens);
                let node = match node {
                    Ok(node) =>  node.clone(),
                    Err(e) => return (Err(e), pos),
                };
                return (Ok(node), pos);
            }
        }

        for op in bool_tokens.iter() {
            if let Ok(Some(_)) = self.find_first_token_skip_brackets(&op, &tokens) {
                return self.parse_bool_expr(&tokens);
            }
        }
        self.parse_math_expr(tokens)
    }
} 