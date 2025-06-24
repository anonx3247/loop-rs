use crate::{ast::ASTNode, ast::loops::{Loop, For, While}, lexer::{token}};
use super::parser::{Parser, ParseError};



impl Parser {
    pub fn parse_loop_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ASTNode>, ParseError>, usize) {
        if matches!(tokens[0], token::Token::Loop(token::Loop::Loop)) 
        || matches!(tokens[0], token::Token::Loop(token::Loop::For))
        || matches!(tokens[0], token::Token::Loop(token::Loop::While)) {

            let ((content, expr), matching_loc) = match self.parse_block_expr(tokens, match tokens[0] {
                token::Token::Loop(token::Loop::For) => Some(|s, tok| s.parse_for_expr(tok)),
                token::Token::Loop(token::Loop::While) => Some(|s, tok| s.parse_expr(tok)),
                token::Token::Loop(token::Loop::Loop) => None,
                _ => return (Err(ParseError::NoLoopFound), 0)
            }) {
                (Ok(c), l) => (c, l),
                (Err(e), l) => return (Err(e), l)
            };


            let output: Result<Box<dyn ASTNode>, ParseError> = match tokens[0] {
                token::Token::Loop(token::Loop::For) => Ok(Box::new(For::new(expr.unwrap(), content.children()))),
                token::Token::Loop(token::Loop::While) => Ok(Box::new(While::new(expr.unwrap(), content.children()))),
                token::Token::Loop(token::Loop::Loop) => Ok(Box::new(Loop::new(content.children()))),
                _ => Err(ParseError::NoLoopFound)
            };

            return (output, matching_loc+1)
        }
        (Err(ParseError::NoLoopFound), 0)

    }

    pub fn parse_for_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ASTNode>, ParseError>, usize) {
        (Err(ParseError::Unimplimented), 0)
    }
}