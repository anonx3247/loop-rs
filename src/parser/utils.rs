use crate::lexer::token;
use super::parser::{Parser, ParseError};

impl Parser {
    pub fn find_first_token_skip_brackets(&mut self, token: &token::Token, tokens: &[token::Token]) -> Result<Option<usize>, ParseError> {
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

    pub fn find_matching_bracket(&mut self, tokens: &[token::Token], loc: usize) -> Result<usize, ParseError> {
        let bracket_pairs = [
            (token::Token::Bracket(token::Bracket::OpenParen), token::Token::Bracket(token::Bracket::CloseParen)),
            (token::Token::Bracket(token::Bracket::OpenBrace), token::Token::Bracket(token::Bracket::CloseBrace)),
            (token::Token::Bracket(token::Bracket::OpenBracket), token::Token::Bracket(token::Bracket::CloseBracket))
        ];
        let open_token = &tokens[loc];
        let (open, close) = bracket_pairs.iter().find(|(open, _)| open_token == open)
            .ok_or(ParseError::NoMatchingBracket)?;
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
        Err(ParseError::NoMatchingBracket)
    }

    pub fn find_opening_brace_for(&mut self, tokens: &[token::Token], cond: token::Token) -> Result<usize, ParseError> {
        if tokens.len() <= 1 {
            return Err(ParseError::NoMatchingBraceForKeyword(cond));
        }
        match cond {
            token::Token::Conditional(token::Conditional::Else)
            | token::Token::Loop(token::Loop::Loop) => {
                if tokens[1] != token::Token::Bracket(token::Bracket::OpenBrace) {
                    return Err(ParseError::NoMatchingBraceForKeyword(cond));
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
        Err(ParseError::NoMatchingBraceForKeyword(cond))
    }

    pub fn find_next_non_whitespace_token(&mut self, tokens: &[token::Token]) -> Option<token::Token> {
        for token in tokens {
            if !matches!(token, token::Token::Whitespace(_)) {
                return Some(token.clone());
            }
        }
        None
    }

    pub fn find_expr_possible_boundary(&mut self, tokens: &[token::Token], assign_mode: bool) -> usize {
        let mut cursor = 0;
        while cursor < tokens.len() {
            match tokens[cursor] {
                token::Token::Loop(_)
                | token::Token::TypeDeclaration(_)
                | token::Token::Function(_)
                | token::Token::Module(_)
                | token::Token::Debug
                => {
                    return cursor
                },
                token::Token::Type(_)
                | token::Token::VariableDeclaration(_) => {
                    if !assign_mode {
                        return cursor;
                    } else {
                        cursor += 1;
                    }
                }
                token::Token::Whitespace(token::Whitespace::Newline) => {
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
                            token::Token::Literal(_) | token::Token::Identifier(_) => {
                                return cursor+1;
                            },
                            _ => {cursor += 1;}
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
                }
                _ => {
                    cursor += 1;
                }
            }
        }
        cursor
    }
} 