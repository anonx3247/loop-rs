use crate::{ast::block::{ElseBlock, Conditional, IfBlock, ElifBlock}, lexer::token};
use crate::ast::*;
use super::parser::{Parser, ParseError};



impl Parser {
    pub fn parse_conditional_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn Conditional>, ParseError>, usize) {
        if let token::Token::Bracket(token::Bracket::CloseBrace) = tokens[0] {
            let match_pos = match self.find_matching_bracket(tokens, 0) {
                Ok(pos) => pos,
                _ => return (Err(ParseError::NoMatchingBracket), 0),
            };

            let content = match self.parse_tokens(&tokens[1..match_pos]) {
                Ok(node) => node,
                Err(e) => return (Err(e), match_pos+1),
            };
            
            if let token::Token::Conditional(token::Conditional::Else) = tokens[match_pos+1] {
                let (prev_conditional, new_pos) = match self.parse_conditional_expr(&tokens[match_pos+2..]) {
                    (Ok(node), pos) => (node, pos),
                    (Err(e), pos) => return (Err(e), pos),
                };
                let new_pos = new_pos + match_pos + 2;
                return (Ok(Box::new(ElseBlock::new(content.children(), prev_conditional))), new_pos);
            } else {
                let if_pos = match self.find_first_token_skip_brackets(&token::Token::Conditional(token::Conditional::If), tokens) {
                    Ok(pos) => pos,
                    Err(e) => return (Err(e), match_pos+1),
                };
                let elif_pos = match self.find_first_token_skip_brackets(&token::Token::Conditional(token::Conditional::Elif), tokens) {
                    Ok(pos) => pos,
                    Err(e) => return (Err(e), match_pos+1),
                };

                let cond_pos = if if_pos.is_some() && elif_pos.is_some() {
                    if if_pos.unwrap() < elif_pos.unwrap() {
                        if_pos.unwrap()
                    } else {
                        elif_pos.unwrap()
                    }
                } else if if_pos.is_some() {
                    if_pos.unwrap()
                } else if elif_pos.is_some() {
                    elif_pos.unwrap()
                } else {
                    return (Err(ParseError::InvalidConditional), match_pos+1);
                };

                let (condition, mut new_pos) = match self.parse_expr(&tokens[match_pos+1..cond_pos]) {
                    (Ok(node), pos) => (node, pos),
                    (Err(e), pos) => return (Err(e), pos),
                };
                new_pos += match_pos;
                assert_eq!(new_pos+1, cond_pos, "new_pos: {} != cond_pos: {}", new_pos+1, cond_pos);
                if let token::Token::Conditional(token::Conditional::Elif) = tokens[new_pos+1] {
                    let prev_conditional = match self.parse_conditional_expr(&tokens[new_pos+2..]).0 {
                        Ok(node) => node,
                        Err(e) => return (Err(e), new_pos+2),
                    };
                    return (Ok(Box::new(ElifBlock::new(condition, content.children(), prev_conditional))), new_pos+2);
                }
                if let token::Token::Conditional(token::Conditional::If) = tokens[new_pos+1] {
                    return (Ok(Box::new(IfBlock::new(condition, content.children()))), new_pos+2);
                }
                return (Err(ParseError::InvalidConditional), new_pos+1);
            }
        }
        (Err(ParseError::ConditionalHasNoBlock), 0)
    }
}