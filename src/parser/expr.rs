use crate::{ast::tuple::TupleASTNode, lexer::token};
use crate::ast::*;
use super::parser::Parser;
use crate::Error;

impl Parser {
    pub fn parse_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, Error>, usize) {

        if tokens.is_empty() {
            return (Ok(Box::new(EmptyASTNode::new())), 0);
        }

        if !tokens.is_empty() && tokens[0] == token::Token::Bracket(token::Bracket::OpenParen) {
            if let Ok(pos) = self.find_matching_bracket(&tokens, 0) {
                if pos == tokens.len() - 1 {
                    if self.is_tuple_expr(&tokens[1..tokens.len()-1]) {
                        let tuple = match self.make_tuple(&tokens[1..tokens.len()-1]) {
                            Ok(t) => t,
                            Err(e) => return (Err(e), tokens.len())
                        };

                        let tuple = match self.parse_tuple(tuple, |s, tok| s.parse_expr(tok).0) {
                            Ok(t) => t,
                            Err(e) => return (Err(e), tokens.len())
                        };

                        return (Ok(TupleASTNode::from_tuple(tuple)), tokens.len());
                    } else {
                        return (self.parse_expr(&tokens[1..tokens.len()-1]).0, tokens.len());
                    }
                }
            }
        }

        if tokens.len() == 1 {
            match tokens[0] {
                token::Token::Identifier(_) => {
                    let identifier = identifier::Identifier::from_token(tokens[0].clone());
                    return (match identifier {
                        Ok(identifier) => Ok(Box::new(identifier)),
                        Err(e) => Err(e),
                    }, 1);
                }
                _ => {
                    let literal = literal::Literal::from_token(tokens[0].clone());
                    return (match literal {
                        Ok(literal) => Ok(Box::new(literal)),
                        Err(e) => Err(e),
                    }, 1);
                }
            }
        }

        let mut tokens = tokens;
        let mut offset = 0;
        while matches!(tokens[0], token::Token::Whitespace(_)) {
            offset += 1;
            tokens = &tokens[1..];
        }

        if let token::Token::Identifier(_) = tokens[0] {
            let pos = 1;
            if pos < tokens.len() && matches!(tokens[pos], token::Token::Bracket(token::Bracket::OpenParen)) {
                return self.parse_fn_call(&tokens);
            }
        }

        if let token::Token::Function(token::Function::Fn) = tokens[0] {
            let max_expr_length= match self.find_expr_possible_boundary(&tokens, true, true, true) {
                Ok(length) => length,
                Err(e) => return (Err(e), offset)
            };
            let tokens = &tokens[..max_expr_length];
            let (node, pos) = self.parse_fn_declaration(&tokens);
            return (node, pos + offset);
        }

        let loop_tokens = [
            token::Token::Loop(token::Loop::While),
            token::Token::Loop(token::Loop::For),
            token::Token::Loop(token::Loop::Loop),
            token::Token::Loop(token::Loop::Break),
            token::Token::Loop(token::Loop::Continue)
        ];

        let decl_tokens = [
            token::Token::Operator(token::Operator::Assign),
            token::Token::Operator(token::Operator::EqualSign),
            token::Token::VariableDeclaration(token::VariableDeclaration::Mut),
            token::Token::VariableDeclaration(token::VariableDeclaration::Let),
            token::Token::Punctuation(token::Punctuation::Colon),
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

        let conditional_tokens = [
            token::Token::Conditional(token::Conditional::If),
            token::Token::Conditional(token::Conditional::Else),
            token::Token::Conditional(token::Conditional::Elif),
        ];

        let max_expr_length= match self.find_expr_possible_boundary(&tokens, true, true, false) {
            Ok(length) => length,
            Err(e) => return (Err(e), offset)
        };
        let tokens = &tokens[..max_expr_length];

        for op in loop_tokens.iter() {
            if let Ok(Some(_)) = self.find_first_token_skip_brackets(&op, &tokens) {
                let (node, pos) = self.parse_loop_expr(&tokens);
                return (node, pos + offset);
            }
        }

        let max_expr_length= match self.find_expr_possible_boundary(&tokens, true, false, false) {
            Ok(length) => length,
            Err(e) => return (Err(e), offset)
        };
        let tokens = &tokens[..max_expr_length];
        

        for op in decl_tokens.iter() {
            if let Ok(Some(_)) = self.find_first_token_skip_brackets(&op, &tokens) {
                let (node, i) = self.parse_assignment_or_declaration_expr(&tokens);
                let node = match node  {
                    Ok(n) => n,
                    Err(e) => return (Err(e), i + offset),
                };
                return (Ok(node), i + offset);
            }
        }

        let max_expr_length= match self.find_expr_possible_boundary(&tokens, false, false, false) {
            Ok(length) => length,
            Err(e) => return (Err(e), offset)
        };
        let tokens = &tokens[..max_expr_length];

        if self.is_tuple_expr(tokens) {
            let tuple = match self.make_tuple(tokens) {
                Ok(t) => t,
                Err(e) => return (Err(e), max_expr_length)
            };

            let tuple = match self.parse_tuple(tuple, |s, tok| s.parse_expr(tok).0) {
                Ok(t) => t,
                Err(e) => return (Err(e), max_expr_length)
            };

            return (Ok(TupleASTNode::from_tuple(tuple)), max_expr_length)
        }


        for op in conditional_tokens.iter() {
            if let Ok(Some(_)) = self.find_first_token_skip_brackets(&op, &tokens) {
                let (node, pos) = self.parse_conditional_expr(&tokens);
                let node = match node {
                    Ok(node) =>  node.clone_to_node(),
                    Err(e) => return (Err(e), pos),
                };
                return (Ok(node), pos + offset);
            }
        }

        for op in bool_tokens.iter() {
            if let Ok(Some(_)) = self.find_first_token_skip_brackets(&op, &tokens) {
                let (n, i) = self.parse_bool_expr(&tokens);
                return (n, i + offset);
            }
        }
        self.parse_math_expr(tokens)
    }
} 