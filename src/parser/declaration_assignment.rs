use crate::ast::tuple::{Clonable, Tuple};
use crate::lexer::token;
use crate::ast::*;
use super::parser::{Parser, ParseError};
use crate::ast::identifier::Identifier;
use crate::ast::type_node::Type;
use crate::Error;

impl Parser {

    pub fn parse_assignment_or_declaration_expr(&mut self, tokens: &[token::Token]) -> (Result<Box<dyn ast::ASTNode>, Error>, usize) {
        let mut start = 0;
        let mut mutable = false;
        let mut is_decl = false;
        let mut pos;

        // check if it's a declaration or assignment
       if matches!(tokens[0], token::Token::VariableDeclaration(token::VariableDeclaration::Mut)) ||
        matches!(tokens[0], token::Token::VariableDeclaration(token::VariableDeclaration::Let)) {
            start = 1;
            mutable = matches!(tokens[0], token::Token::VariableDeclaration(token::VariableDeclaration::Mut));
            is_decl = true;
        }

        // check if it is a : or := expr
        let colon_pos = match self.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Colon), &tokens) {
            Ok(pos) => pos,
            Err(e) => return (Err(e), 0)
        };
        let assign_pos = match self.find_first_token_skip_brackets(&token::Token::Operator(token::Operator::Assign), &tokens) {
            Ok(pos) => pos,
            Err(e) => return (Err(e), 0)
        };
        let mut colon_first = false;
        if colon_pos.is_some() && assign_pos.is_some() {
            let colon_pos = colon_pos.unwrap();
            let assign_pos = assign_pos.unwrap();
            if colon_pos < assign_pos {
                colon_first = true;
            }
            pos = std::cmp::min(colon_pos, assign_pos);
        } else if colon_pos.is_some() {
            pos = colon_pos.unwrap();
            colon_first = true;
        } else if assign_pos.is_some() {
            pos = assign_pos.unwrap();
        } else {
            return (Err(Error::ParserError(ParseError::InvalidExpression)), 0);
        }

        let identifier_tuple = match self.make_tuple(&tokens[start..pos]) {
            Ok(tuple) => tuple,
            Err(e) => return (Err(e), pos)
        };
        if !self.is_identifier_tuple(identifier_tuple.clone()) {
            return (Err(Error::ParserError(ParseError::AssignmentTupleNotIdentifier)), pos);
        }
        let identifier_tuple: Tuple<Identifier> = match self.parse_tuple(identifier_tuple, |_s, tok| {
            assert_eq!(tok.len(), 1);
            assert!(matches!(tok[0], token::Token::Identifier(_)));
            Ok(Identifier::from_token(tok[0].clone()).unwrap())
        }) {
            Ok(tuple) => tuple,
            Err(e) => return (Err(e), pos)
        };
        
        if colon_first {
            let (types, new_pos) = self.make_left_matching_tuple(&tokens[pos+1..], identifier_tuple.clone_element());
            let types = match types {
                Ok(types) => types,
                Err(e) => return (Err(e), new_pos+pos)
            };

            let types = match self.parse_tuple(types, |s, tok| s.parse_type_expr(tok).0) {
                Ok(types) => types,
                Err(e) => return (Err(e), new_pos+pos)
            };

            pos += new_pos;



            let mut value = None;
            if pos < tokens.len() && matches!(tokens[pos], token::Token::Operator(token::Operator::EqualSign)) {
                let (the_value, new_pos) = match self.parse_expr(&tokens[pos+1..]) {
                    (Ok(value), new_pos) => (value, new_pos),
                    (Err(e), new_pos) => return (Err(e), new_pos+pos)
                };
                value = Some(the_value);
                pos += new_pos + 1;
            }

            if let Some(value) = value {
                let node = assignment::VariableDeclarationAssignment {
                    mutable,
                    type_: Some(match Type::from_tuple(types) {
                        Ok(t) => t,
                        Err(_) => return (Err(Error::ParserError(ParseError::CannotBuildTupleType)), pos)
                    }),
                    name: identifier_tuple.map(&|i| i.element()),
                    expr: value,
                };
                return (Ok(Box::new(node)), pos);
            } else {
                    let node = variable_declaration::VariableDeclaration {
                        mutable,
                        type_: match Type::from_tuple(types) {
                            Ok(t) => t,
                            Err(_) => return (Err(Error::ParserError(ParseError::CannotBuildTupleType)), pos)
                        },
                    name: identifier_tuple.map(&|i| i.element()),
                };
                return (Ok(Box::new(node)), pos);
            }
        } else {
            let (value, new_pos) = match self.parse_expr(&tokens[pos+1..]) {
                (Ok(value), new_pos) => (value, new_pos),
                (Err(e), new_pos) => return (Err(e), new_pos+pos)
            };
            pos += new_pos + 1;

            if is_decl {
                let node = assignment::VariableDeclarationAssignment {
                    mutable,
                    type_: None,
                    name: identifier_tuple.map(&|i| i.element()),
                    expr: value,
                };
                return (Ok(Box::new(node)), pos);
            } else {
                let node = assignment::VariableAssignment {
                    name: identifier_tuple.map(&|i| i.element()),
                    expr: value,
                };
                return (Ok(Box::new(node)), pos);
            }
        }
    }

}