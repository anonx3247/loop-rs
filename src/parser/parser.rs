use assignment::VariableAssignment;

use crate::lexer::token;
use crate::ast::*;

use self::token::VariableDeclaration;
pub struct Parser {
    tokens: Vec<token::Token>,
}

#[derive(Debug)]
pub enum ParseError {
    EmptyTokens,
    InvalidMathExpression,
    InvalidOperator,
    InvalidToken,
    NoMatchingBracket,
    Error(String),
}

impl Parser {
    pub fn new(tokens: Vec<token::Token>) -> Self {
        Self { tokens: tokens.into_iter().rev().collect() }
    }

    pub fn parse(&self) -> Result<Box<dyn ast::ASTNode>, ParseError> {
        self.parse_tokens(&self.tokens)
    }

    fn parse_tokens(&self, tokens: &[token::Token]) -> Result<Box<dyn ast::ASTNode>, ParseError> {
        if tokens.len() == 0 {
            return Err(ParseError::EmptyTokens);
        }
        match tokens[0] {
            token::Token::VariableDeclaration(token::VariableDeclaration::Mut) | token::Token::Identifier(_) =>
                self.parse_assignment(tokens),
            _ => self.parse_math_expr(tokens),
        }
    }

    fn find_first_token(&self, token: &token::Token, tokens: &[token::Token]) -> Option<usize> {
        tokens.iter().position(|t| t == token)
    }

    fn find_first_token_skip_brackets(&self, token: &token::Token, tokens: &[token::Token]) -> Result<Option<usize>, ParseError> {
        let mut i = 0;
        while i < tokens.len() {
            if tokens[i] == *token {
                return Ok(Some(i));
            }
            match tokens[i] {
                token::Token::Bracket(_) => {
                    let close_pos = self.find_matching_bracket(&tokens, i)?;
                    i = close_pos + 1;
                }
                _ => i += 1,
            }
        }
        Ok(None)
    }

    fn parse_assignment(&self, tokens: &[token::Token]) -> Result<Box<dyn ast::ASTNode>, ParseError> {
        if tokens.is_empty() {
            Err(ParseError::EmptyTokens)
        } else {
            let mut current = 0;
            let mutable = match tokens[0] {
                token::Token::Identifier(_) => false,
                token::Token::VariableDeclaration(token::VariableDeclaration::Mut) => true,
                _ => return Err(ParseError::InvalidToken),
            };

            current = if mutable { 1 } else { 0 };

            let identifier = match tokens[current] {
                token::Token::Identifier(_) => tokens[current].to_string(),
                _ => return Err(ParseError::InvalidToken),
            };

            current += 1;

            let mut type_ = match tokens[current] {
                token::Token::Punctuation(token::Punctuation::Colon) => match &tokens[current + 1]  {
                    token::Token::Type(t) => Some(t.clone()),
                    _ => return Err(ParseError::InvalidToken),
                },
                _ => None,                    
            };

            current = if type_.is_none() {
                current + 2
            } else {
                current + 1
            };

            let expr = match tokens[current] {
                token::Token::Operator(token::Operator::Assign) => self.parse_math_expr(&tokens[current+1..])?,
                _ => return Err(ParseError::InvalidToken), // for now we only support assignment
            };

            if type_.is_none() {
                let val = expr.eval().or_else(|e| Err(ParseError::Error(e.to_string())))?;
                type_ = match val {
                    Value::Int(_) => Some(token::Type::Int),
                    Value::Float(_) => Some(token::Type::Float),
                    Value::Bool(_) => Some(token::Type::Bool),
                    Value::String(_) => Some(token::Type::String),
                };
            }

            let type_ = match type_ {
                Some(t) => t,
                _ => return Err(ParseError::Error("Type not found".to_string())), // for now we only support assignment
            };
            
            Ok(Box::new(assignment::VariableAssignment {
                mutable: mutable,
                type_: type_,
                name: identifier,
                expr: expr,
            }))
        }
    }

    fn parse_math_expr(&self, tokens: &[token::Token]) -> Result<Box<dyn ast::ASTNode>, ParseError> {
        let tokens = tokens.to_vec();
        if !tokens.is_empty() && tokens[0] == token::Token::Bracket(token::Bracket::CloseParen)
        && tokens[tokens.len()-1] == token::Token::Bracket(token::Bracket::OpenParen)  {
            return self.parse_math_expr(&tokens[1..tokens.len()-1]);
        }
        let operators = [
            token::Token::Operator(token::Operator::Sub),
            token::Token::Operator(token::Operator::Add),
            token::Token::Operator(token::Operator::Div),
            token::Token::Operator(token::Operator::Mul),
            token::Token::Operator(token::Operator::Mod),
            token::Token::Operator(token::Operator::Pow),
            token::Token::Operator(token::Operator::BitAnd),
            token::Token::Operator(token::Operator::BitOr),
            token::Token::Operator(token::Operator::BitXor),
            token::Token::Operator(token::Operator::BitShiftLeft),
            token::Token::Operator(token::Operator::BitShiftRight),
        ];

        for op in operators.iter() {
            if let Some(pos) = self.find_first_token_skip_brackets(&op, &tokens)? {
                let node = binary_operation::BinaryOperation {
                    left: self.parse_math_expr(&tokens[..pos])?,
                    right: self.parse_math_expr(&tokens[pos + 1..])?,
                    operator: match op {
                        token::Token::Operator(op) => op.clone(),
                        _ => return Err(ParseError::InvalidOperator)
                    },
                };
                return Ok(Box::new(node));
            }
        }
        if tokens.len() == 1 {
            let literal = literal::Literal::from_token(tokens[0].clone())
                .map_err(|e| ParseError::Error(e.to_string()))?;
            return Ok(Box::new(literal));
        }

        Err(ParseError::InvalidMathExpression)
    }


    fn find_matching_bracket(&self, tokens: &[token::Token], loc: usize) -> Result<usize, ParseError> {
        let bracket_pairs = [
            (token::Token::Bracket(token::Bracket::OpenParen), token::Token::Bracket(token::Bracket::CloseParen)),
            (token::Token::Bracket(token::Bracket::OpenBrace), token::Token::Bracket(token::Bracket::CloseBrace)), 
            (token::Token::Bracket(token::Bracket::OpenBracket), token::Token::Bracket(token::Bracket::CloseBracket))
        ];
        let mut count = 1;
        let close_pos = loc;
        let mut open_pos = close_pos;
        match bracket_pairs.iter().find(|(_open, close)| tokens[close_pos] == *close) {
            Some((open, close)) => {
                while count > 0 && open_pos < tokens.len() - 1 {
                    open_pos += 1;
                    if tokens[open_pos] == *close {
                        count += 1;
                    } else if tokens[open_pos] == *open {
                        count -= 1;
                    }
                }
                Ok(open_pos)
            }
            _ => Err(ParseError::NoMatchingBracket),
        }
    }
}


