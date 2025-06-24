use crate::{ast::ASTNode, ast::loops::{Loop, For, While}, lexer::{token}};
use super::parser::{Parser, ParseError};

impl Parser {
    pub fn parse_block_expr(
        &mut self, tokens: &[token::Token],
        parse_expr: Option<fn(&mut Self, &[token::Token]) -> (Result<Box<dyn ASTNode>, ParseError>, usize)>
    ) -> (Result<(Box<dyn ASTNode>, Option<Box<dyn ASTNode>>), ParseError>, usize) {
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
            return (Err(ParseError::UnexpectedContentBeforeBlock), 0);
        } else if brace_loc == 1 && parse_expr.is_some() {
            return (Err(ParseError::UnexpectedBeginningOfBlock), 0);
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

        (Ok((content, expr)), matching_loc)
    }
}