use crate::{ast::block::{Conditional, ElifBlock, ElseBlock, IfBlock}, lexer::{token}};
use super::parser::{Parser, ParseError};



impl Parser {

    pub fn parse_conditional_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn Conditional>, ParseError>, usize) {
        match &tokens[0] {
            token::Token::Conditional(c) => {
                if let token::Conditional::Match = c {
                    return (Err(ParseError::Unimplimented), 0)
                }
                let brace_loc = self.find_opening_brace_for(&tokens, tokens[0].clone());
                let brace_loc = match brace_loc {
                    Ok(loc) => loc,
                    Err(e) => return (Err(e), 0)
                };
                let condition = if brace_loc != 1 {
                    let (condition, new_pos) = self.parse_expr(&tokens[1..brace_loc]);
                    let condition = match condition {
                        Ok(c) => c,
                        Err(e) => return (Err(e), new_pos)
                    };
                    assert_eq!(new_pos+1, brace_loc, "missing tokens between if and brace");
                    Some(condition)

                } else {
                    None
                };
                let matching_loc = match self.find_matching_bracket(&tokens, brace_loc) {
                    Ok(loc) => loc,
                    Err(e) => return (Err(e), brace_loc)
                };
                let content = match self.parse_tokens(&tokens[brace_loc+1..matching_loc]) {
                    Ok(c) => c,
                    Err(e) => return (Err(e), matching_loc)
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
                            _ => return (Err(ParseError::NoConditionForConditional), 0)
                        };
                        (Ok(Box::new(IfBlock::new(condition, content.children(), next))), new_pos)
                    },
                    token::Conditional::Elif => {
                        let condition = match condition {
                            Some(c) => c,
                            _ => return (Err(ParseError::NoConditionForConditional), 0)
                        };
                        (Ok(Box::new(ElifBlock::new(condition, content.children(), next))), new_pos)
                    },
                    token::Conditional::Else => {
                        (Ok(Box::new(ElseBlock::new(content.children()))), new_pos)
                    },
                    token::Conditional::Match => 
                        (Err(ParseError::Unimplimented), new_pos)
                }
            },
            _ => (Err(ParseError::NoConditionalFound), 0)

        }
    }
}