use crate::parser::parser::{Parser, ParseError};
use crate::lexer::token;
use crate::ast::type_node::Type;
use crate::Error;


impl Parser {
    pub fn parse_type_expr(&mut self, tokens: &[token::Token]) -> (Result<Type, Error>, usize) {
        let tokens = tokens.to_vec();

        if let token::Token::Function(token::Function::Fn) = tokens[0] {
            let ((signature, name), new_pos) = match self.parse_fn_signature(&tokens) {
                (Ok(k), new_pos) => (k, new_pos),
                (Err(e), new_pos) => return (Err(e), new_pos),
            };
            if let Some(name) = name {
                return (Err(Error::ParserError(ParseError::UnexpectedToken(token::Token::Identifier(name)))), 0);
            }
            return (Ok(Type::FnType(Box::new(signature))), new_pos);
        }

        let max_expr_length= match self.find_expr_possible_boundary(&tokens, false, false, false) {
            Ok(length) => length,
            Err(e) => return (Err(e), 0)
        };
        let tokens = &tokens[..max_expr_length];

        if let Ok(Some(_)) = self.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Comma), tokens) {
            let tuple = match self.make_tuple(tokens) {
                Ok(t) => t,
                Err(e) => return (Err(e), max_expr_length)
            };

            let tuple = match self.parse_tuple(tuple, |s, tok| s.parse_type_expr(tok).0) {
                Ok(t) => t,
                Err(e) => return (Err(e), max_expr_length)
            };

            let tuple = match Type::from_tuple(tuple) {
                Ok(t) => t,
                Err(e) => return (Err(Error::TypeError(e)), max_expr_length)
            };

            return (Ok(tuple), max_expr_length)
        } else {
            if self.is_in_parenthesis(tokens) {
                return match self.parse_type_expr(&tokens[1..tokens.len()-1]).0 {
                    Ok(t) => (Ok(Type::Tuple(vec![t])), max_expr_length),
                    Err(e) => (Err(e), max_expr_length)
                };
            } else {
                let t = match tokens[0].clone() {
                    token::Token::Type(type_) => Ok(Type::from_token_type(type_)),
                    _ => Err(Error::ParserError(ParseError::UnexpectedToken(tokens[0].clone())))
                };
                return (t, max_expr_length)
            }
        }
    }
}