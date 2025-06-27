use crate::{ast::ASTNode, ast::loops::{Loop, For, While}, lexer::{token}};
use super::parser::{Parser, ParseError};

impl Parser {
    pub fn parse_split_on_commas_expr(
        &mut self, tokens: &[token::Token],
        parse_expr: fn(&mut Self, &[token::Token]) -> (Result<Box<dyn ASTNode>, ParseError>, usize)
    ) -> (Result<Vec<Box<dyn ASTNode>>, ParseError>, usize) {
        let mut token_blocks = vec![];
        let mut current_tokens = tokens;
        let mut cursor = 0;
        while cursor < tokens.len() && current_tokens.len() > 0 {
            let next_pos = match self.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Comma), &current_tokens) {
                Ok(pos) => pos,
                Err(e) => return (Err(e), cursor)
            };
            if next_pos.is_some() {
                token_blocks.push(current_tokens[..next_pos.unwrap()].to_vec());
                cursor = next_pos.unwrap()+1;
                current_tokens = &current_tokens[cursor..];
            } else {
                token_blocks.push(current_tokens.to_vec());
                cursor = tokens.len();
            }
        }

        let mut parsed_blocks = vec![];
        for t in token_blocks.iter() {
            match parse_expr(self, t) {
                (Ok(e), l) => parsed_blocks.push(e),
                (Err(e), l) => return (Err(e), l)
            }
        }

        (Ok(parsed_blocks), tokens.len())
    }

    pub fn get_tuple_length(&mut self, tokens: &[token::Token]) -> Result<usize, ParseError> {
        let mut length = 1;
        let mut cursor = 0;
        while cursor < tokens.len() {
            let next_pos = match self.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Comma), &tokens[cursor..]) {
                Ok(pos) => pos,
                Err(e) => return Err(e)
            };
            if next_pos.is_some() {
                length += 1;
                cursor += next_pos.unwrap() + 1;
            } else {
                return Ok(length);
            }
        }
        Ok(length)
    }

    pub fn parse_one_or_n_exprs<T>(&mut self, tokens: &[token::Token], amount: usize, parse_expr: fn(&mut Self, &[token::Token]) -> (Result<T, ParseError>, usize)) -> (Result<Vec<T>, ParseError>, usize) {
        let mut nodes = vec![];
        let mut has_many = true;

        // first check if there is only one or many:
        let next_comma = match self.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Comma), &tokens) {
            Ok(pos) => pos,
            Err(e) => return (Err(e), 0)
        };
        let next_boundary = self.find_expr_possible_boundary(tokens, false, false);

        if next_comma.is_none() || (next_comma.is_some() && next_comma.unwrap() > next_boundary) {
            has_many = false;
        }

        if !has_many || amount == 1 {
            let (node, pos) = parse_expr(self, tokens);
            let node = match node {
                Ok(node) => node,
                Err(e) => return (Err(e), pos)
            };
            nodes = vec![node];
            return (Ok(nodes), pos);
        } else {
            let mut count = amount - 1;
            let mut pos = 0;
            while count > 0 {
                let next_comma = match self.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Comma), &tokens[pos..]) {
                    Ok(pos) => pos,
                    Err(e) => return (Err(e), pos)
                };
                if next_comma.is_some() {
                    count -= 1;
                } else {
                    return (Err(ParseError::InvalidExpression), pos);
                }
                let (node, new_pos) = parse_expr(self, &tokens[pos..next_comma.unwrap()+pos]);
                match node {
                    Ok(node) => nodes.push(node),
                    Err(e) => return (Err(e), new_pos)
                }
                pos += next_comma.unwrap() + 1;
            }
            let (node, new_pos) = parse_expr(self, &tokens[pos..]);
            match node {
                Ok(node) => nodes.push(node),
                Err(e) => return (Err(e), new_pos + pos)
            }
            return (Ok(nodes), new_pos + pos);
        }

    }
}