use crate::{lexer::token};
use crate::ast::*;
use super::parser::{Parser, ParseError};
use crate::Error;

impl Parser {
    pub fn parse_binary_operator_expr(&mut self, tokens: &[token::Token], 
        operators: &[token::Token], 
        left_parser: Option<fn(&mut Self, &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, Error>, usize)>, 
        right_parser: Option<fn(&mut Self, &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, Error>, usize)>
        ) -> (Result<Box<dyn ast::ASTNode>, Error>, usize) {
        
        for op in operators.iter() {
            if let Ok(Some(pos)) = self.find_first_token_skip_brackets(&op, &tokens) {
                let (left, left_pos) = match left_parser {
                    Some(parser) => parser(self, &tokens[..pos]),
                    _ => self.parse_expr(&tokens[..pos]),
                };
                let (right, right_pos) = match right_parser {
                    Some(parser) => parser(self, &tokens[pos + 1..]),
                    _ => self.parse_expr(&tokens[pos + 1..]),
                };
                let node = binary_operation::BinaryOperation {
                    left: match left {
                        Ok(node) => node,
                        Err(e) => return (Err(e), left_pos),
                    },
                    right: match right {
                        Ok(node) => node,
                        Err(e) => return (Err(e), right_pos + pos + 1),
                    },
                    operator: match op {
                        token::Token::Operator(op) => op.clone(),
                        _ => return (Err(Error::ParserError(ParseError::UnexpectedToken(op.clone()))), pos)
                    },
                };
                return (Ok(Box::new(node)), right_pos + pos + 1);
            }
        }

        match tokens[0] {
            token::Token::Identifier(_) => {
                let identifier = identifier::Identifier::from_token(tokens[0].clone());
                return (match identifier {
                    Ok(identifier) => Ok(Box::new(identifier)),
                    Err(e) => Err(e),
                }, 1);
            }
            _ => {
                let literal = literal::Literal::from_token(tokens[0].clone());
                return (match literal {
                    Ok(literal) => Ok(Box::new(literal)),
                    Err(e) => Err(e),
                }, 1);
            }
        }
        
    }

    pub fn parse_math_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, Error>, usize) {
        let operators = [
            token::Token::Operator(token::Operator::Sub),
            token::Token::Operator(token::Operator::Add),
            token::Token::Operator(token::Operator::Div),
            token::Token::Operator(token::Operator::Mul),
            token::Token::Operator(token::Operator::Mod),
            token::Token::Operator(token::Operator::Pow),
            token::Token::Operator(token::Operator::BitAnd),
            token::Token::Operator(token::Operator::BitOr),
            token::Token::Operator(token::Operator::BitXor),
            token::Token::Operator(token::Operator::BitShiftLeft),
            token::Token::Operator(token::Operator::BitShiftRight),
        ];

        self.parse_binary_operator_expr(tokens, &operators, None, None)
    }

    pub fn parse_bool_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, Error>, usize) {
        let operators = [
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
        self.parse_binary_operator_expr(tokens, &operators, None, None)
    }
} 