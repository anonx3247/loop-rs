use std::collections::HashMap;

use crate::ast::scope::Scope;
use crate::ast::type_node::Type;
use crate::{lexer::token};
use crate::ast::function::{FnDeclaration, FnSignature};
use crate::ast::*;
use super::parser::{Parser, ParseError};
use crate::Error;
use crate::ast::tuple::Tuple;

impl Parser {

    pub fn parse_fn_signature(&mut self, tokens: &[token::Token]) -> (Result<(FnSignature, Option<String>), Error>, usize) {
        // assumes that fn isn't part of the signature i.e. to parse fn + ... we only apply this function to ...
        let mut pos = 0;
        match self.check_bounds(tokens, pos) {
            Ok(_) => (),
            Err(e) => return (Err(e), 0),
        }
        let name = match tokens[pos].clone() {
            token::Token::Identifier(name) => Some(name),
            _ => None,
        };
        if name.is_some() {
            pos += 1;
        }
        match self.check_bounds(tokens, pos) {
            Ok(_) => (),
            Err(e) => return (Err(e), 0),
        }
        let params = if matches!(tokens[pos], token::Token::Bracket(token::Bracket::OpenParen)) {
            let matching_loc = match self.find_matching_bracket(&tokens, pos) {
                Ok(loc) => loc,
                Err(e) => return (Err(e), pos),
            };

            if matching_loc == pos + 1 {
                pos += 1;
                HashMap::new()
            } else {
            
                let p = match self.parse_declaration_params(&tokens[pos+1..matching_loc]) {
                    Ok(params) => params,
                    Err(e) => return (Err(e), 0),
                };
                pos = matching_loc + 1;
                p
            }
        } else {
            return (Err(Error::ParserError(ParseError::UnexpectedToken(tokens[pos].clone()))), pos);
        };
        match self.check_bounds(tokens, pos) {
            Ok(_) => (),
            Err(e) => return (Err(e), 0),
        }
        let mut return_type = None;
        if matches!(tokens[pos], token::Token::Function(token::Function::Arrow)) {
            pos += 1;
            let (type_, new_pos) = match self.parse_type_expr(&tokens[pos..]) {
                (Ok(type_), new_pos) => (type_, new_pos),
                (Err(e), new_pos) => return (Err(e), new_pos+pos),
            };
            pos += new_pos;
            return_type = Some(type_);
        }
        (Ok((FnSignature { params, return_type }, name)), pos)
    }

    pub fn parse_fn_declaration(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, Error>, usize) {

        let ((block, expr), new_pos) = match self.parse_block_expr(tokens, Some(|s, tok| s.parse_fn_signature(tok))) {
            (Ok(k), new_pos) => (k, new_pos),
            (Err(e), new_pos) => return (Err(e), new_pos),
        };

        println!("block: {:?}", block);
        println!("expr: {:?}", expr);
        println!("new_pos: {}", new_pos);

        let (signature, name) = match expr {
            Some((signature, name)) => (signature, name),
            None => return (Err(Error::ParserError(ParseError::UnexpectedToken(tokens[0].clone()))), 0),
        };
        (Ok(Box::new(FnDeclaration::from_signature(name, signature, Scope::new(block.children())))), new_pos+1)
    }

    pub fn parse_declaration_params(&mut self, tokens: &[token::Token]) -> Result<HashMap<String, Type>, Error> {
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
                    let type_ = s.parse_type_expr(&tok[2..]).0?;
                    return Ok((name, type_));
                }
            } else {
                return Err(Error::ParserError(ParseError::UnexpectedToken(tok[0].clone())));
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