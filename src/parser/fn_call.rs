use std::collections::HashMap;

use crate::{lexer::token};
use crate::ast::function::FnCall;
use crate::ast::*;
use super::parser::{Parser, ParseError};
use crate::Error;
use crate::ast::tuple::Tuple;

impl Parser {
    pub fn parse_fn_call(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, Error>, usize) {
        let name = match tokens[0].clone() {
            token::Token::Identifier(name) => name,
            _ => return (Err(Error::ParserError(ParseError::UnexpectedToken(tokens[0].clone()))), 0),
        };
        if 1 < tokens.len() && matches!(tokens[1], token::Token::Bracket(token::Bracket::OpenParen)) {
            let matching_loc = match self.find_matching_bracket(&tokens, 1) {
                Ok(loc) => loc,
                Err(e) => return (Err(e), 0),
            };
            let params = match self.parse_params(&tokens[2..matching_loc]) {
                Ok(params) => params,
                Err(e) => return (Err(e), 0),
            };
            return (Ok(Box::new(FnCall { name, params })), matching_loc + 1);
        }
        return (Err(Error::ParserError(ParseError::UnexpectedToken(tokens[0].clone()))), 0);
    }

    fn parse_params(&mut self, tokens: &[token::Token]) -> Result<HashMap<Option<String>, Box<dyn ast::ASTNode>>, Error> {
        let tuple = match self.make_tuple(tokens) {
            Ok(tuple) => tuple,
            Err(e) => return Err(e),
        };
        
        let params = self.parse_tuple(tuple, |s, tok| {
            if let Ok(Some(_)) = s.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Colon), tok) {
                let name = match tok[0].clone() {
                    token::Token::Identifier(name) => name,
                    _ => return Err(Error::ParserError(ParseError::UnexpectedToken(tok[0].clone()))),
                };
    
                if 1 >= tok.len() {
                    return Err(Error::ParserError(ParseError::UnexpectedEndOfInput));
                }else if !matches!(tok[1], token::Token::Punctuation(token::Punctuation::Colon)) {
                    return Err(Error::ParserError(ParseError::UnexpectedToken(tok[1].clone())));
                } else {
                    let value = s.parse_expr(&tok[2..]).0?;
                    return Ok((Some(name), value));
                }
            } else {
                let value = s.parse_expr(&tok).0?;
                return Ok((None, value));
            }
        })?;

        match params {
            Tuple::Empty => Ok(HashMap::new()),
            Tuple::Element((name, value)) => Ok(HashMap::from([(name, value)])),
            Tuple::List(elements) => {
                let mut map = HashMap::new();
                for element in elements {
                    match element {
                        Tuple::Element((name, value)) => {
                            map.insert(name, value);
                        }
                        _ => return Err(Error::ParserError(ParseError::IncorrectFunctionCallSyntax)),
                    }
                }
                Ok(map)
            }
        }

    }
}