use crate::{ast::tuple::{Clonable, Tuple, TupleError}, lexer::token};
use super::parser::Parser;
use crate::Error;

impl Parser {
    pub fn parse_tuple<K: Clonable>(&mut self, tuple: Tuple<Vec<token::Token>>, parse_expr: fn(&mut Self, &[token::Token]) -> Result<K, Error>) -> Result<Tuple<K>, Error> {
        match tuple {
            Tuple::Empty => return Ok(Tuple::Empty),
            Tuple::Element(tokens) => {
                let node = parse_expr(self, &tokens)?;
                return Ok(Tuple::Element(node));
            }
            Tuple::List(elements) => {
                let mut out_tuple = vec![];
                for e in elements.iter() {
                    out_tuple.push(self.parse_tuple(e.clone(), parse_expr)?);
                }
                return Ok(Tuple::List(out_tuple));
            }
        }
    }


    pub fn is_identifier_tuple(&mut self, tuple: Tuple<Vec<token::Token>>) -> bool {
        match tuple {
            Tuple::Empty => return false,
            Tuple::Element(tokens) => {
                if tokens.len() == 1 && matches!(tokens[0], token::Token::Identifier(_)) {
                    return true;
                }
                return false;
            }
            Tuple::List(elements) => {
                for e in elements.iter() {
                    if !self.is_identifier_tuple(e.clone()) {
                        return false;
                    }
                }
                return true;
            }
        }
    }

    pub fn make_tuple(&mut self, tokens: &[token::Token]) -> Result<Tuple<Vec<token::Token>>, Error> {
        // note this function expects the entire token list to be the tuple

        // first check if there is only one or many:
        let next_comma = match self.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Comma), &tokens) {
            Ok(pos) => pos,
            Err(e) => return Err(e)
        };

        if next_comma.is_none() {
            /*
            if tokens.len() >= 2 && tokens[0] == token::Token::Bracket(token::Bracket::OpenParen) && tokens[tokens.len()-1] == token::Token::Bracket(token::Bracket::CloseParen) {
                let inner_tuple = self.make_tuple(&tokens[1..tokens.len()-1])?;
                return Ok(Tuple::List(vec![inner_tuple]));
            } else {
                return Ok(Tuple::Element(tokens.to_vec()));
            }
            */
            return Ok(Tuple::Element(tokens.to_vec()));
        }

        let mut tuple = vec![tokens[..next_comma.unwrap()].to_vec()];
        let mut cursor = next_comma.unwrap() + 1;
        while cursor < tokens.len() {
            let next_comma = match self.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Comma), &tokens[cursor..]) {
                Ok(pos) => pos,
                Err(e) => return Err(e)
            };

            if next_comma.is_none() {
                tuple.push(tokens[cursor..].to_vec());
                break;
            }

            tuple.push(tokens[cursor..next_comma.unwrap()+cursor].to_vec());
            cursor += next_comma.unwrap() + 1;
        }

        match tuple.len() {
            0 => {
                return Ok(Tuple::Empty);
            }
            1 => {
                return Ok(Tuple::Element(tuple[0].to_vec()));
            }
            _ => {
                let mut out_tuple = vec![];
                for t in tuple.iter() {
                    if t.len() >= 2 && t.first().unwrap() == &token::Token::Bracket(token::Bracket::OpenParen) && t.last().unwrap() == &token::Token::Bracket(token::Bracket::CloseParen) {
                        out_tuple.push(self.make_tuple(&t[1..t.len()-1])?);
                    } else {
                        out_tuple.push(Tuple::Element(t.to_vec()));
                    }
                }
                return Ok(Tuple::List(out_tuple));
            }
        }
    }

    pub fn make_left_matching_tuple<T: Clonable>(&mut self, tokens: &[token::Token], structure: Tuple<T>) -> (Result<Tuple<Vec<token::Token>>, Error>, usize) {
        let expr_lim = self.find_expr_possible_boundary(&tokens, false, false);
        let tokens = if expr_lim >= tokens.len() {
            &tokens
        } else {
            &tokens[..expr_lim]
        };

        let tuple = match self.make_tuple(&tokens) {
            Ok(tuple) => tuple,
            Err(e) => return (Err(e), expr_lim+1)
        };
        if !structure.matches_left_structure(&tuple) {
            return (Err(Error::TupleError(TupleError::CannotPairUp)), expr_lim+1);
        }
        (Ok(tuple), expr_lim+1)
    }

}