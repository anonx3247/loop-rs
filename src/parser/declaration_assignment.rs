use crate::lexer::token;
use crate::ast::*;
use super::parser::{Parser, ParseError};
use crate::ast::identifier::Identifier;
use crate::ast::type_node::Type;

impl Parser {

    pub fn parse_assignment_or_declaration_expr(&mut self, tokens: &[token::Token]) -> (Result<Vec<Box<dyn ast::ASTNode>>, ParseError>, usize) {

        let mut start = 0;
        let mut mutable = false;
        let mut is_decl = false;
        let mut nodes: Vec<Box<dyn ASTNode>> = vec![];

       if matches!(tokens[0], token::Token::VariableDeclaration(token::VariableDeclaration::Mut)) ||
        matches!(tokens[0], token::Token::VariableDeclaration(token::VariableDeclaration::Let)) {
            start = 1;
            mutable = matches!(tokens[0], token::Token::VariableDeclaration(token::VariableDeclaration::Mut));
            is_decl = true;
        }

        let colon_pos = match self.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Colon), &tokens) {
            Ok(pos) => pos,
            Err(e) => return (Err(e), 0)
        };
        let assign_pos = match self.find_first_token_skip_brackets(&token::Token::Operator(token::Operator::Assign), &tokens) {
            Ok(pos) => pos,
            Err(e) => return (Err(e), 0)
        };
        let mut colon_first = false;
        let mut min_pos = 0;
        if colon_pos.is_some() && assign_pos.is_some() {
            let colon_pos = colon_pos.unwrap();
            let assign_pos = assign_pos.unwrap();
            if colon_pos < assign_pos {
                colon_first = true;
            }
            min_pos = std::cmp::min(colon_pos, assign_pos);
        } else if colon_pos.is_some() {
            min_pos = colon_pos.unwrap();
        } else if assign_pos.is_some() {
            min_pos = assign_pos.unwrap();
        } else {
            return (Err(ParseError::InvalidExpression), 0);
        }

        let identifier_amount = match self.get_tuple_length(&tokens[..min_pos]) {
            Ok(amount) => amount,
            Err(e) => return (Err(e), min_pos)
        };

        // for now we'll assume that there is can be no tuple dereferencing, e.g. a, b := my_func() or let (a,b), c := my_func()...
        let identifiers = match self.parse_split_on_commas_expr(&tokens[start..min_pos], |s, tok| {
            assert_eq!(tok.len(), 1);
            assert!(matches!(tok[0], token::Token::Identifier(_)));
            (Ok(Box::new(Identifier::from_token(tok[0].clone()).unwrap())), 1)
        }) {
            (Ok(identifiers), pos) => identifiers,
            (Err(e), pos) => return (Err(e), min_pos)
        };
    
        let mut pos = min_pos;

        if colon_first {
            let (types, new_pos) = self.parse_one_or_n_exprs(&tokens[min_pos+1..], identifier_amount, |s, tok| {
                assert_eq!(tok.len(), 1);
                match tok[0].clone() {
                    token::Token::Type(type_) => (Ok(Box::new(Type::from_token_type(type_))), 1),
                    _ => (Err(ParseError::InvalidToken), 0)
                }
            });
            let mut types = match types {
                Ok(types) => types,
                Err(e) => return (Err(e), new_pos)
            };

            if types.len() == 1 && types.len() != identifier_amount {
                for _ in 1..identifier_amount {
                    types.push(types[0].clone())
                }
            }
            pos = new_pos;

            if matches!(tokens[pos], token::Token::Operator(token::Operator::EqualSign)) {
                let (values, new_pos) = self.parse_one_or_n_exprs(&tokens[pos+1..], 
                    identifier_amount, 
                    |s, tok| s.parse_expr(tok),
                );
                let mut values = match values {
                    Ok(values) => values,
                    Err(e) => return (Err(e), new_pos)
                };

                if values.len() == 1 && values.len() != identifier_amount {
                    for _ in 1..identifier_amount {
                        values.push(values[0].clone_to_node())
                    }
                }

                
                for (i, ((identifier, type_), value)) in identifiers.iter().zip(types.iter()).zip(values.iter()).enumerate() {
                    let node = assignment::VariableAssignment {
                        is_also_decl: is_decl,
                        mutable,
                        type_: Some(*type_.clone()),
                        name: identifier.to_string(),
                        expr: (*value).clone_to_node(),
                    };
                    nodes.push(Box::new(node));
                }
                pos = new_pos;
                return (Ok(nodes), pos);
            } else {
                for (i, (identifier, type_)) in identifiers.iter().zip(types.iter()).enumerate() {
                    let node = variable_declaration::VariableDeclaration {
                        mutable,
                        type_: *type_.clone(),
                        name: identifier.to_string(),
                    };
                    nodes.push(Box::new(node));
                }
                return (Ok(nodes), pos);
            }
        } else {
            let (values, new_pos) = self.parse_one_or_n_exprs(
                &tokens[pos+1..], identifier_amount, 
                |s, tok| s.parse_expr(tok),
            );
            pos += new_pos;
            let mut values = match values {
                Ok(values) => values,
                Err(e) => return (Err(e), new_pos)
            };

            if values.len() == 1 && values.len() != identifier_amount {
                for _ in 1..identifier_amount {
                    values.push(values[0].clone_to_node())
                }
            }

            for (i, (identifier, value)) in identifiers.iter().zip(values.iter()).enumerate() {
                let node = assignment::VariableAssignment {
                    is_also_decl: is_decl,
                    mutable,
                    type_: None,
                    name: identifier.element(),
                    expr: (*value).clone_to_node(),
                };
                nodes.push(Box::new(node));
            }
            return (Ok(nodes), pos);
        }
    }

}