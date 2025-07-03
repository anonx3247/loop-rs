use crate::{ast::ASTNode, lexer::{token}};
use super::parser::{Parser, ParseError};
use crate::Error;

impl Parser {
    pub fn parse_block_expr(
        &mut self, tokens: &[token::Token],
        parse_expr: Option<fn(&mut Self, &[token::Token]) -> (Result<Box<dyn ASTNode>, Error>, usize)>
    ) -> (Result<(Box<dyn ASTNode>, Option<Box<dyn ASTNode>>), Error>, usize) {
        let brace_loc = self.find_opening_brace_for(&tokens, tokens[0].clone());
        let brace_loc = match brace_loc {
            Ok(loc) => loc,
            Err(e) => return (Err(e), 0)
        };

        let expr = if brace_loc > 1 && parse_expr.is_some() {
            let (expr, new_pos) = parse_expr.unwrap()(self, &tokens[1..brace_loc]);
            let expr = match expr {
                Ok(e) => e,
                Err(e) => return (Err(e), new_pos)
            };
            Some(expr)
        } else if brace_loc > 1 && parse_expr.is_none() {
            return (Err(Error::ParserError(ParseError::UnexpectedContentBeforeBlock)), 0);
        } else if brace_loc == 1 && parse_expr.is_some() {
            return (Err(Error::ParserError(ParseError::UnexpectedBeginningOfBlock)), 0);
        } else {
            None
        };

        let matching_loc = match self.find_matching_bracket(&tokens, brace_loc) {
            Ok(loc) => loc,
            Err(e) => return (Err(e), brace_loc)
        };
        let (content, _) = match self.parse_tokens(&tokens[brace_loc+1..matching_loc]) {
            (Ok(c), loc) => (c, loc),
            (Err(e), loc) => return (Err(e), brace_loc + loc)
        };

        (Ok((content, expr)), matching_loc)
    }
}