use crate::lexer::token;
use crate::ast::*;
use crate::Error;

pub struct Parser {
    pub tokens: Vec<token::Token>,
}

#[derive(Debug)]
pub enum ParseError {
    Unimplimented,
    CannotBuildTupleType,
    EmptyTokens,
    InvalidExpression,
    InvalidOperator,
    UnexpectedToken(token::Token),
    NoMatchingBracket,
    NoConditionalFound,
    NoMatchingBraceForKeyword(token::Token),
    NoConditionForConditional,
    NoLoopFound,
    UnexpectedContentBeforeBlock,
    UnexpectedBeginningOfBlock,
    AssignmentTupleNotIdentifier,
}

impl Parser {
    pub fn new(tokens: Vec<token::Token>) -> Self {
        Self { tokens: tokens.into_iter().filter(|t| !matches!(t, token::Token::Comment(_))).collect() }
    }

    pub fn parse(&mut self) -> Result<Box<dyn ast::ASTNode>, Error> {
        match self.parse_tokens(&self.tokens.clone()) {
            (Ok(v), _) => Ok(v),
            (Err(e), _) => Err(e)
        }
    }

    pub fn parse_tokens(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, Error>, usize) {
        let mut tokens = tokens.to_vec();
        let mut result = MultiExpression { children: Vec::new() };
        if tokens.is_empty() {
            return (Err(Error::ParserError(ParseError::EmptyTokens)), 0);
        }
        while !tokens.is_empty() {
            let (node, new_pos) = self.parse_expr(&tokens);
            
            match node {
                Ok(node) => result.children.push(node),
                Err(e) => return (Err(e), new_pos),
            };
            tokens = tokens[new_pos..].to_vec();
        }
        (Ok(Box::new(result)), tokens.len())
    }

}