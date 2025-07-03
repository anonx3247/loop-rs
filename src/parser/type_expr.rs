use crate::parser::parser::{Parser, ParseError};
use crate::lexer::token;
use crate::ast::type_node::Type;
use crate::Error;


impl Parser {
    pub fn parse_type_expr(&mut self, tokens: &[token::Token]) -> (Result<Type, Error>, usize) {
        let tokens = tokens.to_vec();

        let max_expr_length= self.find_expr_possible_boundary(&tokens, false, false);
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