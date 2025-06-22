use crate::lexer::token;
use crate::ast::*;

pub struct Parser {
    tokens: Vec<token::Token>,
}

#[derive(Debug)]
pub enum ParseError {
    EmptyTokens,
    InvalidExpression,
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
        println!("parsing tokens: {:?}", tokens);
        let mut result = RootASTNode::new();
        let mut pos = 0;
        if tokens.len() == 0 {
            return Err(ParseError::EmptyTokens);
        }
        let mut it = 0;
        let mut tokens_to_parse = tokens.to_vec();
        while pos < tokens.len() && it < 100 {
            let (node, new_pos) = self.parse_expr(&tokens_to_parse[..]);
            match node {
                Ok(node) => result.push(node),
                Err(e) => return Err(e),
            };
            pos = new_pos;
            tokens_to_parse = tokens_to_parse[pos..].to_vec();
            it += 1;
        }
        Ok(Box::new(result))
    }

    /*
    fn find_first_token(&self, token: &token::Token, tokens: &[token::Token]) -> Option<usize> {
        tokens.iter().position(|t| t == token)
    }
     */

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

    fn parse_expr(&self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, ParseError>, usize) {
        let assign_tokens = [
            token::Token::Operator(token::Operator::Assign),
            token::Token::Operator(token::Operator::EqualSign),
            token::Token::VariableDeclaration(token::VariableDeclaration::Mut)
        ];

        let bool_tokens = [
            token::Token::Operator(token::Operator::And),
            token::Token::Operator(token::Operator::Or),
            token::Token::Operator(token::Operator::Not),
            token::Token::Operator(token::Operator::Eq),
            token::Token::Operator(token::Operator::Neq),
            token::Token::Operator(token::Operator::Gt),
            token::Token::Operator(token::Operator::Gte),
            token::Token::Operator(token::Operator::Lt),
            token::Token::Operator(token::Operator::Lte),
        ];

        for op in assign_tokens.iter() {
            if let Ok(Some(pos)) = self.find_first_token_skip_brackets(&op, &tokens) {
                return self.parse_assignment_expr(&tokens);
            }
        }

        for op in bool_tokens.iter() {
            if let Ok(Some(pos)) = self.find_first_token_skip_brackets(&op, &tokens) {
                return self.parse_bool_expr(&tokens);
            }
        }
        
        self.parse_math_expr(tokens)
    }

    fn parse_binary_operator_expr(&self, tokens: &[token::Token], 
        operators: &[token::Token], 
        left_parser: Option<fn(&[token::Token]) -> (Result<Box<dyn ast::ASTNode>, ParseError>, usize)>, 
        right_parser: Option<fn(&[token::Token]) -> (Result<Box<dyn ast::ASTNode>, ParseError>, usize)>
        ) -> (Result<Box<dyn ast::ASTNode>, ParseError>, usize)
        {
        let tokens = tokens.to_vec();
        if !tokens.is_empty() && tokens[0] == token::Token::Bracket(token::Bracket::CloseParen)
        && tokens[tokens.len()-1] == token::Token::Bracket(token::Bracket::OpenParen)  {
            return (self.parse_binary_operator_expr(&tokens[1..tokens.len()-1], operators, left_parser, right_parser).0, tokens.len());
        }
        for op in operators.iter() {
            if let Ok(Some(pos)) = self.find_first_token_skip_brackets(&op, &tokens) {
                let (left, left_pos) = match left_parser {
                    Some(parser) => parser(&tokens[..pos]),
                    _ => self.parse_binary_operator_expr(&tokens[..pos], operators, left_parser, right_parser),
                };
                let (right, right_pos) = match right_parser {
                    Some(parser) => parser(&tokens[pos + 1..]),
                    _ => self.parse_binary_operator_expr(&tokens[pos + 1..], operators, left_parser, right_parser),
                };
                let node = binary_operation::BinaryOperation {
                    left: match left {
                        Ok(node) => node,
                        Err(e) => return (Err(e), left_pos),
                    },
                    right: match right {
                        Ok(node) => node,
                        Err(e) => return (Err(e), right_pos + pos + 1),
                    },
                    operator: match op {
                        token::Token::Operator(op) => op.clone(),
                        _ => return (Err(ParseError::InvalidOperator), pos)
                    },
                };
                return (Ok(Box::new(node)), right_pos + pos + 1);
            }
        }
        if tokens.len() == 1 {
            match tokens[0] {
                token::Token::Identifier(_) => {
                    let identifier = identifier::Identifier::from_token(tokens[0].clone())
                        .map_err(|e| ParseError::Error(e.to_string()));
                    return (match identifier {
                        Ok(identifier) => Ok(Box::new(identifier)),
                        Err(e) => Err(e),
                    }, 1);
                }
                _ => {
                    let literal = literal::Literal::from_token(tokens[0].clone())
                        .map_err(|e| ParseError::Error(e.to_string()));
                    return (match literal {
                        Ok(literal) => Ok(Box::new(literal)),
                        Err(e) => Err(e),
                    }, 1);
                }
            }
        }
        (Err(ParseError::InvalidExpression), 0)
    }

    fn parse_assignment_expr(&self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, ParseError>, usize) {
        let operators = [
            token::Token::Operator(token::Operator::Assign),
            token::Token::Operator(token::Operator::EqualSign)
        ];

        for op in operators.iter() {
            if let Ok(Some(pos)) = self.find_first_token_skip_brackets(&op, &tokens) {
                let (left, left_pos) = self.parse_expr(&tokens[..pos]);
                let left = match left {
                    Ok(node) => node,
                    Err(e) => return (Err(e), left_pos),
                };
                if op == &token::Token::Operator(token::Operator::Assign) {
                    // make sure the previous token is an identifier and the one before it is either a 'mut' or anything else

                    // [mut] identifier = expr
                    if let Some(token::Token::Identifier(identifier)) = tokens.get(pos + 1) {
                        if let Some(token::Token::VariableDeclaration(token::VariableDeclaration::Mut)) = tokens.get(pos + 2) {
                            let node = assignment::VariableAssignment {
                                mutable: true,
                                type_: None,
                                name: identifier.clone(),
                                expr: left,
                            };
                            return (Ok(Box::new(node)), pos + 3);
                        } else {
                            let node = assignment::VariableAssignment {
                                mutable: false,
                                type_: None,
                                name: identifier.clone(),
                                expr: left,
                            };
                            return (Ok(Box::new(node)), pos + 2);
                        }
                    }
                } else {
                    // [mut] identifier: type = expr
                    if let Some(token::Token::Type(type_)) = tokens.get(pos + 1) {
                        if let Some(token::Token::Punctuation(token::Punctuation::Colon)) = tokens.get(pos + 2) {
                            if let Some(token::Token::Identifier(identifier)) = tokens.get(pos + 3) {
                                if let Some(token::Token::VariableDeclaration(token::VariableDeclaration::Mut)) = tokens.get(pos + 4) {
                                    let node = assignment::VariableAssignment {
                                        mutable: true,
                                        type_: Some(type_.clone()),
                                        name: identifier.clone(),
                                        expr: left,
                                    };
                                    return (Ok(Box::new(node)), pos + 5);
                                } else {
                                    let node = assignment::VariableAssignment {
                                        mutable: false,
                                        type_: Some(type_.clone()),
                                        name: identifier.clone(),
                                        expr: left,
                                    };
                                    return (Ok(Box::new(node)), pos + 3);
                                }
                            }
                        } else {
                            return (Err(ParseError::InvalidToken), pos + 2);
                        }
                    } else {
                        return (Err(ParseError::InvalidToken), pos + 1);
                    }
                }
            }
        }

        (Err(ParseError::InvalidExpression), 0)
        
    }

    fn parse_math_expr(&self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, ParseError>, usize) {
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

        self.parse_binary_operator_expr(tokens, &operators, None, None)
    }

    fn parse_bool_expr(&self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, ParseError>, usize) {
        let operators = [
            token::Token::Operator(token::Operator::And),
            token::Token::Operator(token::Operator::Or),
            token::Token::Operator(token::Operator::Not),
            token::Token::Operator(token::Operator::Eq),
            token::Token::Operator(token::Operator::Neq),
            token::Token::Operator(token::Operator::Gt),
            token::Token::Operator(token::Operator::Gte),
            token::Token::Operator(token::Operator::Lt),
            token::Token::Operator(token::Operator::Lte),
        ];
        self.parse_binary_operator_expr(tokens, &operators, None, None)
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


