use crate::lexer::token;
use crate::ast::*;

pub struct Parser {
    pub tokens: Vec<token::Token>,
}

#[derive(Debug)]
pub enum ParseError {
    Unimplimented,
    EmptyTokens,
    InvalidExpression,
    InvalidOperator,
    InvalidToken,
    NoMatchingBracket,
    NoConditionalFound,
    NoMatchingBraceForKeyword(token::Token),
    NoConditionForConditional,
    NoLoopFound,
    UnexpectedContentBeforeBlock,
    UnexpectedBeginningOfBlock,
    Error(String),
}

impl Parser {
    pub fn new(tokens: Vec<token::Token>) -> Self {
        Self { tokens: tokens.into_iter().filter(|t| !matches!(t, token::Token::Comment(_))).collect() }
    }

    pub fn parse(&mut self) -> Result<Box<dyn ast::ASTNode>, ParseError> {
        self.parse_tokens(&self.tokens.clone())
    }

    pub fn parse_tokens(&mut self, tokens: &[token::Token]) -> Result<Box<dyn ast::ASTNode>, ParseError> {
        let mut tokens = tokens.to_vec();
        let mut result = RootASTNode::new();
        if tokens.is_empty() {
            return Err(ParseError::EmptyTokens);
        }
        while !tokens.is_empty() {
            let (node, new_pos) = self.parse_expr(&tokens);
            
            match node {
                Ok(node) => result.push(node),
                Err(e) => return Err(e),
            };
            tokens = tokens[new_pos..].to_vec();
        }
        Ok(Box::new(result))
    }

}