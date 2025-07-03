use crate::lexer::token;
use super::parser::{Parser, ParseError};
use crate::Error;

impl Parser {
    pub fn find_first_token_skip_brackets(&mut self, token: &token::Token, tokens: &[token::Token]) -> Result<Option<usize>, Error> {
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

    pub fn find_matching_bracket(&mut self, tokens: &[token::Token], loc: usize) -> Result<usize, Error> {
        let bracket_pairs = [
            (token::Token::Bracket(token::Bracket::OpenParen), token::Token::Bracket(token::Bracket::CloseParen)),
            (token::Token::Bracket(token::Bracket::OpenBrace), token::Token::Bracket(token::Bracket::CloseBrace)),
            (token::Token::Bracket(token::Bracket::OpenBracket), token::Token::Bracket(token::Bracket::CloseBracket))
        ];
        let open_token = &tokens[loc];
        let (open, close) = bracket_pairs.iter().find(|(open, _)| open_token == open)
            .ok_or(Error::ParserError(ParseError::NoMatchingBracket))?;
        let mut count = 1;
        let mut i = loc + 1;
        while i < tokens.len() {
            if tokens[i] == *open {
                count += 1;
            } else if tokens[i] == *close {
                count -= 1;
                if count == 0 {
                    return Ok(i);
                }
            }
            i += 1;
        }
        Err(Error::ParserError(ParseError::NoMatchingBracket))
    }

    pub fn find_opening_brace_for(&mut self, tokens: &[token::Token], keyword: token::Token) -> Result<usize, Error> {
        if tokens.len() <= 1 {
            return Err(Error::ParserError(ParseError::NoMatchingBraceForKeyword(keyword)));
        }
        match keyword {
            token::Token::Conditional(token::Conditional::Else)
            | token::Token::Loop(token::Loop::Loop) => {
                if tokens[1] != token::Token::Bracket(token::Bracket::OpenBrace) {
                    return Err(Error::ParserError(ParseError::NoMatchingBraceForKeyword(keyword)));
                } else {
                    return Ok(1)
                }
            },
            _ => {
                let mut brace_open_counter = 1;
                let mut cursor = 1;
                while brace_open_counter > 0 && cursor < tokens.len() {
                    match tokens[cursor] {
                        token::Token::Bracket(token::Bracket::OpenBrace) => brace_open_counter -= 1,
                        token::Token::Conditional(_) 
                        | token::Token::Loop(token::Loop::For) 
                        | token::Token::Loop(token::Loop::Loop) 
                        | token::Token::Loop(token::Loop::While) 
                        | token::Token::Function(token::Function::Fn)
                        | token::Token::TypeDeclaration(token::TypeDeclaration::Component)
                        | token::Token::TypeDeclaration(token::TypeDeclaration::Implement)
                        | token::Token::TypeDeclaration(token::TypeDeclaration::Type) => {
                            brace_open_counter += 1;
                        },
                        _ => {}
                    }
                    cursor += 1;
                }
                if brace_open_counter == 0 {
                    return Ok(cursor-1);
                }
            }
        }
        Err(Error::ParserError(ParseError::NoMatchingBraceForKeyword(keyword)))
    }

    pub fn find_next_non_whitespace_token(&mut self, tokens: &[token::Token]) -> Option<token::Token> {
        for token in tokens {
            if !matches!(token, token::Token::Whitespace(_)) {
                return Some(token.clone());
            }
        }
        None
    }

    pub fn is_type_expr(&self, tokens: &[token::Token]) -> bool {
        if tokens.len() > 0 && matches!(tokens[0], token::Token::Type(_)) {
            return true;
        }
        false
    }

    pub fn is_in_parenthesis(&self, tokens: &[token::Token]) -> bool {
        if tokens.len() > 0 && matches!(tokens[0], token::Token::Bracket(token::Bracket::OpenParen)) && matches!(tokens[tokens.len()-1], token::Token::Bracket(token::Bracket::CloseParen)) {
            return true;
        }
        false
    }

    pub fn is_tuple_expr(&mut self, tokens: &[token::Token]) -> bool {
        if tokens.len() > 0 && matches!(tokens[0], token::Token::Bracket(token::Bracket::OpenParen)) && matches!(tokens[tokens.len()-1], token::Token::Bracket(token::Bracket::CloseParen)) {
            return self.is_tuple_expr(&tokens[1..tokens.len()-1]);
        } else {
            if let Ok(Some(_)) = self.find_first_token_skip_brackets(&token::Token::Punctuation(token::Punctuation::Comma), tokens) {
                return true;
            }
            return false;
        }
    }

    pub fn find_expr_possible_boundary(&mut self, tokens: &[token::Token], assign_mode: bool, loop_mode: bool) -> usize {
        let is_type_expr = self.is_type_expr(tokens);
        let mut cursor = 0;
        while cursor < tokens.len() {
            match tokens[cursor] {
                token::Token::Loop(_) => if !loop_mode {
                    return cursor;
                } else {
                    cursor += 1;
                },
                token::Token::TypeDeclaration(_)
                | token::Token::Function(_)
                | token::Token::Module(_)
                | token::Token::Debug
                => {
                    return cursor
                },
                token::Token::VariableDeclaration(_) => {
                    if !assign_mode {
                        return cursor;
                    } else {
                        cursor += 1;
                    }
                }
                token::Token::Type(_) => {
                    if !is_type_expr && !assign_mode {
                        return cursor;
                    } else if !assign_mode {
                        if cursor + 1 >= tokens.len() {
                            return cursor+1;
                        } else {
                            match tokens[cursor +1] {
                                token::Token::Punctuation(token::Punctuation::Comma) | token::Token::Bracket(_) => {
                                    cursor += 1;
                                },
                                _ => {
                                    return cursor+1;
                                }
                            }
                        }
                    } else {
                        cursor += 1;
                    }
                }
                token::Token::Whitespace(token::Whitespace::Newline) => {
                    if is_type_expr {
                        return cursor;
                    }

                    if cursor + 1 < tokens.len() {
                        if let token::Token::Whitespace(token::Whitespace::Newline) = tokens[cursor+1] {
                            return cursor
                        }
                    }
                    cursor += 1;
                },
                token::Token::Literal(_) | token::Token::Identifier(_) => {
                    match self.find_next_non_whitespace_token(&tokens[cursor+1..]) {
                        Some(tok) => match tok {
                            token::Token::Operator(_) 
                            | token::Token::Punctuation(_)
                            | token::Token::Bracket(_) => {
                                cursor += 1;
                            },
                            _ => {
                                return cursor+1;
                            }
                        },
                        _ => {
                            cursor += 1;
                        }
                    }
                },
                token::Token::Bracket(_) => {
                    cursor = match self.find_matching_bracket(&tokens, cursor) {
                        Ok(new_pos) => new_pos + 1,
                        Err(_) => {
                            return cursor; // If we can't find a matching bracket, we stop here
                        }
                    }
                },
                token::Token::Operator(token::Operator::EqualSign)
                | token::Token::Operator(token::Operator::Assign)
                | token::Token::Operator(token::Operator::PlusAssign)
                | token::Token::Operator(token::Operator::MinusAssign)
                | token::Token::Operator(token::Operator::MulAssign)
                | token::Token::Operator(token::Operator::DivAssign)
                | token::Token::Operator(token::Operator::ModAssign) => {
                    if !assign_mode {
                        return cursor;
                    } else {
                        cursor += 1;
                    }
                },
                _ => {
                    cursor += 1;
                }
            }
        }
        cursor
    }
} 