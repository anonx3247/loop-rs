use crate::{ast::{conditional::{Conditional, ElifBlock, ElseBlock, IfBlock}, scope::Scope}, lexer::{token}};
use super::parser::{Parser, ParseError};
use crate::Error;



impl Parser {

    pub fn parse_conditional_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn Conditional>, Error>, usize) {
        match &tokens[0] {
            token::Token::Conditional(c) => {
                if let token::Conditional::Match = c {
                    return (Err(Error::ParserError(ParseError::Unimplimented)), 0)
                }
                let ((content, condition), matching_loc) = match self.parse_block_expr(tokens, match c {
                    token::Conditional::Else => None,
                    _ => Some(|s, tok| s.parse_expr(tok))
                }) {
                    (Ok(c), l) => (c, l),
                    (Err(e), l) => return (Err(e), l)
                };

                let mut new_pos = matching_loc + 1;
                let next: Option<Box<dyn Conditional>>;
                if new_pos < tokens.len() && c != &token::Conditional::Else {
                    next = match tokens[new_pos] {
                        token::Token::Conditional(token::Conditional::Elif)
                        |token::Token::Conditional(token::Conditional::Else) => {
                            let (next, next_loc) = match self.parse_conditional_expr(&tokens[new_pos..]) {
                                (Ok(n), l) => (n, l),
                                (Err(e), l) => return (Err(e), l)
                            };
                            new_pos += next_loc;
                            Some(next)
                        },
                    _ => None
                    }
                } else {
                    next = None;
                }

                match c {
                    token::Conditional::If => {
                        let condition = match condition {
                            Some(c) => c,
                            _ => return (Err(Error::ParserError(ParseError::NoConditionForConditional)), 0)
                        };
                        (Ok(Box::new(IfBlock::new(condition, Scope::new(content.children()), next))), new_pos)
                    },
                    token::Conditional::Elif => {
                        let condition = match condition {
                            Some(c) => c,
                            _ => return (Err(Error::ParserError(ParseError::NoConditionForConditional)), 0)
                        };
                        (Ok(Box::new(ElifBlock::new(condition, Scope::new(content.children()), next))), new_pos)
                    },
                    token::Conditional::Else => {
                        (Ok(Box::new(ElseBlock::new(Scope::new(content.children())))), new_pos)
                    },
                    token::Conditional::Match => 
                        (Err(Error::ParserError(ParseError::Unimplimented)), new_pos)
                }
            },
            _ => {
                println!("no conditional found: {:?}", tokens[0]);
                (Err(Error::ParserError(ParseError::NoConditionalFound)), 0)
            }

        }
    }
}